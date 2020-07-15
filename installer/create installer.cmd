del *.msi
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc
candle installer.wxs -arch x64
light installer.wixobj -dcl:high -ext WixUIExtension.dll -ext WixUtilExtension.dll -out jikwaa-agent-x64.msi
del installer.wixobj
del *.wixpdb
candle installer.wxs -arch x86
light installer.wixobj -dcl:high -ext WixUIExtension.dll -ext WixUtilExtension.dll -out jikwaa-agent-x86.msi
del installer.wixobj
del *.wixpdb