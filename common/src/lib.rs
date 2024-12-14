use std::sync::Once;

pub use color_eyre::Result;
pub use grid::*;
pub use itertools::*;
pub use paste::paste;
pub use positioning::*;

pub type SS = &'static str;

pub type Solution<R = usize> = fn(SS) -> R;

pub const CLEAR_TERM: &str = "\x1b[2J\x1b[H";

mod grid;
mod positioning;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| color_eyre::install().unwrap());
}

pub fn to_usize(input: impl AsRef<str>) -> usize {
    input.as_ref().parse().unwrap()
}

pub fn to_isize(input: impl AsRef<str>) -> isize {
    input.as_ref().parse().unwrap()
}

pub fn first<A, B>(tup: (A, B)) -> A {
    tup.0
}

pub fn second<A, B>(tup: (A, B)) -> B {
    tup.1
}

pub fn swap<A, B>(tup: (A, B)) -> (B, A) {
    (tup.1, tup.0)
}

#[macro_export]
macro_rules! boilerplate {
    {
        $($name:ident => { $($input:ident$(($($p:expr),*))? -> $value:expr),* $(,)? })*
    } => {
        fn main() {
            $crate::init();
            $($({
                let input = include_str!(concat!(stringify!($input), ".txt"));
                println!(concat!("Result of ", stringify!($name), ", ", stringify!($input), ": {}"), $name(input $(, $($p),*)?));
            })*)*
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::paste;

            $($(
                paste!{
                    #[test]
                    fn [<$name _ $input $(_ $($p)_*)?>]() {
                        $crate::init();
                        let input = include_str!(concat!(stringify!($input), ".txt"));
                        assert_eq!($name(input $(, $($p),*)?), $value);
                    }
                }
            )*)*

        }
    };
}
