extern crate user32;
extern crate winapi;

pub mod display {
    use std::ffi::CString;
    use user32::MessageBoxA;
    use winapi::winuser::{MB_OK, MB_ICONINFORMATION, MB_ICONERROR};

    pub fn information(title: &str, message: &str) -> String {
        let lp_text = CString::new(title).unwrap();
        let lp_caption = CString::new(message).unwrap();

        unsafe {
            MessageBoxA(
                std::ptr::null_mut(),
                lp_text.as_ptr(),
                lp_caption.as_ptr(),
                MB_OK | MB_ICONINFORMATION
            ); 
        }

        format!("Printed Information\nTitle: {}\nMessage: {}", title, message)
    }

    pub fn error(title: &str, message: &str) -> String {
        let lp_text = CString::new(title).unwrap();
        let lp_caption = CString::new(message).unwrap();

        unsafe {
            MessageBoxA(
                std::ptr::null_mut(),
                lp_text.as_ptr(),
                lp_caption.as_ptr(),
                MB_OK | MB_ICONERROR
            ); 
        }

        format!("Printed Error\nTitle: {}\nMessage: {}", title, message)
    }
}

