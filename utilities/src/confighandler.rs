//! Config file handler
//! Cares about formatting and type conversion

use crate::prelude::*;

use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;

use std::collections::HashMap;

/// Value abstraction to convert to and from values
#[derive(Clone, Debug)]
pub enum Value {
    Value(String),
    Array(Vec<String>),
}

struct ConfigSection {
    header: String,
    body: HashMap<String, Value>,
}

impl Value {
    /// Creates an empty value
    pub fn empty() -> Value {
        Value::Value("".to_string())
    }

    /// Creates an empty array value
    pub fn empty_array() -> Value {
        Value::Array(Vec::new())
    }

    pub fn convert_array<T: FromStr>(&self) -> VerboseResult<Vec<T>> {
        match self {
            Value::Array(value_array) => {
                let mut target_array = Vec::with_capacity(value_array.len());

                for value_string in value_array {
                    match value_string.parse::<T>() {
                        Ok(val) => target_array.push(val),
                        Err(_) => create_error!(format!("error parsing array {}", value_string)),
                    }
                }

                Ok(target_array)
            }
            _ => create_error!("key_value has wrong format"),
        }
    }

    pub fn convert_value<T: FromStr>(&self) -> VerboseResult<T> {
        match self {
            Value::Value(value_string) => match value_string.parse::<T>() {
                Ok(val) => Ok(val),
                Err(_) => create_error!(format!("error parsing value {}", value_string)),
            },
            _ => create_error!("key_value has wrong format"),
        }
    }
}

/// Creates a value `Value::Value(String)`, internal conversion to string
///
/// # Arguments
///
/// `value` type has to implement `Display` trait
impl<T> From<&T> for Value
where
    T: Display,
{
    fn from(value: &T) -> Self {
        Value::Value(format!("{}", value))
    }
}

/// Create a value `Value::Array(Vec<String>)`, internal conversion to string
///
/// # Arguments
///
/// `array` array of type, type has to implement `Display` trait
impl<T> From<&[T]> for Value
where
    T: Display,
{
    fn from(array: &[T]) -> Self {
        Value::Array(array.iter().map(|v| format!("{}", v)).collect())
    }
}

/// Handler struct
pub struct ConfigHandler {}

impl ConfigHandler {
    /// Reads the given config file
    ///
    /// # Arguments
    ///
    /// `file_name` file that is going to be read
    pub fn read_config(file_name: &str) -> VerboseResult<HashMap<String, HashMap<String, Value>>> {
        let file = match File::open(file_name) {
            Ok(file) => file,
            Err(msg) => create_error!(format!("error opening config file({}): {}", file_name, msg)),
        };

        let mut infos = HashMap::new();
        let mut current_section: Option<ConfigSection> = None;

        for line_res in BufReader::new(file).lines() {
            if let Ok(line) = line_res {
                let mut trimmed = line.trim().to_string();

                if trimmed.starts_with('#') || trimmed.is_empty() {
                    continue;
                } else if trimmed.starts_with('[') && trimmed.ends_with(']') {
                    trimmed.remove(0);
                    trimmed.pop();

                    if let Some(ref section) = current_section {
                        infos.insert(section.header.clone(), section.body.clone());
                    }

                    current_section = Some(ConfigSection {
                        header: trimmed,
                        body: HashMap::new(),
                    });
                } else {
                    let mut split = trimmed.split('=');

                    let key = match split.nth(0) {
                        Some(key) => key.trim().to_string(),
                        None => {
                            println!("cannot get key from line: {}", trimmed);
                            continue;
                        }
                    };

                    let value = match split.last() {
                        Some(value) => value.trim().to_string(),
                        None => {
                            println!("cannot get value from line: {}", trimmed);
                            continue;
                        }
                    };

                    if value.starts_with('[') && value.ends_with(']') {
                        let mut trimmed_value = value;
                        trimmed_value.remove(0);
                        trimmed_value.pop();

                        let value_split = trimmed_value.split(';');
                        let mut value_array = Vec::new();

                        for v in value_split {
                            if !v.is_empty() {
                                value_array.push(v.trim().to_string());
                            }
                        }

                        if let Some(ref mut section) = current_section {
                            section.body.insert(key, Value::Array(value_array));
                        }
                    } else if let Some(ref mut section) = current_section {
                        section.body.insert(key, Value::Value(value));
                    }
                }
            }
        }

        // also push the last section
        if let Some(section) = current_section {
            infos.insert(section.header, section.body);
        }

        Ok(infos)
    }

    /// writes a formatted config file
    ///
    /// # Arguments
    ///
    /// `file_name` the file to which the config gets written
    /// `sections` the sections and keys that are going to be written
    pub fn write_config(
        file_name: &str,
        sections: &[(String, Vec<(String, Value)>)],
    ) -> VerboseResult<()> {
        let mut file = match File::create(file_name) {
            Ok(file) => file,
            Err(msg) => create_error!(format!(
                "error creating config file({}): {}",
                file_name, msg
            )),
        };

        for (header, body) in sections {
            let fmt_header = format!("[{}]\n", header);

            if file.write_all(fmt_header.as_bytes()).is_err() {
                create_error!(format!("failed writing section: {}", fmt_header));
            }

            for (key, value) in body {
                let fmt_key_value = format!(
                    "{} = {}\n",
                    key,
                    match value {
                        Value::Value(val) => val.clone(),
                        Value::Array(array) => {
                            let mut array_value = "[".to_string();

                            for (i, val) in array.iter().enumerate() {
                                // if element is not the last one
                                if i != array.len() - 1 {
                                    array_value = format!("{}{}, ", array_value, val);
                                } else {
                                    array_value = format!("{}{}", array_value, val);
                                }
                            }

                            format!("{}]", array_value)
                        }
                    }
                );

                if file.write_all(fmt_key_value.as_bytes()).is_err() {
                    create_error!(format!("failed writing key value: {}", fmt_key_value));
                }
            }

            if file.write_all("\n".as_bytes()).is_err() {
                create_error!("failed writing new line");
            }
        }

        Ok(())
    }
}
