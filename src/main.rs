extern crate winreg;
use std::io;
//use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("Reading system info...");
    //Open predefined reg key
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    //Create subkey and all missing parent keys with KEY_ALL_ACCESS permissions
    //use create_subkey_with_flags to create with different permissions
    let (gkey, disp) = hklm.create_subkey("SOFTWARE\\Policies\\Google\\Chrome")?;
    //if succeeds return a tuple with subkey create and its disposition,
    //REG_CREATED_NEW_KEY or REG_OPENED_EXISTING_KEY
    match disp {
        REG_CREATED_NEW_KEY => println!("A new key was created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened")
    }
    //converts a value from a rust type, writed it to the registry value "ToRegValue" (DWord = u32)
    //print value of "IncognitoModeAvailability"
    gkey.set_value("IncognitoModeAvailability", &2u32)?;
    let dword_val: u32 = gkey.get_value("IncognitoModeAvailability")?;
    println!("IncognitoModeAvailability = {}", dword_val);

    //check if subkey has been created and handle error
    hklm.open_subkey("SOFTWARE\\Policies\\Google\\Chrome").unwrap_or_else(|e| match e.kind() {
        io::ErrorKind::NotFound => panic!("Key doesn't exist"),
        io::ErrorKind::PermissionDenied => panic!("Access denied"),
        _ => panic!("{:?}", e),
    });
    Ok(())
}
