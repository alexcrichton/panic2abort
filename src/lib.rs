#![allow(bad_style)]

pub fn linkme() {}

#[no_mangle]
pub extern fn _Unwind_RaiseException() {
    extern { fn abort(); }
    unsafe { abort(); }
}
