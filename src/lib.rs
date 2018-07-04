extern crate libc;

use std::ffi::CStr;
use std::io;
use std::io::prelude::*;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn new_ref_array() -> *mut Vec<*mut c_void> {
    let foo: Box<Vec<*mut c_void>> = Box::new(Vec::with_capacity(8));
    let vec_pointer = Box::into_raw(foo);
    return vec_pointer as *mut Vec<*mut c_void>;
}

#[no_mangle]
pub extern "C" fn ref_array_store(arr_ptr: *mut Vec<*mut c_void>, n: i32, ref_ptr: *mut c_void) {
    let mut arr = unsafe { Box::from_raw(arr_ptr) };
    let n_size = n as usize;
    let len = n_size + 1;
    if arr.len() < len {
        arr.resize(len, 0 as *mut c_void);
    }
    arr[n_size] = ref_ptr;
    mem::forget(arr);
}

#[no_mangle]
pub extern "C" fn println(arr_ptr: *mut Vec<*mut c_void>) {
    let arr = unsafe { Box::from_raw(arr_ptr) };
    for val in arr.iter() {
        if !val.is_null() {
            let v = (*val) as *const c_char;
            print!("{}", unsafe { CStr::from_ptr(v) }.to_str().unwrap());
        }
    }
    println!();
    io::stdout().flush().ok().expect("Could not flush stdout");
    mem::forget(arr);
}

#[test]
fn insert_sparse() {
    unsafe {
        let ar = new_ref_array();

        let hello = std::ffi::CString::new("Hello, world!").unwrap();
        let c_str = hello.as_ptr() as *mut c_void;
        mem::forget(hello);

        ref_array_store(ar, 4, c_str);
        println(ar);

        let boxed_arr = Box::from_raw(ar);
        assert_eq!(5, boxed_arr.len());
    }
}
