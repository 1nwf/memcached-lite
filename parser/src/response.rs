use crate::Entry;
#[derive(Debug)]
pub enum Response {
    Store(StoreResponse),
    Retrieve(Entry),
    End,
    Error,
    ClientError,
    ServerError,
    // errors
    // send not stored error from the error
    // add to report that these things were implemented
    InvalidKey,
    CommandError,
    ValueError,
}

#[derive(Debug)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}
#[derive(Debug)]
pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}

impl Response {
    pub fn from_string(s: &str) -> Self {
        if s.split(" ").collect::<Vec<&str>>().len() == 1 {
            assert!(&s[s.len() - 2..] == "\r\n");
            match &s[..s.len() - 2] {
                "END" => Self::End,
                "CLIENT_ERROR" => Self::ClientError,
                "SERVER_ERROR" => Self::ServerError,
                "ERROR" => Self::Error,
                _ => Self::Store(StoreResponse::from_string(s)),
            }
        } else {
            let idx = s.find(" ").unwrap();
            if !(&s[..idx] == "VALUE") {
                panic!("invalid response");
            }
            return Self::Retrieve(Entry::from_string(&s[idx + 1..]));
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Response::Store(s) => s.to_string(),
            Response::Retrieve(e) => e.to_string(),
            Response::End => format!("END\r\n"),
            Response::Error => format!("ERROR\r\n"),
            Response::ClientError => format!("CLIENT_ERROR\r\n"),
            Response::ServerError => format!("SERVER_ERROR\r\n"),
            _ => panic!("invalid response")
            // Response::InvalidKey => format!(:),
            // Response::CommandError => todo!(),
            // Response::ValueError => todo!(),
        }
    }
}

impl StoreResponse {
    fn from_string(s: &str) -> Self {
        if s.contains(" ") || &s[s.len() - 2..] != "\r\n" {
            panic!("invalid response")
        }
        match &s[..s.len() - 2] {
            "STORED" => Self::Stored,
            "NOT_STORED" => Self::NotStored,
            "EXISTS" => Self::Exists,
            "NOT_FOUND" => Self::NotFound,
            _ => panic!("invalid response"),
        }
    }
    fn to_string(&self) -> String {
        let s = match self {
            StoreResponse::Stored => "STORED",
            StoreResponse::NotStored => "NOT_STORED",
            StoreResponse::Exists => "EXISTS",
            StoreResponse::NotFound => "NOT_FUND",
        };
        return format!("{}\r\n", s);
    }
}
