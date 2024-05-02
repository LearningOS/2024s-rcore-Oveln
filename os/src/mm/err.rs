#[derive(Debug)]
pub enum TranslateError {
    NotMapped
}

pub type TranslateResult<T> = Result<T,TranslateError>;