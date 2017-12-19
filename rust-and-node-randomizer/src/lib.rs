extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;
use libc::c_char;

#[no_mangle]
pub extern fn randomize(pw: *const c_char, num: u8) -> *const c_char {
    // vector to store the chars
    let mut char_vec = Vec::new();
    
    // transferring C String
    let c_str = unsafe {
        CStr::from_ptr(pw)
    };
    let a = c_str.to_str().unwrap();

    // use that C String
    for c in a.chars() {
        let mut s = (c as u8 + num) as char;
        char_vec.push(s);
    }
    let p = char_vec.iter().cloned().collect::<String>();

    // create a C string
    let new_s = CString::new(p).unwrap();
    let new_pw = new_s.as_ptr();
    std::mem::forget(new_s);
    new_pw
}