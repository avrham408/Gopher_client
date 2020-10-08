#[derive(Debug, PartialEq)]
pub enum Error {
    ParsingUrl((String, u16)),
    ConnectToServer((String, u16)),
    WriteError,
    PartialBufferWriten,
    ReadError,
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ParsingUrl((url, port)) => write!(f, "Parsing url failed ip:{} port:{}", url, port),
            Self::ConnectToServer((url, port)) => write!(f, "connect to server failed ip:{} port:{}", url, port),
            Self::WriteError => write!(f, "write to server failed"),
            Self::PartialBufferWriten => write!(f, "not all buffer sent to server"),
            Self::ReadError => write!(f, "reading from stream failed"),
        }
    }
}

impl std::error::Error for Error {}