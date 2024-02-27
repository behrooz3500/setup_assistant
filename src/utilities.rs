// std
use std::ptr;
use std::ffi::OsStr;
use std::error::Error;
use std::os::windows::ffi::OsStrExt;

// winapi
use winapi::um::winuser::{
    MessageBoxW,
    MB_ICONINFORMATION,
    MB_ICONWARNING,
    MB_ICONERROR,
    MB_OK,
};
use winapi::um::wow64apiset::IsWow64Process;
use winapi::um::processthreadsapi::GetCurrentProcess;

// log
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};


/// Enum representing different window types
pub enum WindowType {
    _Error,
    Information,
    _Warning,
}


/// Display an OK message box with the specified window title, message, and type
/// 
/// # Arguments
/// 
/// * `window_title` - The title to be displayed in the message box
/// * `window_message` - The message to be displayed in the message box
/// * `window_type` - The type of the message box (Error, Information, or Warning)
pub fn message_box(window_title: &str, window_message: &str, window_type: WindowType) {
    let wide_error_message: Vec<u16> = OsStr::new(window_message)
        .encode_wide()
        .chain(Some(0))
        .collect();
    let wide_window_title: Vec<u16> = OsStr::new(window_title)
        .encode_wide()
        .chain(Some(0))
        .collect();
    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            wide_error_message.as_ptr(),
            wide_window_title.as_ptr(),
            MB_OK | match window_type {
                WindowType::_Error => MB_ICONERROR,
                WindowType::Information => MB_ICONINFORMATION,
                WindowType::_Warning => MB_ICONWARNING,
            },
        );
    };
}


/// Set up file logger for the application using log and log4rs as the backend
/// 
/// # Returns
/// 
/// Result<(), Box<dyn Error>> - Result indicating success or an error
pub fn setup_logging() -> Result<(), Box<dyn Error>> {
    let log_file = FileAppender::builder()
        .append(true)
        .build("data/logs/setup_assistant.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(config)?;
    Ok(())
}



pub fn is_64bit_os() -> bool {
    let mut is_wow64: i32 = 0;
    unsafe {
        IsWow64Process(GetCurrentProcess(), &mut is_wow64);
    }
    is_wow64 != 0
}

