use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ErrKind {
    Input,
    Parser,
    Writer,
    Get,
}

#[derive(Debug)]
pub struct IssueParserErr {
    pub msg: String,
    pub kind: ErrKind,
}

impl Error for IssueParserErr {}

impl fmt::Display for IssueParserErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} error: {}", &self.map_err_kind(), &self.msg)
    }
}

impl IssueParserErr {
    fn map_err_kind(&self) -> String {
        let k = match &self.kind {
            ErrKind::Input => "Input",
            ErrKind::Parser => "Parser",
            ErrKind::Writer => "Writer",
            ErrKind::Get => "Get",
        };

        String::from(k)
    }
}
