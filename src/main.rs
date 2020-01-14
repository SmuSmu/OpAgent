extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::fs::File;
use std::io::prelude::*;

fn display_reg_value(rv: &winreg::RegValue) -> String {
    use winreg::enums::RegType::*;
    use winreg::types::FromRegValue;
    match rv.vtype {
        REG_SZ | REG_EXPAND_SZ | REG_MULTI_SZ => {
            String::from_reg_value(rv).unwrap_or_default()
        }
        REG_DWORD => u32::from_reg_value(rv).unwrap_or_default().to_string(),
        REG_QWORD => u64::from_reg_value(rv).unwrap_or_default().to_string(),
        _ => panic!("can only process reg value of type string, u32 or u64"),
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    
        #[allow(dead_code)]
    fn regreadvalue_intbig (regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
            let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        
            let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
        
            match subkey {
                Ok(subkey) => {
                    let thevalue: u64 = subkey.get_value(regvalue).unwrap_or_default();
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

            #[allow(dead_code)]
    fn regreadvalue_gen (regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    
        let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
    
        match subkey {
            Ok(subkey) => {
                use winreg::enums::RegType::*;
                match subkey.get_raw_value(regvalue) {
                    Ok(raw_value) => {
                        match raw_value.vtype {
                            REG_NONE => println!("None"),
                            REG_SZ => {regreadvalue_string(&regpath, &regvalue, &inifile);},
                            REG_EXPAND_SZ => {regreadvalue_string(&regpath, &regvalue, &inifile);},
                            REG_BINARY => println!("blob"),
                            REG_DWORD => {regreadvalue_int(&regpath, &regvalue, &inifile);},
                            REG_DWORD_BIG_ENDIAN => println!("64-bit number"),
                            REG_LINK => println!("null-terminated path"),
                            REG_MULTI_SZ => println!("null-terminated, null-separated, list of strings"),
                            REG_RESOURCE_LIST => println!("list of resources"),
                            REG_FULL_RESOURCE_DESCRIPTOR => println!("resource"),
                            REG_RESOURCE_REQUIREMENTS_LIST => println!("dependencies"),
                            REG_QWORD => {regreadvalue_intbig(&regpath, &regvalue, &inifile);},
                        }
                    }
                    Err(_) => {
                        let iniline: String = format!("{}\\{}={}\n", regpath, regvalue, "");
                        let binaryiniline = iniline.as_bytes();
                        inifile.write_all(binaryiniline).expect("could not write line");
                        },
                    }

                },
            Err(_error) => {
                let iniline: String = format!("{}\\{}={}\n", regpath, regvalue, "");
                let binaryiniline = iniline.as_bytes();
                inifile.write_all(binaryiniline).expect("could not write line");
                },
            };
        }

fn regreadvalue(regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
    let thevalue = match subkey {
        Ok(subkey) => {
            let v = subkey.get_raw_value(regvalue).unwrap();
            display_reg_value(&v)
        }
        Err(_) => "".to_string(),
    };
    let iniline = format!("{}\\{}={}\n", regpath, regvalue, thevalue);
    let binaryiniline = iniline.as_bytes();
    inifile
        .write_all(binaryiniline)
        .expect("could not write line");
}

fn main() -> std::io::Result<()> {

    let mut inifile = File::create("output.ini")?;
    inifile.write_all(b"[FileInfo]\n")?;
    inifile.write_all(b"Version=1\n")?;
    inifile.write_all(b"[Machine]\n")?;
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName", &inifile);
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentVersion", &inifile);
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionID", &inifile);
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ReleaseId", &inifile);
    regreadvalue(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid", &inifile);
    regreadvalue(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "ComputerName", &inifile);
    regreadvalue(r#"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"#, "Domain", &inifile);
    regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "ComputerHardwareId", &inifile);
    regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemManufacturer", &inifile);
    regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemProductName", &inifile);
    regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSVersion", &inifile);
    regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSReleaseDate", &inifile);
    regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "EnclosureType", &inifile);

    Ok(())
}