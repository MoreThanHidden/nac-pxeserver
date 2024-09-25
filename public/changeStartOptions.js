var objShell = WSH.CreateObject("Wscript.Shell");
var pattern = /MULTI\(\d+\)DISK\(\d+\)RDISK\(\d+\)PARTITION\(\d+\)/gi;
var startOptions = objShell.RegRead("HKLM\\System\\CurrentControlSet\\Control\\SystemStartOptions");

startOptions = startOptions.replace(pattern, "ramdisk(0)");
objShell.RegWrite("HKLM\\System\\CurrentControlSet\\Control\\SystemStartOptions", startOptions);

var systemdrive = objShell.ExpandEnvironmentStrings("%SYSTEMDRIVE%")
var objFSO = WSH.CreateObject("Scripting.FileSystemObject")
var smspath = systemdrive.concat("\\sms\\data\\")

objFSO.CreateFolder(smspath);
objFSO.CopyFile("TsmBootstrap.ini",smspath);
objFSO.CopyFile("Variables.dat",smspath);