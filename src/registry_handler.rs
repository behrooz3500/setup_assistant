// +------------------+
// |    dependencies  |
// +------------------+

// std
use std::fs;
use std::io;
use std::env;
use std::path::PathBuf;
use std::process::Command;

// winreg
use winreg::enums::{
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_ALL_ACCESS, KEY_READ, KEY_WOW64_32KEY,
    KEY_WOW64_64KEY,
};
use winreg::RegKey;

// internal: constants
use crate::constants::*;

// internal: utilities
use crate::utilities::is_64bit_os;

// +-------------------+
// | static variables  |
// +-------------------+

static HKCU: RegKey = RegKey::predef(HKEY_CURRENT_USER);
static HKLM: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);

// +------------------+
// | public functions |
// +------------------+

/// This function checks the registry key that is set after executing reboot command
///
/// # Returns
/// `bool` - whether the key is set or not
pub fn _is_reboot_performed() -> bool {
    match &HKLM.open_subkey_with_flags(REBOOT_REGISTRY_PATH, KEY_READ | KEY_WOW64_64KEY) {
        Ok(reg_key) => match reg_key.get_value::<String, &str>(REBOOTED_KEY_NAME) {
            Ok(value) => value == REBOOTED_KEY_VALUE,
            Err(_) => false,
        },
        Err(_) => false,
    }
}

/// Set a registry key in the local machine registry to show the reboot status.
///
/// # Arguments
///
/// * `value` - The value to be set on the "rebooted" key to
///
/// # Returns
///
/// `std::io::Result<()>` - Whether the operation was successful or not
pub fn set_rebooted_key(value: i8) -> std::io::Result<()> {
    match &HKLM.create_subkey_with_flags(REBOOT_REGISTRY_PATH, KEY_ALL_ACCESS | KEY_WOW64_64KEY) {
        Ok((reg_key, _)) => {
            reg_key
                .set_value(REBOOTED_KEY_NAME, &value.to_string())
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

/// schedule one time tasks after reboot
/// A task for setup.exe and another for registry restoration is scheduled
///
/// # Returns
///
/// `std::io::Result<()>` - Whether the operation was successful or not
pub fn schedule_setup_task() -> std::io::Result<()> {
    let current_path = env::current_dir()?;
    let setup_path = current_path.join(DATA_FOLDER_NAME).join(SETUP_EXE_NAME);
    let registry_restore_path = current_path
        .join(DATA_FOLDER_NAME)
        .join(REGISTRY_RESTORE_EXECUTABLE);

    match &HKLM.open_subkey_with_flags(REGISTRY_RUNONCE_PATH, KEY_ALL_ACCESS | KEY_WOW64_64KEY) {
        Ok(reg_key) => {
            match reg_key.set_value("MoeinAssistant", &setup_path.to_str().unwrap()) {
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

/// Export startup registry keys to file and then delete the registry keys
///
/// # Returns
///
/// `std::io::Result<()>` - Whether the operation was successful or not
pub fn export_and_delete_startup_registry_keys() -> std::io::Result<()> {
    let current_path: PathBuf = env::current_dir().expect("failed to get the current directory");
    let registry_hives: [(&str, &RegKey); 2] =
        [("HKEY_LOCAL_MACHINE", &HKLM), ("HKEY_CURRENT_USER", &HKCU)];
    for (hive_name, reg_key) in &registry_hives {
        export_and_delete_registry_key(reg_key, &current_path, hive_name)?;
    }
    Ok(())
}

// +-----------------------+
// |  private functions    |
// +-----------------------+

/// Export and delete a registry key.
///
/// This function exports the specified registry key to a file, then deletes the exported registry key.
///
/// # Arguments
///
/// * `reg_key` - A reference to the registry key.
/// * `current_path` - The current path where the application is running.
/// * `hive_name` - Name of the registry hive.
///
/// # Returns
///
/// A `Result` indicating success (`Ok`) or an `std::io::Error` if an error occurs.
///
fn export_and_delete_registry_key(
    reg_key: &RegKey,
    current_path: &PathBuf,
    hive_name: &str,
) -> Result<(), std::io::Error> {
    let is_64bit = is_64bit_os();

    if is_64bit {
        log::info!("X64 os found.");
        log::info!("Attempting to create registry backup files for both 32 and 64bit paths.");

        // Open registry key with 64-bit access
        match reg_key
            .open_subkey_with_flags(REGISTRY_STARTUP_PATH, KEY_ALL_ACCESS | KEY_WOW64_64KEY)
        {
            Ok(_) => {
                let is_64bit_view = true;
                let key_full_path: String = format!("{}\\{}", hive_name, REGISTRY_STARTUP_PATH);
                let file_extension: &str = "X64.reg";
                let backup_file_path: PathBuf = match create_backup_files(current_path, hive_name, file_extension) {
                    Ok(path) => path,
                    Err(err) => {
                        log::error!("Error creating backup file: {}", err);
                        return Ok(())
                    }
                };
                if let Err(err) = export_registry_key_to_file(backup_file_path, key_full_path.as_str(), is_64bit_view) {
                    log::error!("failed to export registry key to file:{}", err);
                }
                if let Err(err) = delete_registry_key(&key_full_path, is_64bit_view) {
                    log::error!("Failed to delete registry key: {}", err);
                }
            }
            Err(err) => log::error!("Failed to open registry key with 64-bit access. Probably not existing{}{}: {}", hive_name, REGISTRY_STARTUP_PATH, err),
            
        }
        // Open registry key with 32-bit access
        match reg_key
            .open_subkey_with_flags(REGISTRY_STARTUP_PATH, KEY_ALL_ACCESS | KEY_WOW64_32KEY)
        {
            Ok(_) => {
                let is_64bit_view = false;
                let key_full_path: String = format!("{}\\{}", hive_name, REGISTRY_STARTUP_PATH_WOW);
                let file_extension: &str = "X32.reg";
                let backup_file_path: PathBuf = match create_backup_files(current_path, hive_name, file_extension) {
                    Ok(path) => path,
                    Err(err) => {
                        log::error!("Error creating backup file: {}", err);
                        return Ok(())
                    }
                };
                if let Err(err) = export_registry_key_to_file(backup_file_path, key_full_path.as_str(), is_64bit_view) {
                    log::error!("failed to export registry key to file:{}", err);
                }
                if let Err(err) = delete_registry_key(&key_full_path, is_64bit_view) {
                    log::error!("Failed to delete registry key: {}", err);
                }
            }
            Err(err) => log::error!("Failed to open registry key with 64-bit access. Probably not existing{}{}: {}", hive_name, REGISTRY_STARTUP_PATH, err),
        };
    } else {
        log::info!("X86 os found.");
        log::info!("Attempting to create registry backup files.");

        // Open registry key
        match reg_key.open_subkey_with_flags(REGISTRY_STARTUP_PATH, KEY_ALL_ACCESS) {
            Ok(_) => {
                let key_full_path: String = format!("{}\\{}", hive_name, REGISTRY_STARTUP_PATH);
                let file_extension: &str = ".reg";
                let backup_file_path: PathBuf = match create_backup_files(current_path, hive_name, file_extension) {
                    Ok(path) => path,
                    Err(err) => {
                        log::error!("Error creating backup file: {}", err);
                        return Ok(())
                    }
                };
                if let Err(err) = export_registry_key_to_file(backup_file_path, key_full_path.as_str(), is_64bit) {
                    log::error!("failed to export registry key to file:{}", err);
                }
                if let Err(err) = delete_registry_key(&key_full_path, is_64bit) {
                    log::error!("Failed to delete registry key: {}", err);
                }
                
            }
            Err(err) => log::error!("Failed to open registry key. Probably not existing: {}", err),
        }
    }
    Ok(())
}


/// Create backup file for registry keys
///
/// # Arguments
///
/// * `current_path` - The current path where the application is running.
/// * `hive_name` - Name of the registry hive.
/// * `file_extension` - String to be used as the extension of the created file
///
/// # Returns
///
/// A `Result` indicating success (`Ok`) or an `std::io::Error` if an error occurs.
///
fn create_backup_files(
    current_path: &PathBuf,
    hive_name: &str,
    file_extension: &str,
) -> std::io::Result<PathBuf> {
    let backup_file_path = current_path.join(DATA_FOLDER_NAME).join(format!(
        "{}{}",
        hive_name.to_lowercase(),
        file_extension
    ));
    log::info!(
        "Registry file path for {} in {} is: {}",
        file_extension,
        hive_name,
        backup_file_path.display()
    );

    fs::File::create(&backup_file_path)?;
    log::info!("The file is created successfully.");
    Ok(backup_file_path)
}


/// Export startup registry keys to the created files
///
/// # Arguments
///
/// * `file_path` - Path to the created file
/// * `key_path` - Path to the registry key
///
/// # Returns
///
/// A `Result` indicating success (`Ok`) or an `std::io::Error` if an error occurs.
///
fn export_registry_key_to_file(file_path: PathBuf, key_path: &str, is_64bit_view: bool) -> std::io::Result<()> {
    let mut export_command = Command::new("reg");
    export_command.arg("export");
    export_command.arg(key_path);
    export_command.arg(&file_path);
    export_command.arg("/y");
    if is_64bit_view {
        export_command.arg("/reg:64");
    }

    match export_command.status() {
            Ok(status) => {
                if status.success() {
                    log::info!("registry at {} exported successfully in {}", key_path, &file_path.display());
                    Ok(())
                } else {
                    Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to export registry key {} to file {}", key_path, &file_path.display()),
                ))
                }
            }
            Err(err) => {
                log::error!("Exporting {} to {} completely failed.", key_path, &file_path.display());
                Err(err)
            },
        }
}


/// Delete registry keys
///
/// # Arguments
///
/// * `key_path` - Path to the registry key
///
/// # Returns
///
/// A `Result` indicating success (`Ok`) or an `std::io::Error` if an error occurs.
///
fn delete_registry_key(key_path: &str, is_64bit_view: bool) -> std::io::Result<()> {
    let mut delete_command = Command::new("reg");
    delete_command.arg("delete");
    delete_command.arg(key_path);
    delete_command.arg("/f");
    if is_64bit_view {
        delete_command.arg("/reg:64");
    }

    match delete_command.status() {
            Ok(status) => {
                if status.success() {
                    log::info!("registry at {} deleted successfully", key_path);
                    Ok(())
                } else {
                    Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to delete registry key {}", key_path),
                    ))
                }
            }
            Err(err) => {
                log::error!("Deleting {} completely failed.", key_path);
                Err(err)
            }
        }
}
