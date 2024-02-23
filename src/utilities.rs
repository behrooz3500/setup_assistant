// winapi
use winapi::um::winuser::{MessageBoxW, MB_ICONINFORMATION, MB_OK};


pub fn message_box(window_title: &str, window_message: &str) {
    let wide_error_message: Vec<u16> = OsStr::new(window_message)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    let wide_window_title: Vec<u16> = OsStr::new(window_title)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            wide_error_message.as_ptr(),
            wide_window_title.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        );
    };
}

