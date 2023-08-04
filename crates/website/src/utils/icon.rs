#[macro_export]
macro_rules! icon {
    ($name: ident) => {
        Icon::from(FaIcon::$name)
    };
}
