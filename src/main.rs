extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};
use std::fs::File;
use std::io::prelude::*;

fn regreadvalue (regpath: &str, regvalue: &str, mut _inifile: std::fs::File) {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);

    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ)
                    .expect("Failed to open subkey");
    let thevalue: String = subkey.get_value(regvalue)
                    .expect("Failed to read product name");
    
    println!("{}\\{}={}", regpath, regvalue, thevalue);
    _inifile.write_all(b"yeah\n");
}


fn main() -> std::io::Result<()> {

    
    //regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentVersion");
    //regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionID");
    //regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ReleaseId");
    //regreadvalue(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid");

    let mut inifile = File::create("foo.txt")?;
    inifile.write_all(b"[main]\n")?;
    inifile.write_all(b"[data]\n")?;
    
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName", inifile);
    
    Ok(())
}