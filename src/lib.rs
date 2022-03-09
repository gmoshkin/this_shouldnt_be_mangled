use std::convert::TryFrom;

fn generic_calls_extern<T: TryFrom<i32>>() -> Option<T> {
    unsafe {
        this_shouldnt_be_mangled()
    }.try_into().ok()
}

extern "C" {
    fn this_shouldnt_be_mangled() -> i32;
}

#[no_mangle]
pub fn foo() -> usize {
    generic_calls_extern().unwrap()
}

#[no_mangle]
pub fn bar() -> f64 {
    generic_calls_extern().unwrap()
}
