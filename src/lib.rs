use gostd::builtin::*;
use gostd::bytes::Buffer;
use gostd::io::{ByteWriter, StringWriter};
use gostd::strings;
use std::collections::HashMap;
use std::io::{BufRead, Error, Read, Write};
use std::sync::Mutex;
use std::{fs, os};

pub trait Settings {
    fn new() -> Self;
    fn property(&self, key: &str) -> Option<String>;
    fn property_slice(&self, key: &str) -> Option<Vec<String>>;
    fn set_property_slice(&mut self, key: &str, value: Vec<String>);
    fn set_property(&mut self, key: &str, value: &str);
    fn load(&mut self, r: impl Read) -> Result<(), Error>;
    fn load_from_file(&mut self, file_path: &str) -> Result<(), Error>;
    fn store(&self, w: impl Write) -> Result<(), Error>;
    fn store_to_file(&self, file_path: &str) -> Result<(), Error>;
    fn line(key: &str, value: &str, buf: &mut Buffer);
    fn parse_line(&mut self, line: &str);
    fn property_names(&self) -> Vec<String>;
    fn is_comment_line(line: &str) -> bool;
}

#[derive(Default)]
pub struct Properties {
    object: Mutex<HashMap<String, String>>,
}

impl Settings for Properties {
    fn new() -> Self {
        Self::default()
    }

    fn property(&self, key: &str) -> Option<String> {
        match self.object.lock().unwrap().get(key) {
            Some(value) => Some(value.to_owned()),
            None => None,
        }
    }

    fn property_slice(&self, key: &str) -> Option<Vec<String>> {
        match self.object.lock().unwrap().get(key) {
            Some(value) => Some(
                strings::Split(value, ",")
                    .iter()
                    .map(|&x| x.to_owned())
                    .collect(),
            ),
            None => None,
        }
    }

    fn set_property_slice(&mut self, key: &str, values: Vec<String>) {
        let value = strings::Join(values, ",");
        self.set_property(key, &value);
    }

    fn set_property(&mut self, key: &str, value: &str) {
        self.object
            .lock()
            .unwrap()
            .insert(key.to_owned(), value.to_owned());
    }

    fn load(&mut self, r: impl Read) -> Result<(), Error> {
        let mut br = std::io::BufReader::new(r);
        let mut line = String::new();
        loop {
            match br.read_line(&mut line) {
                Err(err) => err,
                Ok(i) => {
                    if i == 0 {
                        break;
                    } else {
                        self.parse_line(&line);
                        continue;
                    }
                }
            };
        }
        Ok(())
    }

    fn load_from_file(&mut self, file_path: &str) -> Result<(), Error> {
        let f = fs::File::open(file_path)?;
        self.load(f)
    }

    fn store(&self, mut w: impl Write) -> Result<(), Error> {
        let mut buf = Buffer::new();
        self.object
            .lock()
            .unwrap()
            .iter()
            .for_each(|(k, v)| Self::line(&k, &v, &mut buf));
        w.write(buf.Bytes().as_slice())?;
        Ok(())
    }

    fn store_to_file(&self, file_path: &str) -> Result<(), Error> {
        let f = fs::File::create(file_path)?;
        self.store(f)?;
        Ok(())
    }

    fn line(key: &str, value: &str, buf: &mut Buffer) {
        buf.WriteString(key);
        buf.WriteString(" = ");
        buf.WriteString(value);
        buf.WriteByte(b'\n');
    }

    fn parse_line(&mut self, line: &str) {
        let lineStr = strings::TrimSpace(line);
        if Self::is_comment_line(lineStr) {
            return;
        }
        let splitStrs = strings::Split(lineStr, "=");
        let key = strings::TrimSpace(splitStrs[0]);
        let value = strings::TrimSpace(splitStrs[1]);
        self.set_property(key, value);
    }

    fn property_names(&self) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        for (k, _) in self.object.lock().unwrap().iter() {
            names.push(k.to_owned())
        }
        names
    }

    fn is_comment_line(line: &str) -> bool {
        if line.len() == 0 {
            return true;
        }
        if strings::HasPrefix(line, "#")
            || strings::HasPrefix(line, "//")
            || strings::HasPrefix(line, "/*")
        {
            return true;
        }
        false
    }
}
