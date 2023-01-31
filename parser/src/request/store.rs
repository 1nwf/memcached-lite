use crate::Entry;

pub const STORE_COMMANDS: [&str; 5] = ["set", "add", "replace", "append", "prepend"];
pub const RETRIEVE_COMMANDS: [&str; 2] = ["get", "gets"];
pub const DELETE_COMMAND: &str = "delete";
pub const FLUSH_COMMAND: &str = "flush_all";

#[derive(Debug)]
pub enum StoreRequest {
    Set(Entry),
    Add(Entry),
    Replace(Entry),
    Append(Entry),
    Prepend(Entry),
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
    pub fn from_str(s: &str) -> StoreRequest {
        let idx = s.find(" ").unwrap();
        let cmd = &s[..idx];
        let entry_string = &s[idx + 1..];
        let request = Self::get_cmd_from_str(cmd);
        let entry = Entry::from_req_str(entry_string);
        return request(entry);
    }
    pub fn to_string(&self) -> String {
        match self {
            StoreRequest::Set(e) => format!("set {}", e.to_req_str()),
            StoreRequest::Add(e) => format!("add {}", e.to_req_str()),
            StoreRequest::Replace(e) => format!("replace {}", e.to_req_str()),
            StoreRequest::Append(e) => format!("append {}", e.to_req_str()),
            StoreRequest::Prepend(e) => format!("prepend {}", e.to_req_str()),
        }
    }
}
