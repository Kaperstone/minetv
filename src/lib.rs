extern crate winapi;
extern crate user32;

#[no_mangle] 
pub extern "stdcall" fn DllMain(module: u32, reason_for_call: u32, reserved: u32) -> bool { 
    match reason_for_call { 
        1 => {
            let caption = std::ffi::CString::new("success").unwrap();
			let text = std::ffi::CString::new("mine.tv was success loaded").unwrap();
            
            unsafe {
                user32::MessageBoxA(std::ptr::null_mut(), text.as_ptr(), caption.as_ptr(), 0);
            }
        },
        _ => ()
    };
    true
} 