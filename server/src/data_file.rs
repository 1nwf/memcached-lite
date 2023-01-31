use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use protocol::Entry;

pub struct Data {
    file: File,
}

impl Data {
    pub fn new(path: &Path) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();

        Self { file }
    }

    pub fn read_data(&mut self) -> HashMap<String, Entry> {
        let mut contents = String::new();
        let mut data = HashMap::new();
        self.file.read_to_string(&mut contents).unwrap();
        let mut iter = contents.lines();
        while let Some(line) = iter.next() {
            let next = iter.next().unwrap();
            let value = format!("{}\r\n{}\r\n", line, next);
            let e = Entry::from_string(&value, false);
            data.insert(e.key.clone(), e);
        }

        return data;
    }

    pub fn write_data(&mut self, entry: &Entry) {
        self.file
            .write_all(entry.to_string(false).as_bytes())
            .unwrap();
    }
}
