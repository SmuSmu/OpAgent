//#![windows_subsystem = "windows"]
#![windows_subsystem = "console"] 

extern crate winreg;
extern crate serde_json;
extern crate static_vcruntime; //https://users.rust-lang.org/t/static-vcruntime-distribute-windows-msvc-binaries-without-needing-to-deploy-vcruntime-dll/57599

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::io::prelude::Write;
use serde::{Serialize};
use std::collections::HashMap;


#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct DataXhange {
    FileVersion: u8,
    BitnessOS: String,
    Jikwaa: Jikwaa,
    Machine: Machine,
    Windows: Windows,
    SystemInformation: SystemInformation,
    HardwareConfig: HardwareConfig,
    Software: HashMap<String,HashMap<String,String>>,
    SoftwareWOW6432Node: HashMap<String,HashMap<String,String>>
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
struct Jikwaa {
    OrgID: String, 
    SecKey: String,
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

fn os_bitness () -> String {
    match bitness::os_bitness() {
        Ok(bitness) => {
            match bitness {
                bitness::Bitness::X86_32 => return "x32".to_string() ,
                bitness::Bitness::X86_64 => return "x64".to_string() ,
                bitness::Bitness::Unknown => return "Unknown".to_string()
                }
            },
        Err(_) => return "Error".to_string(),
        }
    }

fn bytes_to_hex (input: &winreg::RegValue) -> String {
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

fn regreadvalue(regpath: &str, regvalue: &str) ->String {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
    match subkey {
        Ok(subkey) => {
            let subvalue = subkey.get_raw_value(regvalue);
            let _strange = match subvalue {
                Ok(subvalue) => return display_reg_value(&subvalue),
                Err(_) => return "".to_string(),
                };
            },
        Err(_) => return "".to_string(),
        }
    }

    fn regkeyloop(regpath: &str) -> HashMap<String,HashMap<String,String>> {
        let mut output = HashMap::new();
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
        match subkey {
            Ok(subkey) => {
                for name in subkey.enum_keys().map(|x| x.unwrap()) {
                    //println!("-----{}\\{}", regpath,name);
                    output.insert(name.to_string(), regvalloop(format!("{}\\{}",regpath,name).as_str()));
                    }
                return output;
                },
            Err(_) => return output,
            }
        }

    fn regvalloop(regpath: &str) -> HashMap<String,String> {
        let mut output = HashMap::new();
        let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);
        let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ);
        match subkey {
            Ok(subkey) => {
                for (name, _value) in subkey.enum_values().map(|x| x.unwrap()) {
                    output.insert(name.to_string(), regreadvalue(regpath, name.as_str()));
                    }
                return output;
                },
            Err(_) => return output,
            }
        }

fn main() -> std::io::Result<()> {

    let org_id: String = regreadvalue(r#"SOFTWARE\jikwaa"#, "OrgID");
    let sec_key: String = regreadvalue(r#"SOFTWARE\jikwaa"#, "SecKey");

    //println!("{} : {}", org_id,sec_key);
    let myjson = DataXhange {
        FileVersion : 1 ,
        BitnessOS : os_bitness () ,
        Jikwaa : Jikwaa {
            OrgID: regreadvalue(r#"SOFTWARE\jikwaa"#, "OrgID") ,
            SecKey: regreadvalue(r#"SOFTWARE\jikwaa"#, "SecKey") ,
            } ,
        Machine : Machine {
            MachineGuid: regreadvalue(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid"), 
            ComputerName: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\ComputerName\ComputerName"#, "ComputerName"), 
            Domain: regreadvalue(r#"SYSTEM\CurrentControlSet\Services\Tcpip\Parameters"#, "Domain"), 
            } ,
        Software : regkeyloop (r#"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall"#),
        SoftwareWOW6432Node : regkeyloop (r#"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"#),
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
            SystemManufacturer: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\SystemInformation"#, "SystemManufacturer"), 
            SystemProductName: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\SystemInformation"#, "SystemProductName"), 
            ComputerHardwareId: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\SystemInformation"#, "ComputerHardwareId"), 
            BIOSVersion: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\SystemInformation"#, "BIOSVersion"), 
            BIOSReleaseDate: regreadvalue(r#"SYSTEM\CurrentControlSet\Control\SystemInformation"#, "BIOSReleaseDate"), 
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
    
    //println!("{}", serde_json::to_string(&myjson).unwrap());
    let mut jsonfile = std::fs::File::create("output.json")?;

    

    //jsonfile.write_all(serde_json::to_string(&myjson).unwrap().as_bytes())?;
    jsonfile.write_all(serde_json::to_string_pretty(&myjson).unwrap().as_bytes())?;

    let resp = attohttpc::post("https://jikwaa.net/api/1/inventory.php")
    //let resp = attohttpc::post("https://postman-echo.com/post")   // for testing
        .param("OrgID", org_id)         // set a query parameter
        .param("SecKey", sec_key)       // set a query parameter
        .json(&myjson)?                 // set the request body (json feature required)
        .send()?;                       // send the request

    // Check if the status is a 2XX code.
    if resp.is_success()
        {
        // Consume the response body as text and print it.
        println!("Output : {}", resp.text()?);
        }
    else
        {
        panic!("ServerError : {}", resp.text()?);
        }

    Ok(())
    }