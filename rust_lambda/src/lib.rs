//! # functions test
//! Not used. This is an example for running test CI

pub fn hello_from_rust()-> () {
    println!("Hello from Rust!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_from_rust() {
        let res = hello_from_rust();
        assert_eq!(res, ());
    }
}
