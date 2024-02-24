// internal: constants
pub mod constants;
use constants::SCRIPT_PATH;

// internal: xml_handler
mod xml_handler;
use xml_handler::{remove_files, write_xml_file};

// internal: registry_handler
mod registry_handler;
use registry_handler::{
    delete_registry_key, export_registry_key, schedule_setup_task, set_rebooted_key,
};

// internal: utilities
mod utilities;
use utilities::{message_box, setup_logging, WindowType};

// std
use std::process::Command;



fn main() {
    setup_logging().expect("Failed to initialize logger.");
    log::info!("================================================");
    // local date time
    log::info!("Starting setup assistant.");
    
    let message: &str = "شروع فرایند نصب نرم افزار معین";
    let title: &str = "معین";
    message_box(title, message, WindowType::Information);

    let mut shutdown_command = Command::new("shutdown");
    shutdown_command.args(["/r", "/t", "60"]);

    let mut script_execution_command = Command::new("cmd");
    script_execution_command.arg("/C").arg(SCRIPT_PATH);

    // let need_reboot: bool = !is_rebooted();

    log::info!("exporting registry key...");
    match export_registry_key() {
        Ok(_) => {
            log::info!("registry keys are successfully exported!");
        }
        Err(err) => {
            log::error!("failed to export registry keys: {}", err);
        }
    }

    log::info!("deleting registry key...");
    match delete_registry_key() {
        Ok(_) => {
            log::info!("registry keys are successfully deleted!");
        }
        Err(err) => {
            log::error!("failed to delete registry keys: {}", err);
        }
    }

    match write_xml_file() {
        Ok(_) => {
            log::info!("xml file created.");
            match script_execution_command.status() {
                Ok(_) => {
                    log::info!("script executed successfully!");
                    match remove_files(){
                        Ok(_) => log::info!("files removed successfully!"),
                        Err(err) => {
                            log::error!("failed to remove files: {}", err);
                        },
                    }
                    match set_rebooted_key(1) {
                        Ok(_) => log::info!("rebooted key set successfully!"),
                        Err(err) => {
                            log::error!("failed to set rebooted key: {}", err);
                        },
                    }
                    match schedule_setup_task() {
                        Ok(_) => log::info!("setup task scheduled successfully!"),
                        Err(err) => {
                            log::error!("failed to schedule setup task: {}", err);
                        },
                    }
                    match shutdown_command.status() {
                        Ok(_) => log::info!("reboot planned"),
                        Err(err) => {
                            log::error!("failed to reboot: {}", err);
                        },
                    }
                    
                }
                Err(err) => {
                    log::error!("failed to execute script: {}", err);
                }
            }
        }
        Err(err) => log::error!("failed to create xml file: {}", err),
    }
    log::info!("Setup assistant finished.");

}
