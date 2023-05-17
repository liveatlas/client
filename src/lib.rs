#![deny(clippy::all, unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery)]
// print functions do not work on wasm (it compiles, but is ignored silently)
#![cfg_attr(
    all(target_family = "wasm", not(test)),
    deny(clippy::print_stderr, clippy::print_stdout, clippy::print_literal)
)]

pub fn add(left: usize, right: usize) -> usize {
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
