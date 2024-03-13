use std::fs;
use std::env;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use log::LevelFilter;
use log4rs::append::file;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};

// mod constants;
// use constants::{LOCAL_REGISTRY_KEY, USER_REGISTRY_KEY};

fn main() {
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    setup_logging().expect("Failed to setup logging");
    log::info!("Current directory: {}", current_dir.display());
    match fs::read_dir(current_dir) {
        Ok(entries) => {
            log::info!("Found some files");
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.path();
                    log::info!("Found file: {}", file_name.display());
                    if  file_name.extension().is_some() && file_name.extension().unwrap() == "reg" {
                        let is_64bit_view: bool = file_name.to_str().unwrap().contains("64");
                        log::info!("Found registry file:{}", file_name.display());
                        match restore_registry(entry.path(), is_64bit_view) {
                            Ok(_) => {
                                log::info!("registry at {} restored successfully", file_name.display());
                            },
                            Err(err) => log::info!("Error restoring registry: {}", err),
                        }
                    }
                }
            }
        }
        Err(err) => println!("Error reading directory: {}", err),
    }
}

fn restore_registry(file_path: PathBuf, is_64bit_view: bool) -> std::io::Result<()> {
    let mut import_command = Command::new("reg");
    import_command.arg("import");
    import_command.arg(&file_path);
    if is_64bit_view {   
        import_command.arg("/reg:64");
    }
    match import_command.status() {
        Ok(status) => {
            if status.success() {
                log::info!("registry at {} imported successfully", file_path.display());
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to import registry file {}", file_path.display()),
                ))
            }
        }
        Err(err) => {
            log::error!("Importing {} completely failed.", file_path.display());
            Err(err)
        }
    }
}


fn setup_logging() -> Result<(), Box<dyn Error>> {
    let log_file = FileAppender::builder()
        .append(true)
        .build("logs/setup_assistant.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config)?;
    Ok(())
}
