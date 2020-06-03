use libc::c_int;

extern "C" {
    pub fn hello_();
    pub fn add_(a: *const c_int, b: *const c_int, c: *mut c_int);
}
