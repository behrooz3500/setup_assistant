// internal: constants
pub mod constants;
use constants::SCRIPT_PATH;

// internal: xml_handler
mod xml_handler;
use xml_handler::{remove_files, write_xml_file};

// internal: registry_handler
mod registry_handler;
use registry_handler::{delete_registry_key, export_registry_key, is_rebooted, schedule_setup_task, set_rebooted_key, message_box};

// internal: utilities
mod utilities;
use utilities::message_box;

// std
use std::process::Command;


fn main() {
    let message: &str = "شروع فرایند نصب نرم افزار معین";
    let title: &str = "معین";
    message_box(title, message);
    
    let mut shutdown_command = Command::new("shutdown");
    shutdown_command.args(["/r", "/t", "60"]);

    let mut script_execution_command = Command::new("cmd");
    script_execution_command.arg("/C").arg(SCRIPT_PATH);

    let need_reboot: bool = !is_rebooted();

    export_registry_key().unwrap();
    delete_registry_key().unwrap();

    match write_xml_file() {
        Ok(_) => match script_execution_command.status() {
            Ok(_) => {
                remove_files().unwrap();
                if need_reboot {
                    set_rebooted_key(1).unwrap();
                    schedule_setup_task().unwrap();  
                    match shutdown_command.status() {
                        Ok(_) => {}
                        Err(err) => println!("Error in reboot: {}", err),
                    }
                }
            }
            Err(err) => println!("Error in command execution: {}", err),
        },
        Err(err) => println!("Error in xml_writer: {}", err),
    }
}
