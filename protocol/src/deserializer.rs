use std::cell::Cell;

pub struct Deserializer<'a> {
    pub input: Cell<&'a str>, // TODO:  use cell?
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer {
            input: Cell::new(input),
        }
    }

    pub fn next_word(&self) -> Option<&str> {
        let input = self.input.get();
        if input.is_empty() {
            return None;
        }
        let space_idx = input.find(" ");
        if let Some(idx) = space_idx {
            self.input.set(&input[idx + 1..]);
            return Some(&input[..idx]);
        }
        self.input.set("");
        return Some(input);
    }

    pub fn get_input(&self) -> &str {
        return &self.input.get();
    }
    pub fn is_empty(&self) -> bool {
        return self.input.get().is_empty();
    }

    pub fn next_line(&self) -> Option<&str> {
        let input = self.input.get();
        let idx = input.find("\r\n");
        if let Some(idx) = idx {
            let line = &input[..idx];
            self.input.set(&input[idx + 2..]);
            return Some(line);
        }
        return None;
    }

    pub fn words(&self) -> Vec<&str> {
        return self
            .input
            .get()
            .split(' ')
            .filter(|e| !e.is_empty())
            .collect();
    }
    pub fn split_words(s: &str) -> Vec<&str> {
        return s.split(' ').filter(|e| !e.is_empty()).collect();
    }
}
