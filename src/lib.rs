#![doc(html_playground_url = "https://play.rust-lang.org/")]
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
//!        ["Debug", "Info", "Warn"].iter().map(|s| s.to_string()).collect(),
//!    );
//!    match p.store_to_file("config.properties") {
//!        Ok(()) => println!("store to file app.conf success"),
//!        Err(err) => println!("store to file app.conf failed: {}", err),
//!    }
//! ```
//! # Output
//! ```text
//! $ cat config.properties
//! HttpPort = 8081
//! LogLevel = Debug,Info,Warn
//! MongoServer = mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest
//! ```
#![allow(unused)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use gostd::bytes::Buffer;
use gostd::io::{ByteWriter, StringWriter};
use gostd::strings;
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::sync::Mutex;

/// Summary of read and write methods for management configuration files
/// <details class="rustdoc-toggle top-doc">
/// <summary class="docblock">zh-cn</summary>
/// 管理配置文件的读写方法汇总
/// </details>
pub trait Settings {
    /// Searches for the property with the specified key in this property list.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 用指定的键在此属性列表中搜索属性。
    /// </details>
    fn property(&self, key: &str) -> Option<String>;
    /// Search for attributes in this attribute list using the specified key to return multiple attributes connected by "," converted to slices.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 用指定的键在此属性列表中搜索属性，把","连接的多个属性转换为切片返回。
    /// </details>
    fn property_slice(&self, key: &str) -> Option<Vec<String>>;
    /// Set multiple attributes for the specified key, converting multiple attribute values into a "," concatenated attribute string.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 为指定的键设置多个属性，把多个属性值转换成“，”连接的属性字符串。
    /// </details>
    fn set_property_slice(&mut self, key: &str, value: Vec<String>);
    /// Update the specified key and properties. If the key does not exist, create a new one.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 更新指定的键和属性,如果键不存在就新建。
    /// </details>
    fn set_property(&mut self, key: &str, value: &str);
    /// Reads a property list (key and element pairs) from the input character stream in a simple line-oriented format.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 从输入流读取属性列表。
    /// </details>
    fn load(&mut self, r: impl Read) -> Result<(), Error>;
    /// Reads a property list from a file
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// load_from_file 从文件中读取属性列表
    /// </details>
    ///
    /// # Example
    /// ```
    /// use gostd_settings::{builder, Settings};
    /// fn main() -> Result<(), std::io::Error> {
    ///     let file = "./config.properties";
    ///     let mut p = builder().file_type_properties().build();
    ///
    ///     p.load_from_file(file)?;
    ///
    ///     if let Some(httpProt) = p.property("HttpPort") {
    ///         println!("{}", httpProt)
    ///     }
    ///     if let Some(logLevel) = p.property_slice("LogLevel") {
    ///         println!("{:?}", logLevel)
    ///     }
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Output
    ///
    /// ```text
    /// 8081
    /// ["Debug", "Info", "Warn"]
    /// ```
    fn load_from_file(&mut self, file_path: &str) -> Result<(), Error>;
    /// Writes this property list (key and element pairs) in this Properties table to the output stream in a format suitable for loading into a Properties table using the Load() method.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 将属性列表写入输出流。
    /// </details>
    fn store(&self, w: impl Write) -> Result<(), Error>;
    /// Writes a list of property to a file.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 将属性列表写入文件。
    /// </details>
    ///
    /// # Example
    /// ```
    ///    use gostd_settings::{Settings, builder};
    ///    let mut p = builder().file_type_properties().build();
    ///    p.set_property("HttpPort", "8081");
    ///    p.set_property(
    ///        "MongoServer",
    ///        "mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest",
    ///    );
    ///    p.set_property_slice(
    ///        "LogLevel",
    ///        ["Debug", "Info", "Warn"].iter().map(|s| s.to_string()).collect(),
    ///    );
    ///    match p.store_to_file("config.properties") {
    ///        Ok(()) => println!("store to file app.conf success"),
    ///        Err(err) => println!("store to file app.conf failed: {}", err),
    ///    }
    /// ```
    /// # Output
    /// ```text
    /// $ cat config.properties
    /// HttpPort = 8081
    /// LogLevel = Debug,Info,Warn
    /// MongoServer = mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest
    /// ```
    fn store_to_file(&self, file_path: &str) -> Result<(), Error>;
    /// Returns an enumeration of all keys in the property list.
    /// <details class="rustdoc-toggle top-doc">
    /// <summary class="docblock">zh-cn</summary>
    /// 返回属性列表中所有键的枚举。
    /// </details>
    fn property_names(&self) -> Vec<String>;
}

pub fn builder() -> SettingsBuilder {
    SettingsBuilder { properties: false }
}

#[derive(Default)]
struct Properties {
    object: Mutex<HashMap<String, String>>,
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
        let line_str = strings::TrimSpace(line);
        if Self::is_comment_line(line_str) {
            return;
        }
        let split_strs = strings::Split(line_str, "=");
        let key = strings::TrimSpace(split_strs[0]);
        let value = strings::TrimSpace(split_strs[1]);
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
        let mut br = BufReader::new(r);
        let mut line = String::new();
        loop {
            match br.read_line(&mut line) {
                Ok(i) => {
                    if i == 0 {
                        break;
                    } else {
                        self.parse_line(&line);
                        line.clear();
                    }
                }
                Err(err) => return Err(err),
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
