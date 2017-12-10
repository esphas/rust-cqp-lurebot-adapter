
set PACKAGE_NAME=me.icefla.lurebot_adapter

cargo build --target=i686-pc-windows-msvc
mkdir dist
copy %~dp0\target\i686-pc-windows-msvc\debug\cqp_lurebot_adapter.dll %~dp0\dist\%PACKAGE_NAME%.dll
copy %~dp0\cqp_lurebot_adapter.json %~dp0\dist\%PACKAGE_NAME%.json
