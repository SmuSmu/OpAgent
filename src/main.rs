extern crate winreg;

use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ}; 

fn regreadvalue (regpath: &str, regvalue: &str) {
    let hklm = winreg::RegKey::predef(HKEY_LOCAL_MACHINE);

    let subkey = hklm.open_subkey_with_flags(regpath, KEY_READ)
                    .expect("Failed to open subkey");
    let thevalue: String = subkey.get_value(regvalue)
                    .expect("Failed to read product name");
    
    println!("{}\\{}={}", regpath, regvalue, thevalue);
}

fn main() {


    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ProductName");
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "CurrentVersion");
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "EditionID");
    regreadvalue(r#"SOFTWARE\Microsoft\Windows NT\CurrentVersion"#, "ReleaseId");
    regreadvalue(r#"SOFTWARE\Microsoft\Cryptography"#, "MachineGuid");


}