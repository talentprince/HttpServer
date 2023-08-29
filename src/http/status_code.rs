use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason(&self) -> &str {
        match self {
            StatusCode::OK => "Ok",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as u16)
    }
}
