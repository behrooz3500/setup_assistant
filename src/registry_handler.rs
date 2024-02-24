// internal: constants
use crate::constants::*;

// std
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

// winreg
use winreg::RegKey;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS};



pub fn _is_rebooted() -> bool {
    let hklm: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    match hklm.open_subkey_with_flags(REBOOT_REGISTRY_PATH, KEY_ALL_ACCESS) {
        Ok(reg_key) => match reg_key.get_value::<String, &str>("rebooted") {
            Ok(value) => value == "1",
            Err(_) => false,
        },
        Err(_) => false,
    }
}

pub fn set_rebooted_key(value: i8) -> std::io::Result<()> {
    let hklm: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    match hklm.create_subkey_with_flags(REBOOT_REGISTRY_PATH, KEY_ALL_ACCESS) {
        Ok((reg_key, _)) => {
            reg_key
                .set_value("rebooted", &value.to_string())
                .expect("Failed to set value");
        }
        Err(err) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("failed to create reboot key: {}", err),
            ))
        }
    }
    Ok(())
}

pub fn export_registry_key() -> std::io::Result<()> {

    let current_path: PathBuf = env::current_dir().expect("failed to get the current directory");
    let local_path_header: &str = "HKEY_LOCAL_MACHINE";
    let user_path_header: &str = "HKEY_CURRENT_USER";
    let hklm: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu: RegKey = RegKey::predef(HKEY_CURRENT_USER);

    let registry_headers: [(&str, RegKey); 2] = [(local_path_header, hklm), (user_path_header, hkcu)];

    for (header, reg_key) in &registry_headers {
        let output_file: String = format!("{}.reg", header.to_lowercase());
        let full_path = current_path.join(DATA_FOLDER_NAME).join(output_file);

        match reg_key.open_subkey_with_flags(REGISTRY_STARTUP_PATH, KEY_ALL_ACCESS) {
            Ok(_) => {
                
                match fs::File::create(&full_path) {
                    Ok(_) => {}
                    Err(err) => return Err(err),
                }

                let _status = Command::new("regedit")
                    .arg("/e")
                    .arg(&full_path)
                    .arg(format!("{}\\{}", header, REGISTRY_STARTUP_PATH))
                    .status()?;

                
            },
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, format!("failed to open {} registry keys, probably they do not exist", header)))
        }
    }
    Ok(())
}

pub fn delete_registry_key() -> std::io::Result<()> {
    let local_path_header: &str = "HKEY_LOCAL_MACHINE";
    let user_path_header: &str = "HKEY_CURRENT_USER";
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let registry_headers: [(&str, RegKey); 2] =
        [(local_path_header, hklm), (user_path_header, hkcu)];

    for (header, reg_key) in &registry_headers {
        match reg_key.open_subkey_with_flags(REGISTRY_STARTUP_PATH, KEY_ALL_ACCESS) {
            Ok(_) => {
                let _status = Command::new("cmd")
                    .arg("/C")
                    .arg("reg")
                    .arg("delete")
                    .arg(format!("{}\\{}", header, REGISTRY_STARTUP_PATH))
                    .arg("/f")
                    .status()?;

            }
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, format!("failed to open {} registry keys, probably they do not exist", header))),
        }
    }
    Ok(())
}

pub fn schedule_setup_task() -> std::io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let current_path = env::current_dir()?;
    let setup_path = current_path.join(DATA_FOLDER_NAME).join(SETUP_EXE_NAME);
    let registry_restore_path = current_path.join(DATA_FOLDER_NAME).join(REGISTRY_RESTORE_EXECUTABLE);

    match hklm.open_subkey_with_flags(REGISTRY_RUNONCE_PATH, KEY_ALL_ACCESS) {
        Ok(reg_key) => {
            match reg_key.set_value("testapp", &setup_path.to_str().unwrap()) {
                Ok(_) => {}
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "failed setting value for setup task schedule probably due to permissions",
                    ))
                }
            }
            match reg_key.set_value("registry_restore", &registry_restore_path.to_str().unwrap()) {
                Ok(_) => {}
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "failed setting value for registry restore task schedule probably due to permissions",
                    ))
                }
            }
        }
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "failed to open run once key",
            ))
        }
    }
    Ok(())
}


