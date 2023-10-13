use thiserror::Error;

use crate::user_error::UserError;

#[derive(Error, Debug, Clone, PartialEq)]
#[error("invalid path")]
pub struct InvalidPathError;

impl UserError for InvalidPathError {
    fn user_error(&self) -> String {
        self.to_string()
    }
}

#[derive(Error, Debug, Clone, PartialEq)]
#[error("invalid name: {escaped_name}")]
pub struct InvalidNameError {
    pub name: String,
    pub escaped_name: String,
}

impl InvalidNameError {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            escaped_name: Self::escape_name(name),
        }
    }

    pub fn escape_name(name: &str) -> String {
        String::from_utf8(
            name.bytes()
                .flat_map(|b| std::ascii::escape_default(b))
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}

impl UserError for InvalidNameError {
    fn user_error(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::user_error::UserError;

    use super::InvalidNameError;

    #[test]
    pub fn test_invalid_name_error() {
        assert_eq!(
            InvalidNameError::new("Hello world\0").user_error(),
            "invalid name: Hello world\\x00"
        )
    }
}
