pub trait ErrToStr {
    fn to_bot_str(&self) -> String;
    fn to_web_str(&self) -> String;
}
