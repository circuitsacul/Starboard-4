#[macro_export]
macro_rules! locale_func {
    ($name:ident) => {
        fn $name() -> [(&'static str, &'static str); 3] {
            use $crate::translations::Lang;
            [
                ("en-US", Lang::En.$name()),
                ("en-GB", Lang::En.$name()),
                ("pt-BR", Lang::Pt.$name()),
            ]
        }
    };
}
