#[derive(Debug)]
pub enum AppError{
    ValidationError(String),
    DatabaseError
}