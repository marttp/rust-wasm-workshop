#[no_mangle]
// The no_mangle attribute ensures that the function name lands in the binary as is, otherwise you couldn't later call it by name
// extern "C" ensures it uses the C-compatible ABI, and thus what WebAssembly (and JavaScript) expects.
pub extern "C" fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
