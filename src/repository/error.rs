#[derive(Debug)]
pub enum UserError {
    NotFound,
    AlreadyExists,
    InvalidPassword,
    InvalidCredentials,
    InternalError,
}

#[derive(Debug)]
pub enum PageError {
    NotFound,
    AlreadyExists,
    InternalError
}
