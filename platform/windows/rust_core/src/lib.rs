#[unsafe(no_mangle)]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_sub(a: i32, b: i32) -> i32 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_mul(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::{rust_add, rust_mul, rust_sub};

    #[test]
    fn pure_ffi_exports_work() {
        assert_eq!(rust_add(2, 3), 5);
        assert_eq!(rust_sub(9, 4), 5);
        assert_eq!(rust_mul(3, 7), 21);
    }
}
