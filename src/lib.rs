//! gostd_settings is a library for reading and writing property files. gostd_settings can be saved in or loaded from the stream. Each key and its corresponding value in the attribute list is a string. It is thread safe: multiple threads can share a single gostd_ Settings object without external synchronization.
//! <details class="rustdoc-toggle top-doc">
//! <summary class="docblock">zh-cn</summary>
//! gostd_settings 是一个用于读写属性文件的库。gostd_settings可保存在流中或从流中加载。 属性列表中每个键及其对应值都是一个字符串。它是线程安全的：多个线程可以共享单个 gostd_settings 对象而无需进行外部同步。
//! </details>
//!
//! # Example
//! ```
//!    use gostd_settings::{Settings, builder};
//!    let mut p = builder().file_type_properties().build();
//!    p.set_property("HttpPort", "8081");
//!    p.set_property(
//!        "MongoServer",
//!        "mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest",
//!    );
//!    p.set_property_slice(
//!        "LogLevel",
//!        ["Debug".to_owned(), "Info".to_owned(), "Warn".to_owned()].to_vec(),
//!    );
//!    match p.store_to_file("config.properties") {
//!        Ok(()) => println!("store to file app.conf success"),
//!        Err(err) => println!("store to file app.conf failed: {}", err),
//!    }
//! ```
//! # Output
//! ```
//! $ cat config.properties
//! HttpPort = 8081
//! LogLevel = Debug,Info,Warn
//! MongoServer = mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest
//! ```
use gostd::builtin::*;
use gostd::bytes::Buffer;
use gostd::io::{ByteWriter, StringWriter};
use gostd::strings;
use std::collections::HashMap;
use std::io::{BufRead, Error, Read, Write};
use std::sync::Mutex;
use std::{fs, os};

pub trait Settings {
    fn property(&self, key: &str) -> Option<String>;
    fn property_slice(&self, key: &str) -> Option<Vec<String>>;
    fn set_property_slice(&mut self, key: &str, value: Vec<String>);
    fn set_property(&mut self, key: &str, value: &str);
    fn load(&mut self, r: impl Read) -> Result<(), Error>;
    /// Reads a property list from a file
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// load_from_file 从文件中读取属性列表
    /// </details>
    ///
    /// # Example
    /// ```
    ///    use gostd_settings::{Settings, builder};
    ///    let mut p = builder().file_type_properties().build();
    /// ```
    fn load_from_file(&mut self, file_path: &str) -> Result<(), Error>;
    fn store(&self, w: impl Write) -> Result<(), Error>;
    fn store_to_file(&self, file_path: &str) -> Result<(), Error>;
    // fn line(key: &str, value: &str, buf: &mut Buffer);
    // fn parse_line(&mut self, line: &str);
    fn property_names(&self) -> Vec<String>;
    // fn is_comment_line(line: &str) -> bool;
}

pub fn builder() -> SettingsBuilder {
    SettingsBuilder { properties: false }
}

#[derive(Default)]
struct Properties {
    object: Mutex<HashMap<String, Value>>,
}

enum Value {
    V(String),
    Map(HashMap<String, String>),
}

pub struct SettingsBuilder {
    properties: bool,
}

impl SettingsBuilder {
    pub fn file_type_properties(&mut self) -> Self {
        Self { properties: true }
    }
    pub fn build(self) -> impl Settings {
        if self.properties {
            return Properties::default();
        }
        return Properties::default();
    }
}

impl Properties {
    fn line(key: &str, value: &str, buf: &mut Buffer) {
        buf.WriteString(key);
        buf.WriteString(" = ");
        buf.WriteString(value);
        buf.WriteByte(b'\n');
    }

    fn parse_line(&mut self, line: &str) {
        print!("lineStr: {}", line);
        let line_str = strings::TrimSpace(line);
        if Self::is_comment_line(line_str) {
            return;
        }
        let split_strs = strings::Split(line_str, "=");
        let key = strings::TrimSpace(split_strs[0]);
        let value = strings::TrimSpace(split_strs[1]);
        println!("value: {}", value);
        self.set_property(key, value);
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

impl Settings for Properties {
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
                    .map(|s| s.to_string())
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
            if let Ok(i) = br.read_line(&mut line) {
                if i == 0 {
                    break;
                } else {
                    self.parse_line(&line);
                    line.clear();
                }
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "read_line failed",
                ));
            }
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

    fn property_names(&self) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        for (k, _) in self.object.lock().unwrap().iter() {
            names.push(k.to_owned())
        }
        names
    }
}
