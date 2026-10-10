use std::io::ErrorKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrMsg {
    pub msg: String,
    pub ek: ErrKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrKind {
    pub err: ErrorKind,
    pub code: u64,
}

// =============== IMPLEMENTATIONS ===============
impl ErrMsg {
    pub fn new(msg: String, err: ErrorKind, code: u64) -> Self {
        Self {
            msg,
            ek: ErrKind::new(err, code),
        }
    }
}

impl ErrKind {
    pub fn new(err: ErrorKind, code: u64) -> Self {
        Self { err, code }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = ErrMsg::new("File not found".to_string(), ErrorKind::NotFound, 404);
        assert_eq!(err.msg, "File not found");
        assert_eq!(err.ek.err, ErrorKind::NotFound);
        assert_eq!(err.ek.code, 404);
    }
}
