extern crate winreg;
extern crate serde_json;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::io::prelude::Write;
use serde::{Serialize};


#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct DataXhange {
    FileVersion: u8,
    Machine: Machine,
    Windows: Windows,
    SystemInformation: SystemInformation,
    HardwareConfig: HardwareConfig,
    Software: Vec<Software>,
    SoftwareWOW6432Node: Vec<Software>,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct Software {
    DisplayName: String, 
    Publisher: String, 
    DisplayVersion: String, 
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct Machine {
    MachineGuid: String, 
    ComputerName: String, 
    Domain: String, 
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct Windows {
    ReleaseId: String, 
    ProductName: String, 
    InstallDate: String, 
    InstallTime: String, 
    EditionID: String,
    EditionSubManufacturer: String,
    EditionSubstring: String,
    EditionSubVersion: String,
    InstallationType: String,
    CurrentVersion: String,
    CurrentType: String,
    CurrentMajorVersionNumber: String,
    CurrentMinorVersionNumber: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct SystemInformation {
    SystemManufacturer: String, 
    SystemProductName: String, 
    BIOSVersion: String, 
    BIOSReleaseDate: String, 
    ComputerHardwareId: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct HardwareConfig {
    BaseBoardManufacturer: String, 
    BaseBoardProduct: String, 
    BIOSReleaseDate: String, 
    BIOSVendor: String, 
    BIOSVersion: String, 
    EnclosureType: String, 
    SystemBiosVersion: String, 
    SystemFamily: String, 
    SystemManufacturer: String, 
    SystemProductName: String, 
    SystemSKU: String, 
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

fn regreadvaluetoinifile(regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
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

fn regreadvalue(regpath: &str, regvalue: &str) ->String {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
    let thevalue = match subkey {
        Ok(subkey) => {
            let v = subkey.get_raw_value(regvalue).unwrap();
            display_reg_value(&v)
        }
        Err(_) => "".to_string(),
    };
    return thevalue.trim().to_string();
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
        regreadvaluetoinifile(regpath, name.as_str(), &inifile);
    }
}

fn main() -> std::io::Result<()> {

    let testvec = Software {
        DisplayName : "Software".to_string(),
        Publisher : "Ms".to_string(),
        DisplayVersion : "1".to_string(),
    };
    let testvec2 = Software {
        DisplayName : "Software 2".to_string(),
        Publisher : "Ms".to_string(),
        DisplayVersion : "2".to_string(),
    };

    let mut myjson = DataXhange {
        FileVersion : 1 ,
        Machine : Machine {
            MachineGuid: regreadvalue(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid"), 
            ComputerName: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "ComputerName"), 
            Domain: regreadvalue(r#"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"#, "Domain"), 
            } ,
        Software : Vec::new(),
        SoftwareWOW6432Node : Vec::new(),
        Windows : Windows {
            ReleaseId: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ReleaseID"), 
            ProductName: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName"), 
            InstallDate: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "InstallDate"), 
            InstallTime: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "InstallTime"), 
            EditionID: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionID"), 
            EditionSubManufacturer: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionSubManufacturer"), 
            EditionSubstring: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionSubstring"), 
            EditionSubVersion: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionSubVersion"), 
            InstallationType: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "InstallationType"), 
            CurrentVersion: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentVersion"), 
            CurrentType: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentType"), 
            CurrentMajorVersionNumber: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentMajorVersionNumber"), 
            CurrentMinorVersionNumber: regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentMinorVersionNumber"), 
            } ,
        SystemInformation : SystemInformation {
            SystemManufacturer: regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemManufacturer"), 
            SystemProductName: regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemProductName"), 
            ComputerHardwareId: regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "ComputerHardwareId"), 
            BIOSVersion: regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSVersion"), 
            BIOSReleaseDate: regreadvalue(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSReleaseDate"), 
            } ,
        HardwareConfig : HardwareConfig {
            BaseBoardManufacturer: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "BaseBoardManufacturer"),
            BaseBoardProduct: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "BaseBoardProduct"),
            BIOSReleaseDate: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "BIOSReleaseDate"),
            BIOSVendor: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "BIOSVendor"),
            BIOSVersion: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "BIOSVersion"),
            EnclosureType: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "EnclosureType"),
            SystemBiosVersion: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "SystemBiosVersion"),
            SystemFamily: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "SystemFamily"),
            SystemManufacturer: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "SystemManufacturer"),
            SystemProductName: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "SystemProductName"),
            SystemSKU: regreadvalue(r#"SYSTEM\HardwareConfig\Current"#, "SystemSKU"),
            }
        };
    
    myjson.Software.push(testvec);
    myjson.Software.push(testvec2);
    
    println!("{}", serde_json::to_string(&myjson).unwrap());

    


    let mut inifile = std::fs::File::create("output.ini")?;
    let mut jsonfile = std::fs::File::create("output.json")?;

    //jsonfile.write_all(serde_json::to_string(&myjson).unwrap().as_bytes())?;
    jsonfile.write_all(serde_json::to_string_pretty(&myjson).unwrap().as_bytes())?;
    inifile.write_all(b"Version=1\n")?;
    inifile.write_all(b"[Machine]\n")?;
    
    // already JSON
    regreadvaluetoinifile(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "ComputerName", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"#, "Domain", &inifile);
    regreadvaluetoinifile(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName", &inifile);
    regreadvaluetoinifile(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentVersion", &inifile);
    regreadvaluetoinifile(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionID", &inifile);
    regreadvaluetoinifile(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ReleaseId", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "ComputerHardwareId", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemManufacturer", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "SystemProductName", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSVersion", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\ControlSet001\Control\SystemInformation"#, "BIOSReleaseDate", &inifile);
    regreadvaluetoinifile(r#"SYSTEM\HardwareConfig\Current"#, "EnclosureType", &inifile);
    
    // needs to be in JSON
    inifile.write_all(b"[Software]\n")?;
    regkeyloop(r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall"#, &inifile);
    inifile.write_all(b"[Software_WOW6432Node]\n")?;
    regkeyloop(r#"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"#, &inifile);

    Ok(())
}