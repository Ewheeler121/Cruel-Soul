//thank you random blogger
//https://wesleywiser.github.io/post/rust-windows-messagebox-hello-world/    
#[cfg(target_os = "windows")]
pub mod display {
    extern crate user32;
    extern crate winapi;

    use std::ffi::CString;
    use user32::MessageBoxA;
    use winapi::winuser::{MB_OK, MB_ICONINFORMATION, MB_ICONERROR};

    pub fn information(message: &str, title: &str) {
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
    }

    pub fn error(message: &str, title: &str) {
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
    }
}

#[cfg(target_os = "linux")]
pub mod display {
    use dialog_box;

    pub fn information(message: &str, title: &str) {
        dialog_box::information(&message);
    }

    pub fn error(message: &str, title: &str) {
        dialog_box::error(&message);
    }
}
