#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]

extern crate encoding;

use std::ffi::{ CString, CStr };
use self::encoding::{ Encoding, EncoderTrap, DecoderTrap };
use self::encoding::all::GB18030;

macro_rules! gb18030 {
	( $x: expr ) => (
        CString::new(GB18030.encode($x, EncoderTrap::Ignore).unwrap()).unwrap().into_raw()
    )
}

macro_rules! utf8 {
    ( $x: expr ) => (
        GB18030.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore).unwrap()
    )
}

mod types;
use types::{ Event, Request, LogPriority, Chat, Identity, Error };


// APIs


#[link(name = "CQP")]
extern "stdcall" {
    fn CQ_addLog                (ac: i32, priority: i32, t: *const i8, msg: *const i8) -> i32;
    fn CQ_getLoginQQ            (ac: i32) -> i64;
    fn CQ_getLoginNick          (ac: i32) -> *const i8;
    fn CQ_getAppDirectory       (ac: i32) -> *const i8;
    fn CQ_getCookies            (ac: i32) -> *const i8;
    fn CQ_getCsrfToken          (ac: i32) -> i32;
    fn CQ_sendLike              (ac: i32, qq: i64) -> i32;
    fn CQ_sendPrivateMsg        (ac: i32, qq: i64, msg: *const i8) -> i32;
    fn CQ_sendGroupMsg          (ac: i32, group: i64, msg: *const i8) -> i32;
    fn CQ_sendDiscussMsg        (ac: i32, discussion: i64, msg: *const i8) -> i32;
    fn CQ_setGroupBan           (ac: i32, group: i64, qq: i64, duration: i64) -> i32;
    fn CQ_setGroupAnonymousBan  (ac: i32, group: i64, name: *const i8, duration: i64) -> i32;
    fn CQ_setGroupWholeBan      (ac: i32, group: i64, enabled: i32) -> i32;
    fn CQ_setGroupAnonymous     (ac: i32, group: i64, enabled: i32) -> i32;
    fn CQ_setGroupKick          (ac: i32, group: i64, qq: i64, refuse_rejoin: i32) -> i32;
    fn CQ_setGroupAdmin         (ac: i32, group: i64, qq: i64, enabled: i32) -> i32;
    fn CQ_setGroupSpecialTitle  (ac: i32, group: i64, qq: i64, title: *const i8, duration: i64) -> i32;
    fn CQ_setGroupCard          (ac: i32, group: i64, qq: i64, card: *const i8) -> i32;
    fn CQ_setGroupLeave         (ac: i32, group: i64, dismiss: i32) -> i32;
    fn CQ_setDiscussLeave       (ac: i32, discussion: i64) -> i32;
    fn CQ_setFriendAddRequest   (ac: i32, response: *const i8, response_type: i32, comment: *const i8) -> i32;
    fn CQ_setGroupAddRequestV2  (ac: i32, response: *const i8, request_type: i32, response_type: i32, reason: *const i8) -> i32;
    fn CQ_getGroupMemberInfoV2  (ac: i32, group: i64, qq: i64, nocache: i32) -> *const i8;
    fn CQ_getStrangerInfo       (ac: i32, qq: i64, nocache: i32) -> *const i8;
    fn CQ_setFatal              (ac: i32, err: *const i8) -> i32;
}

#[derive(Debug)]
pub struct App {
    pub authcode: i32
}

impl App {

    pub fn log(&self, priority: LogPriority,/* category: &str,*/ content: &str) -> i32 {
        let category = gb18030!("lurebot"/*category*/);
        let content = gb18030!(content);
        unsafe {
            CQ_addLog(self.authcode, priority as i32, category, content)
        }
    }

    pub fn error(&self, message: &str) -> i32 {
        self.log(LogPriority::Error, message)
    }

    pub fn fatal(&self, err: &str) -> i32 {
        let err = gb18030!(err);
        unsafe {
            CQ_setFatal(self.authcode, err)
        }
    }

    pub fn qq(&self) -> i64 {
        unsafe {
            CQ_getLoginQQ(self.authcode)
        }
    }

    pub fn nickname(&self) -> String {
        unsafe {
            utf8!(CQ_getLoginNick(self.authcode))
        }
    }

    pub fn app_directory(&self) -> String {
        unsafe {
            utf8!(CQ_getAppDirectory(self.authcode))
        }
    }

    pub fn cookies(&self) -> String {
        unsafe {
            utf8!(CQ_getCookies(self.authcode))
        }
    }

    pub fn csrf_token(&self) -> i32 {
        unsafe {
            CQ_getCsrfToken(self.authcode)
        }
    }

    pub fn send_like(&self, ident: Identity) -> i32 {
        if let Identity::Specific(qq) = ident {
            unsafe {
                CQ_sendLike(self.authcode, qq)
            }
        } else {
            self.error("send_like#ident");
            Error::ArgumentError as i32
        }
    }

    pub fn send_message(&self, chat: Chat, message: &str) -> i32 {
        let message = gb18030!(message);
        unsafe {
            match chat {
                Chat::Private(qq) => CQ_sendPrivateMsg(self.authcode, qq, message),
                Chat::Group(group) => CQ_sendGroupMsg(self.authcode, group, message),
                Chat::Discussion(discussion) => CQ_sendDiscussMsg(self.authcode, discussion, message),
            }
        }
    }

    pub fn group_kick(&self, chat: Chat, ident: Identity, refuse_rejoin: bool) -> i32 {
        if let Chat::Group(group) = chat {
            if let Identity::Specific(qq) = ident {
                unsafe {
                    CQ_setGroupKick(self.authcode, group, qq, refuse_rejoin as i32)
                }
            } else {
                self.error("group_kick#ident");
                Error::ArgumentError as i32
            }
        } else {
            self.error("group_kick#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn group_ban(&self, chat: Chat, ident: Identity, duration: i64) -> i32 {
        if let Chat::Group(group) = chat {
            match ident {
                Identity::Specific(qq) => unsafe {
                    CQ_setGroupBan(self.authcode, group, qq, duration)
                },
                Identity::Anonymous(name) => unsafe {
                    let name = gb18030!(&name);
                    CQ_setGroupAnonymousBan(self.authcode, group, name, duration)
                },
                Identity::Whole => unsafe {
                    CQ_setGroupWholeBan(self.authcode, group, (duration > 0) as i32)
                },
            }
        } else {
            self.error("group_ban#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn group_set_anonymous(&self, chat: Chat, enabled: bool) -> i32 {
        if let Chat::Group(group) = chat {
            unsafe {
                CQ_setGroupAnonymous(self.authcode, group, enabled as i32)
            }
        } else {
            self.error("group_set_anonymous#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn group_set_admin(&self, chat: Chat, ident: Identity, enabled: bool) -> i32 {
        if let Chat::Group(group) = chat {
            if let Identity::Specific(qq) = ident {
                unsafe {
                    CQ_setGroupAdmin(self.authcode, group, qq, enabled as i32)
                }
            } else {
                self.error("group_set_admin#ident");
                Error::ArgumentError as i32
            }
        } else {
            self.error("group_set_admin#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn group_set_special_title(&self, chat: Chat, ident: Identity, title: &str, duration: i64) -> i32 {
        if let Chat::Group(group) = chat {
            if let Identity::Specific(qq) = ident {
                let title = gb18030!(title);
                unsafe {
                    CQ_setGroupSpecialTitle(self.authcode, group, qq, title, duration)
                }
            } else {
                self.error("group_set_special_title#ident");
                Error::ArgumentError as i32
            }
        } else {
            self.error("group_set_special_title#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn group_set_card(&self, chat: Chat, ident: Identity, card: &str) -> i32 {
        if let Chat::Group(group) = chat {
            if let Identity::Specific(qq) = ident {
                let card = gb18030!(card);
                unsafe {
                    CQ_setGroupCard(self.authcode, group, qq, card)
                }
            } else {
                self.error("group_set_card#ident");
                Error::ArgumentError as i32
            }
        } else {
            self.error("group_set_card#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn leave_group(&self, chat: Chat, dismiss: bool) -> i32 {
        if let Chat::Group(group) = chat {
            unsafe {
                CQ_setGroupLeave(self.authcode, group, dismiss as i32)
            }
        } else {
            self.error("leave_group#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn leave_discussion(&self, chat: Chat) -> i32 {
        if let Chat::Discussion(discussion) = chat {
            unsafe {
                CQ_setDiscussLeave(self.authcode, discussion)
            }
        } else {
            self.error("leave_discussion#chat");
            Error::ArgumentError as i32
        }
    }

    pub fn group_member_info(&self, chat: Chat, ident: Identity, nocache: bool) -> String {
        if let Chat::Group(group) = chat {
            if let Identity::Specific(qq) = ident {
                unsafe {
                    utf8!(CQ_getGroupMemberInfoV2(self.authcode, group, qq, nocache as i32))
                }
            } else {
                self.error("group_member_info#ident");
                String::from("---ERROR---")
            }
        } else {
            self.error("group_member_info#chat");
            String::from("---ERROR---")
        }
    }

    pub fn stranger_info(&self, ident: Identity, nocache: bool) -> String {
        if let Identity::Specific(qq) = ident {
            unsafe {
                utf8!(CQ_getStrangerInfo(self.authcode, qq, nocache as i32))
            }
        } else {
            self.error("stranger_info#ident");
            String::from("---ERROR---")
        }
    }
}

static mut CQPAPP: App = App { authcode: 0 };


// Events

mod adapter;

#[export_name="AppInfo"]
pub extern "stdcall" fn AppInfo() -> *const i8 {
    gb18030!("9,me.icefla.lurebot_adapter")
}

#[export_name="Initialize"]
pub extern "stdcall" fn Initialize(ac: i32) -> i32 {
    unsafe {
        CQPAPP.authcode = ac;
    }
    adapter::initialize()
}

#[export_name="Startup"]
pub extern "stdcall" fn onStartup() -> i32 {
    adapter::startup()
}

#[export_name="Exit"]
pub extern "stdcall" fn onExit() -> i32 {
    adapter::exit()
}

#[export_name="Enable"]
pub extern "stdcall" fn onEnable() -> i32 {
    adapter::enable()
}

#[export_name="Disable"]
pub extern "stdcall" fn onDisable() -> i32 {
    adapter::disable()
}

#[export_name="PrivateMessage"]
pub extern "stdcall" fn onPrivateMessage(_subtype: i32, _id: i32, qq: i64, msg: *const i8, _font: i32) -> i32 {
    let chat = Chat::Private(qq);
    let ident = Identity::Specific(qq);
    adapter::message(chat, ident, unsafe{ &utf8!(msg) })
}
