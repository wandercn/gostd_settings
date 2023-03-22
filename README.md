# [gostd_settings](https://github.com/wandercn/gostd_settings)

[![crates.io](https://img.shields.io/crates/v/gostd_settings.svg?color=yellow)](https://crates.io/crates/gostd_settings)
[![Released API docs](https://docs.rs/gostd_settings/badge.svg)](https://docs.rs/gostd_settings)
[![GPL3 licensed](https://img.shields.io/github/license/wandercn/gostd_settings.svg)](./LICENSE)
[![Downloads of Crates.io](https://img.shields.io/crates/d/gostd_settings.svg)](https://crates.io/crates/gostd_settings)
[![Lines of code](https://img.shields.io/tokei/lines/github/wandercn/gostd_settings.svg)](#)
[![Build](https://img.shields.io/github/actions/workflow/status/wandercn/gostd_settings/.github/workflows/rust.yml?branch=master)](#)
[![Languages](https://img.shields.io/github/languages/top/wandercn/gostd_settings.svg)](#)

gostd_settings is library for reading and writing properties files. 是一个用于读写属性配置文件的库


## 新建配置并保存到文件 

```rust
use gostd_settings::{Settings, builder};
fn main() {
   let mut p = builder().file_type_properties().build();
   p.set_property("HttpPort", "8081");
   p.set_property(
       "MongoServer",
       "mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest",
   );
   p.set_property_slice(
       "LogLevel",
       ["Debug", "Info", "Warn"].iter().map(|s| s.to_string()).collect(),
   );
   match p.store_to_file("config.properties") {
       Ok(()) => println!("store to file app.conf success"),
       Err(err) => println!("store to file app.conf failed: {}", err),
   }
}

// output

    $ cat config.properties
     HttpPort = 8081
     LogLevel = Debug,Info,Warn
     MongoServer = mongodb://10.11.1.5,10.11.1.6,10.11.1.7/?replicaSet=mytest
```

## 从文件读取配置

```rust
use gostd_settings::{builder, Settings};
fn main() -> Result<(), std::io::Error> {
    let file = "./config.properties";
    let mut p = builder().file_type_properties().build();

    p.load_from_file(file)?;

    if let Some(http_prot) = p.property("HttpPort") {
        println!("{}", http_prot)
    }
    if let Some(logLevel) = p.property_slice("LogLevel") {
        println!("{:?}", log_level)
    }
    Ok(())
}

// output

    8081
    ["Debug", "Info", "Warn"]

```

