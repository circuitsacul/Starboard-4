use std::error::Error;

pub type Res<T=()> = Result<T, Box<dyn Error + Send + Sync>>;
