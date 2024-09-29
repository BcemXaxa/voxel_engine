#[macro_export]
macro_rules! for_multi {
    ($range:expr $(, $($rest:expr),*)?$(,)?; $func:expr $(; $($iter:ident),*)?) => {
        for i in $range {
            for_multi!($($($rest),*)?; $func; $($($iter,)*)? i);
        }
    };

    (; $func:expr; $($iter:ident),*) => {
        $func($($iter),*)
    };
}
