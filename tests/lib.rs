extern crate lbfgsb_sys;

#[test]
fn hello() {
    use lbfgsb_sys::hello as ffi;
    use std::os::raw::c_int;
    let a: c_int = 10;
    let b: c_int = 2;
    let mut result: c_int = 0;

    let _ = unsafe {
        ffi::add_(&a as *const _, &b as *const _, &mut result);
    };

    println!("result: {:?}", result);
}
