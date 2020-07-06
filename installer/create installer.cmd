candle installer.wxs
light installer.wixobj -dcl:high -ext WixUIExtension.dll -ext WixUtilExtension.dll
del installer.wixobj
del installer.wixpdb