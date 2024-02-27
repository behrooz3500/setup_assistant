// internal: constants
pub mod constants;
use constants::{DATA_FOLDER_NAME, SCRIPT_PATH};

// internal: xml_handler
mod xml_handler;
use xml_handler::{remove_files, write_xml_file};

// internal: registry_handler
mod registry_handler;
use registry_handler::{
    export_and_delete_startup_registry_keys, schedule_setup_task, set_rebooted_key,
};

// internal: utilities
mod utilities;
use utilities::{message_box, setup_logging, WindowType};

// std
use std::fs;
use std::path::Path;
use std::process::Command;

// serde-json
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    clean_startup_apps: bool,
    restore_startup_apps: bool,
    first_time_reboot: bool,
    reboot_timer: u32,
    change_locale: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clean_startup_apps: true,
            restore_startup_apps: true,
            first_time_reboot: true,
            reboot_timer: 60,
            change_locale: true,
        }
    }
}

fn main() {
    // Initialize logging
    setup_logging().expect("Failed to initialize logger.");
    log::info!("================================================");
    log::info!("Starting setup assistant.");

    // Read the config file or use default values
    let config_path = Path::new(DATA_FOLDER_NAME).join("config.json");
    let config_data: String = match fs::read_to_string(config_path) {
        Ok(data) => {
            log::info!("config file found!");
            data
        }
        Err(err) => {
            log::error!("failed to read config file: {}", err);
            String::from("")
        }
    };

    let config: Config = if !config_data.is_empty() {
        match serde_json::from_str(&config_data) {
            Ok(config) => {
                log::info!("config file parsed successfully!");
                config
            }
            Err(err) => {
                log::error!("failed to parse config file: {}", err);
                Config::default()
            }
        }
    } else {
        log::info!("config file not found!");
        Config::default()
    };
    log::info!("{:?}", config);

    // Display a message box to show the execution of the app
    let message: &str = "شروع فرایند نصب نرم افزار معین";
    let title: &str = "معین";
    message_box(title, message, WindowType::Information);

    // Create the reboot command
    // todo: set the reboot time based on an external config file
    let mut shutdown_command = Command::new("shutdown");
    shutdown_command.args(["/r", "/t", config.reboot_timer.to_string().as_str()]);

    // Create the script execution command
    let mut script_execution_command = Command::new("cmd");
    script_execution_command.arg("/C").arg(SCRIPT_PATH);

    if config.clean_startup_apps {
        log::info!("exporting registry key...");
        match export_and_delete_startup_registry_keys() {
            Ok(_) => {
                log::info!("registry keys are successfully exported!");
            }
            Err(err) => {
                log::error!("failed to export registry keys: {}", err);
            }
        }
    } else {
        log::info!("registry keys export skipped!");
    }

    if config.change_locale {
        log::info!("changing locale...");
        match write_xml_file() {
            Ok(_) => {
                log::info!("xml file created.");
                match script_execution_command.status() {
                    Ok(_) => {
                        log::info!("script executed successfully!");
                        match remove_files() {
                            Ok(_) => log::info!("files removed successfully!"),
                            Err(err) => {
                                log::error!("failed to remove files: {}", err);
                            }
                        }
                    }
                    Err(err) => {
                        log::error!("failed to execute script: {}", err);
                    }
                }
            }
            Err(err) => log::error!("failed to create xml file: {}", err),
        }
    } else {
        log::info!("locale change skipped!");
    }

    log::info!("scheduling setup task...");
    match schedule_setup_task() {
        Ok(_) => log::info!("setup task scheduled successfully!"),
        Err(err) => {
            log::error!("failed to schedule setup task: {}", err);
        }
    }

    if config.first_time_reboot {
        log::info!("setting rebooted key...");
        match set_rebooted_key(1) {
            Ok(_) => log::info!("rebooted key set successfully!"),
            Err(err) => {
                log::error!("failed to set rebooted key: {}", err);
            }
        }
        log::info!("rebooting...");
        match shutdown_command.status() {
            Ok(_) => log::info!("reboot planned"),
            Err(err) => {
                log::error!("failed to reboot: {}", err);
            }
        }
    } else {
        log::info!("reboot skipped!");
    }

    log::info!("Setup assistant finished.");
}
