use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uchar};

use libloading::{Library, Symbol};

#[link(name = "LibCxx")]
extern "C" {
    fn hello_world();
}

#[cfg(target_os = "windows")]
const LIB_NAME: &str = "LibSharedCxx.dll";

#[cfg(target_os = "linux")]
const LIB_NAME: &str = "libLibSharedCxx.so";

#[cfg(target_os = "macos")]
const LIB_NAME: &str = "libLibSharedCxx.dylib";

fn main() {
    unsafe {
        hello_world();

        let lib = Library::new(LIB_NAME).expect("Failed to load library");

        let writeTiff: Symbol<
            unsafe extern "C" fn(
                filename: *const c_char,
                data: *const c_uchar,
                width: c_int,
                height: c_int,
            ) -> bool,
        > = lib.get(b"writeTiff").expect("Failed to load function");

        let mut data = vec![0u8; 512 * 512];
        for y in 0..512 {
            for x in 0..512 {
                let color = (x + y) as f32 / 1024.0 * 255.0;
                data[y * 512 + x] = color as u8;
            }
        }

        let filename = CString::new("gradient.tif").expect("CString::new failed");
        let result = writeTiff(filename.as_ptr(), data.as_ptr(), 512, 512);
        if result {
            println!("Wrote {}", filename.to_str().unwrap());
        } else {
            println!("Failed to write {}", filename.to_str().unwrap());
        }
    }
}
