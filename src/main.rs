extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::fs::File;
use std::io::prelude::*;

fn regreadvalue_string (regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);

    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);

    match subkey {
        Ok(subkey) => {
            let thevalue: String = subkey.get_value(regvalue).unwrap_or_default();
            let iniline: String = format!("{}\\{}={}\n", regpath, regvalue, thevalue);
            let binaryiniline = iniline.as_bytes();
            inifile.write_all(binaryiniline).expect("could not write line");
            },
        Err(_error) => {
            let iniline: String = format!("{}\\{}={}\n", regpath, regvalue, "");
            let binaryiniline = iniline.as_bytes();
            inifile.write_all(binaryiniline).expect("could not write line");
            },
        };
    }
fn regreadvalue_int (regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    
        let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
    
        match subkey {
            Ok(subkey) => {
                let thevalue: u32 = subkey.get_value(regvalue).unwrap_or_default();
                let iniline: String = format!("{}\\{}={}\n", regpath, regvalue, thevalue);
                let binaryiniline = iniline.as_bytes();
                inifile.write_all(binaryiniline).expect("could not write line");
                },
            Err(_error) => {
                let iniline: String = format!("{}\\{}={}\n", regpath, regvalue, "");
                let binaryiniline = iniline.as_bytes();
                inifile.write_all(binaryiniline).expect("could not write line");
                },
            };
        }


fn main() -> std::io::Result<()> {

    let mut inifile = File::create("output.ini")?;
    inifile.write_all(b"[FileInfo]\n")?;
    inifile.write_all(b"Version=1\n")?;
    inifile.write_all(b"[Machine]\n")?;
    regreadvalue_string(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName", &inifile);
    regreadvalue_string(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentVersion", &inifile);
    regreadvalue_string(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionID", &inifile);
    regreadvalue_string(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ReleaseId", &inifile);
    regreadvalue_string(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid", &inifile);
    regreadvalue_string(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "ComputerName", &inifile);
    regreadvalue_string(r#"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"#, "Domain", &inifile);
    regreadvalue_string(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "ComputerHardwareId", &inifile);
    regreadvalue_string(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemManufacturer", &inifile);
    regreadvalue_string(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemProductName", &inifile);
    regreadvalue_string(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSVersion", &inifile);
    regreadvalue_string(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSReleaseDate", &inifile);
    regreadvalue_int(r#"SYSTEM\HardwareConfig\Current"#, "EnclosureType", &inifile);
    
    Ok(())
}