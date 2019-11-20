extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::fs::File;
use std::io::prelude::*;

fn regreadvalue (regpath: &str, regvalue: &str, mut inifile: &std::fs::File) {
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
    
    Ok(())
}