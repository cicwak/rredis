use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
};

use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, IntoStaticStr)]
enum AvailableCommand {
    #[strum(serialize = "SET")]
    SET,
    #[strum(serialize = "GET")]
    GET,
    #[strum(serialize = "DEL")]
    DEL,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, IntoStaticStr)]
pub enum Data {
    String(String),
    Integer(i64),
    Boolean(bool),
    Null(Option<String>),
    // Binary(Vec<u8>),  // Для картинок или произвольных данных
    // List(Vec<Value>), // Можно даже делать вложенные структуры
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    data: Data,
    ttl: i32,
}

#[derive(Clone)]
pub struct Storage {
    data: Arc<RwLock<HashMap<String, Value>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn process_input(&self, input: &str) -> Result<String, String> {
        let data = self.process_command(input);
        match data {
            Ok(value) => Ok(String::from_utf8(self.data_to_vec_u8(value.data)).unwrap()),
            Err(_) => Err("Error: Unknown command".to_string()),
        }
    }

    fn data_to_vec_u8(&self, data: Data) -> Vec<u8> {
        match data {
            Data::String(string) => string.into_bytes(),
            Data::Integer(integer) => integer.to_string().into_bytes(),
            Data::Boolean(boolean) => boolean.to_string().into_bytes(),
            Data::Null(null) => null.unwrap_or("null".to_string()).to_string().into_bytes(),
        }
    }

    fn process_command(&self, input: &str) -> Result<Value, String> {
        let cmd = input.split_whitespace().next().unwrap_or("");
        let command_enum =
            AvailableCommand::from_str(cmd).map_err(|_| format!("Unknown command - {}", cmd))?;

        let handler = match command_enum {
            AvailableCommand::SET => Self::handle_set,
            AvailableCommand::GET => Self::handle_get,
            AvailableCommand::DEL => Self::handle_del,
        };

        handler(self, input.split_whitespace().skip(1).collect())
            .map_err(|error| format!("Error: {error:?}"))
    }

    fn handle_del(&self, args: Vec<&str>) -> Result<Value, String> {
        self.del(args.get(0).ok_or("DEL: Key is not specified")?)
    }

    pub fn del(&self, key: &str) -> Result<Value, String> {
        let mut map = self.data.write().expect("RwLock poisoned");
        map.remove(key);

        Ok(Value {
            data: Data::String("Ok".to_string()),
            ttl: -1,
        })
    }

    fn handle_set(&self, args: Vec<&str>) -> Result<Value, String> {
        let key = args.get(0).ok_or("SET: Key is not specified")?;
        let value = args.get(1).ok_or("SET: Value is not specified")?;
        let ttl = args.get(2).unwrap_or(&"-1");

        let set_data = self.set(key, value, i32::from_str(ttl).ok());
        match set_data {
            Ok(set_data) => Ok(set_data),
            Err(error) => Err(error),
        }
    }

    pub fn set(&self, key: &str, value: &str, ttl: Option<i32>) -> Result<Value, String> {
        let mut map = self.data.write().expect("RwLock poisoned");
        map.insert(
            key.to_string(),
            Value {
                data: Data::from_str(value).unwrap_or(Data::String("None".to_string())),
                ttl: ttl.unwrap_or(-1),
            },
        );

        Ok(Value {
            data: Data::String("Ok".to_string()),
            ttl: ttl.unwrap_or(-1),
        })
    }

    fn handle_get(&self, args: Vec<&str>) -> Result<Value, String> {
        let key = args.get(0).ok_or("GET: Key is not specified")?;
        let value = self.get(key);

        match value {
            Ok(value) => Ok(Value {
                data: value.data,
                ttl: value.ttl,
            }),
            Err(error) => Err(error),
        }
    }

    pub fn get(&self, key: &str) -> Result<Value, String> {
        let map = self.data.read().expect("RwLock poisoned");

        Ok(map.get(key).cloned().unwrap_or(Value {
            data: Data::Null(Option::None),
            ttl: -1,
        }))
    }
}
