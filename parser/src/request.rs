use crate::Entry;
const STORE_COMMANDS: [&str; 5] = ["set", "add", "replace", "append", "prepend"];
const RETRIEVE_COMMANDS: [&str; 2] = ["get", "gets"];
const DELETE_COMMAND: &str = "delete";
const FLUSH_COMMAND: &str = "flush_all";

#[derive(Debug)]
pub enum Request {
    Store(StoreRequest),
    Retreive(RetrieveRequest),
    FlushAll,
    Delete(String),
}

impl Request {
    pub fn from_str(s: &str) -> Self {
        let idx = s.find(" ");
        if let Some(idx) = idx {
            let cmd = &s[..idx];
            if STORE_COMMANDS.contains(&cmd) {
                return Request::Store(StoreRequest::from_str(s));
            } else if RETRIEVE_COMMANDS.contains(&cmd) {
                return Request::Retreive(RetrieveRequest::from_str(s));
            } else if cmd == DELETE_COMMAND {
                let req = s
                    .split(" ")
                    .filter(|e| !e.is_empty())
                    .collect::<Vec<&str>>();
                if req.len() != 2 {
                    panic!("invalid cmd")
                }
                return Request::Delete(req[1].to_string());
            }
        }
        let cmd = s
            .split(' ')
            .filter(|e| !e.is_empty())
            .collect::<Vec<&str>>();
        if cmd.len() <= 2 && cmd[0] == FLUSH_COMMAND && cmd[1] == "\r\n"
            || cmd[0] == (FLUSH_COMMAND.to_owned() + "\r\n")
        {
            return Request::FlushAll;
        }

        panic!("invalid command");
    }
    pub fn to_string(&self) -> String {
        match self {
            Request::Store(s) => s.to_string(),
            Request::Retreive(r) => r.to_string(),
            Request::FlushAll => "flush_all\r\n".into(),
            Request::Delete(key) => format!("delete {}\r\n", key),
        }
    }
}

#[derive(Debug)]
pub enum StoreRequest {
    Set(Entry),
    Add(Entry),
    Replace(Entry),
    Append(Entry),
    Prepend(Entry),
}

#[derive(Debug)]
pub enum RetrieveRequest {
    Get(String),
    Gets(String),
}

impl StoreRequest {
    fn get_cmd_from_str(s: &str) -> fn(Entry) -> StoreRequest {
        let k = match s {
            "set" => StoreRequest::Set,
            "add" => StoreRequest::Add,
            "replace" => StoreRequest::Replace,
            "append" => StoreRequest::Append,
            "prepend" => StoreRequest::Prepend,
            _ => panic!("invalid"),
        };
        return k;
    }
    fn from_str(s: &str) -> StoreRequest {
        let idx = s.find(" ").unwrap();
        let cmd = &s[..idx];
        let entry_string = &s[idx + 1..];
        let request = Self::get_cmd_from_str(cmd);
        let entry = Entry::from_req_str(entry_string);
        return request(entry);
    }
    fn to_string(&self) -> String {
        match self {
            StoreRequest::Set(e) => format!("set {}", e.to_req_str()),
            StoreRequest::Add(e) => format!("add {}", e.to_req_str()),
            StoreRequest::Replace(e) => format!("replace {}", e.to_req_str()),
            StoreRequest::Append(e) => format!("append {}", e.to_req_str()),
            StoreRequest::Prepend(e) => format!("prepend {}", e.to_req_str()),
        }
    }
}

impl RetrieveRequest {
    fn get_cmd_from_str(s: &str) -> fn(String) -> RetrieveRequest {
        let k = match s {
            "get" => RetrieveRequest::Get,
            "gets" => RetrieveRequest::Gets,
            _ => panic!("invalid"),
        };
        return k;
    }
    fn from_str(s: &str) -> RetrieveRequest {
        let request = s
            .split(" ")
            .filter(|e| !e.is_empty())
            .map(|e| e.trim())
            .collect::<Vec<&str>>();
        if request.len() != 2 {
            panic!("InvalidRequest")
        }
        return Self::get_cmd_from_str(request[0])(request[1].to_string());
    }
    fn to_string(&self) -> String {
        match self {
            RetrieveRequest::Get(key) => format!("get {}\r\n", key),
            RetrieveRequest::Gets(_) => todo!(),
        }
    }
}
