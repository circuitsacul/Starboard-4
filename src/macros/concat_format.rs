/// Designed for multi-line strings with args.
#[macro_export]
macro_rules! concat_format {
    ($($string: literal $(<- $($arg: expr),*)?;)+) => {
        $(
            format!(
                $string,
                $($(
                    $arg,
                )*)*
            ) +&
        )* *""
    };
}
