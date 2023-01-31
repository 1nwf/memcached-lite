use std::cell::RefCell;

pub struct Deserializer<'a> {
    pub input: RefCell<&'a str>, // TODO:  use cell?
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer {
            input: RefCell::new(input),
        }
    }

    pub fn is_empty(&self) -> bool {
        return self.input.borrow().is_empty();
    }
    pub fn next_line(&self) -> Result<&str, &str> {
        let mut input = self.input.borrow_mut();
        let idx = input.find("\r\n");
        if let Some(idx) = idx {
            let line = &input[..idx];
            *input = &input[idx + 2..];
            return Ok(line);
        }
        return Err("invalid input");
    }

    pub fn words(&self) -> Vec<&str> {
        return self
            .input
            .borrow()
            .split(' ')
            .filter(|e| !e.is_empty())
            .collect();
    }
}
