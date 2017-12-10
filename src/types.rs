
#[derive(Debug)]
pub enum Event {
    Ignore,
    Block,
}

#[derive(Debug)]
pub enum Request {
    Allow,
    Deny,
}

#[derive(Debug)]
pub enum LogPriority {
    Debug = 0,
    Info = 10,
    InfoSuccess = 11,
    InfoReceive = 12,
    InfoSend = 13,
    Warning = 20,
    Error = 30,
    Fatal = 40,
}

#[derive(Debug)]
pub enum Chat {
    Private(i64),
    Group(i64),
    Discussion(i64),
}

#[derive(Debug)]
pub enum Identity {
    Specific(i64),
    Anonymous(String),
    Whole,
}

// return code
// https://d.cqp.me/Pro/%E5%BC%80%E5%8F%91/Error
#[derive(Debug)]
pub enum Error {
    ArgumentError = -20,
}
