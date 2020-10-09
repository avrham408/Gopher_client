#[derive(Debug, PartialEq)]
pub enum Error {
    GetIpFromAddrs((String, u16)),
    ConnectToServer((String, u16)),
    WriteError,
    PartialBufferWriten,
    ReadError,
    ParseUrl(String),
    UndefinedError,
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::GetIpFromAddrs((url, port)) => write!(f, "Parsing url failed ip:{} port:{}", url, port),
            Self::ConnectToServer((url, port)) => write!(f, "connect to server failed ip:{} port:{}", url, port),
            Self::WriteError => write!(f, "write to server failed"),
            Self::PartialBufferWriten => write!(f, "not all buffer sent to server"),
            Self::ReadError => write!(f, "reading from stream failed"),
            Self::ParseUrl(url) => write!(f, "parsing url failed URL:{}", url),
            Self::UndefinedError => write!(f, "undefined error"),
        }
    }
}

impl std::error::Error for Error {}