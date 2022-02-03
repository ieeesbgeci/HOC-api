#[derive(Debug)]
pub enum ApiError{
	ParseError,
	EnvError,
	DbError,
}
pub enum CheckResponse{
	CheckFlag(bool),
}