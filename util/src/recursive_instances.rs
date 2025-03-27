//! In Haskell, a recursive function such as this:
//! ```haskell
//! nested :: Show a => Int -> a -> String
//! nested 0 x       = show x
//! nested n x | n>0 = nested (n-1) [x]
//!
//! main = do
//!   n <- getLine
//!   putStrLn (nested (read n) 'a')
//! ```
//! will apparently make it fail to resolve a recursive instance, meaning that it will resort to
//! dynamic-dispatched typeclass dictionaries. Let us see if Rust will _also_ resort to dynamic-dispatch
//! dictionaries, or if the typechecker will just reach recursion-limit??

use std::fmt::{Debug, Error, Formatter};
// error: reached the recursion limit while instantiating `nested::<[[...; 1]; 1]>`
// this fails to compile (when used) because static-dispatch here is undecidable !!!
// fn nested<T: Debug>(n: u32, t: T) -> String {
//     match n {
//         0 => format!("{t:?}"),
//         _ => nested(n - 1, [t]),
//     }
// }

// // however this here dispatches just as expected, and works exactly like Haskell would
fn nested(n: u32, t: Box<dyn Debug>) -> String {
    match n {
        0 => format!("{t:?}"),
        _ => nested(n - 1, Box::new([t])),
    }
}

#[cfg(test)]
mod tests {
    use crate::recursive_instances::{nested};

    #[test]
    fn test_nested_instances() {
        assert_eq!("[\"foo\"]", nested(1, Box::new("foo")));
        assert_eq!("[[[\"foo\"]]]", nested(3, Box::new("foo")));
    }
}
