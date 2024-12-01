pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    ParseErr(String),
    EvalErr(String),
    GeneralErr(String),
}
