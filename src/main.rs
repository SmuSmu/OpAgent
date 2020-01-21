extern crate winreg;
extern crate serde_json;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::io::prelude::Write;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct DataXhange {
    file_version: u8,
    BIOS: BIOS,
    //address: Address,
    //phones: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct BIOS {
    SystemManufacturer: String, 
    SystemProductName: String, 
    BIOSVersion: String, 
    BIOSReleaseDate: String, 
}

fn bytes_to_hex (input: &winreg::RegValue) -> String
{
    let mut output: String = "".to_string();
    for x in &input.bytes {
        output = format!("{} {}", output, format!("{:02x}", x));
    }
    return output.trim().to_string();
}

fn display_reg_value(rv: &winreg::RegValue) -> String {
    use winreg::enums::RegType::*;
    use winreg::types::FromRegValue;
    match rv.vtype {
        REG_NONE                            => "REG_NONE".to_string(),
        REG_SZ                              => String::from_reg_value(rv).unwrap_or_default(),
        REG_EXPAND_SZ                       => String::from_reg_value(rv).unwrap_or_default(),
        REG_MULTI_SZ                        => String::from_reg_value(rv).unwrap_or_default().replace(&['\r', '\n', '\t'][..], " "),
        REG_DWORD                           => u32::from_reg_value(rv).unwrap_or_default().to_string(),
        REG_QWORD                           => u64::from_reg_value(rv).unwrap_or_default().to_string(),
        REG_BINARY                          => bytes_to_hex (rv),
        REG_DWORD_BIG_ENDIAN                => "REG_DWORD_BIG_ENDIAN".to_string(),
        REG_LINK                            => "REG_LINK".to_string(),
        REG_RESOURCE_LIST                   => "REG_RESOURCE_LIST".to_string(),
        REG_FULL_RESOURCE_DESCRIPTOR        => "REG_FULL_RESOURCE_DESCRIPTOR".to_string(),
        REG_RESOURCE_REQUIREMENTS_LIST      => "REG_RESOURCE_REQUIREMENTS_LIST".to_string(),
    }
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
    let iniline = format!("{}\\{}={}\n", regpath, regvalue, thevalue.trim());
    let binaryiniline = iniline.as_bytes();
    inifile
        .write_all(binaryiniline)
        .expect("could not write line");
}

fn regkeyloop(regpath: &str, inifile: &std::fs::File) {
    let subkey = winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(regpath, KEY_READ)
        .unwrap();
    for name in subkey.enum_keys().map(|x| x.unwrap()) {
        //println!("{}", name);
        regvalloop(format!("{}\\{}", regpath, name).as_str(), &inifile);
    }
}

fn regvalloop(regpath: &str, inifile: &std::fs::File) {
    let subkey = winreg::RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(regpath, KEY_READ)
        .unwrap();
    for (name, _value) in subkey.enum_values().map(|x| x.unwrap()) {
        //println!("{}={}", regpath, name);
        regreadvalue(regpath, name.as_str(), &inifile);
    }
}


fn main() -> std::io::Result<()> {

    let myjson = DataXhange {
        file_version : 1 ,
        BIOS : BIOS {
            SystemManufacturer: "String".to_string(), 
            SystemProductName: "String".to_string(), 
            BIOSVersion: "String".to_string(), 
            BIOSReleaseDate: "String".to_string(), 
            }
        };
    println!("{}", serde_json::to_string(&myjson).unwrap());


    let mut inifile = std::fs::File::create("output.ini")?;

    inifile.write_all(serde_json::to_string(&myjson).unwrap().as_bytes())?;
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

    regreadvalue(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "REG_BINARY", &inifile);
    regreadvalue(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "REG_MULTI_SZ", &inifile);
    inifile.write_all(b"[Software]\n")?;
    regkeyloop(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall"#, &inifile);
    inifile.write_all(b"[Software_WOW6432Node]\n")?;
    regkeyloop(r#"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"#, &inifile);

    Ok(())
}