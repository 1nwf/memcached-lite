use crate::Entry;
#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteResponse {
    Deleted,
    NotFound,
}
#[derive(Debug, PartialEq, Eq)]
pub enum StoreResponse {
    Stored,
    NotStored,
    Exists,
    NotFound,
}

impl Response {
    pub fn from_string(s: &str) -> Self {
        println!("res: {:?}", s);
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
            return Self::Retrieve(Entry::from_res_str(&s[idx + 1..]));
        }
    }

    pub fn to_string(&self) -> String {
        let s = match self {
            Response::Store(s) => s.to_string(),
            Response::Retrieve(e) => format!("VALUE {}", e.to_res_str()),
            Response::End => format!("END\r\n"),
            Response::Error => format!("ERROR\r\n"),
            Response::ClientError => format!("CLIENT_ERROR\r\n"),
            Response::ServerError => format!("SERVER_ERROR\r\n"),
            _ => panic!("invalid response")
            // Response::InvalidKey => format!(:),
            // Response::CommandError => todo!(),
            // Response::ValueError => todo!(),
        };
        if *self != Response::End {
            return format!("{}END\r\n", s);
        }
        return s;
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
