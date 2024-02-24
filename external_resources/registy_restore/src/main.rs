use std::env;
use std::path::PathBuf;
use std::process::Command;

mod constants;
use constants::{LOCAL_REGISTRY_KEY, USER_REGISTRY_KEY};

fn main() {
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    let local_registry_path = current_dir.join(LOCAL_REGISTRY_KEY);
    let user_registry_path = current_dir.join(USER_REGISTRY_KEY);
    let registry_paths: [&str; 2] = [local_registry_path.to_str().unwrap(), user_registry_path.to_str().unwrap()];

    for path in &registry_paths {
        let mut import_command = Command::new("reg");
        import_command
            .args(["import", path])
            .output()
            .expect("Failed to execute command");
    }
}

