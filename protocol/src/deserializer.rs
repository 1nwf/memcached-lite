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

    pub fn next_word(&self) -> Option<&str> {
        let mut input = self.input.borrow_mut();
        if input.is_empty() {
            return None;
        }
        let space_idx = input.find(" ");
        if let Some(idx) = space_idx {
            let s = &input[..idx];
            *input = &input[idx + 1..];
            return Some(s);
        }
        let s = *input;
        *input = "";
        return Some(s);
    }

    pub fn get_input(&self) -> &str {
        return &self.input.borrow();
    }
    pub fn is_empty(&self) -> bool {
        return self.input.borrow().is_empty();
    }
    // TODO: retrun option instead of result
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
    pub fn split_words(s: &str) -> Vec<&str> {
        return s.split(' ').filter(|e| !e.is_empty()).collect();
    }
}
