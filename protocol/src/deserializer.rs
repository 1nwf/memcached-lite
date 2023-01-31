pub struct Deserializer<'a> {
    pub input: &'a str,
}

impl<'a> Deserializer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Deserializer { input }
    }

    pub fn is_empty(&self) -> bool {
        return self.input.is_empty();
    }
    pub fn next_line(&mut self) -> Result<&str, &str> {
        let idx = self.input.find("\r\n");
        if let Some(idx) = idx {
            let line = &self.input[..idx];
            self.input = &self.input[idx + 2..];
            return Ok(line);
        }
        return Err("invalid input");
    }

    pub fn words(&self) -> Vec<&str> {
        return self.input.split(' ').filter(|e| !e.is_empty()).collect();
    }
}
