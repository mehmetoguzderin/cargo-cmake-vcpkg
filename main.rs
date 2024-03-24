use std::ffi::{c_char, c_int, c_uchar, CString};

use libloading::{Library, Symbol};

#[cfg(target_os = "windows")]
const LIB_NAME: &str = "./lib-cpp.dll";

#[cfg(target_os = "linux")]
const LIB_NAME: &str = "./liblib-cpp.so";

#[cfg(target_os = "macos")]
const LIB_NAME: &str = "./liblib-cpp.dylib";

fn main() {
    unsafe {
        let lib = Library::new(LIB_NAME).expect("Failed to load library");

        let hello_world: Symbol<unsafe extern "C" fn() -> ()> =
            lib.get(b"hello_world").expect("Failed to load function");

        hello_world();

        let write_tiff: Symbol<
            unsafe extern "C" fn(
                filename: *const c_char,
                data: *const c_uchar,
                width: c_int,
                height: c_int,
            ) -> bool,
        > = lib.get(b"write_tiff").expect("Failed to load function");

        let mut data = vec![0u8; 512 * 512];
        for y in 0..512 {
            for x in 0..512 {
                let mut color = (x + y) as f32 / 1024.0 * 255.0;
                let dx = x as f32 - 256.0;
                let dy = y as f32 - 256.0;
                let distance = (dx * dx + dy * dy).sqrt();
                if distance < 192.0 && distance > 160.0 {
                    color = 255.0;
                }
                data[y * 512 + x] = color as u8;
            }
        }

        let filename = CString::new("gradient.tif").expect("CString::new failed");
        let result = write_tiff(filename.as_ptr(), data.as_ptr(), 512, 512);
        if result {
            println!("Wrote {}", filename.to_str().unwrap());
        } else {
            println!("Failed to write {}", filename.to_str().unwrap());
        }
    }
}
