use std::sync::Once;

pub use color_eyre::Result;
pub use itertools::*;
pub use paste::paste;

pub type SS = &'static str;

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
