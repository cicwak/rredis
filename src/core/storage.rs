use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, RwLock},
};

use crate::core::{commands::AvailableCommand, time_manager::TimeManager, value::Value};

#[derive(Clone)]
pub struct Storage {
    data: Arc<RwLock<HashMap<String, Value>>>,
    time_manager: TimeManager,
}

impl Storage {
    pub fn new(time_manager: TimeManager) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            time_manager,
        }
    }

    fn data_to_vec_u8(&self, data: String) -> Vec<u8> {
        data.into_bytes()
    }

    // fn data_to_vec_u8(&self, data: Data) -> Vec<u8> {
    //     match data {
    //         Data::String(string) => string.into_bytes(),
    //         Data::Integer(integer) => integer.to_string().into_bytes(),
    //         Data::Boolean(boolean) => boolean.to_string().into_bytes(),
    //         Data::Null(null) =>
    // null.unwrap_or("null".to_string()).to_string().into_bytes(),     }
    // }

    pub fn process_input(&self, input: &str) -> Result<String, String> {
        let data = self.process_command(input);
        match data {
            Ok(value) => Ok(String::from_utf8(self.data_to_vec_u8(value.data)).unwrap()),
            Err(e) => Err(e),
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
            AvailableCommand::PING => Self::hangle_ping,
            AvailableCommand::STATS => Self::handle_stats,
        };

        handler(self, input.split_whitespace().skip(1).collect())
            .map_err(|error| format!("Error: {error:?}"))
    }

    fn handle_stats(&self, _: Vec<&str>) -> Result<Value, String> {
        let map = self.data.read().expect("RwLock poisoned");

        let mut val: Vec<String> = Vec::with_capacity(map.len());
        for (key, ret) in map.iter() {
            if let Some(ret) = map.get(key) {
                // Skip expired entries so STATS reflects only visible (non-expired) keys.
                if self.time_manager.is_expire_time(ret.ttl) {
                    continue;
                }
                val.push(format!("key={},value={},ttl={}", key, ret.data, ret.ttl));
            }
        }

        Ok(Value {
            data: val.join("|"),
            ttl: -1,
        })
    }

    fn hangle_ping(&self, _: Vec<&str>) -> Result<Value, String> {
        Ok(Value {
            data: "PONG".to_string(),
            ttl: -1,
        })
    }

    fn handle_del(&self, args: Vec<&str>) -> Result<Value, String> {
        self.del(args.get(0).ok_or("DEL: Key is not specified")?)
    }

    pub fn del(&self, key: &str) -> Result<Value, String> {
        let mut map = self.data.write().expect("RwLock poisoned");
        map.remove(key);

        Ok(Value {
            data: "Ok".to_string(),
            ttl: -1,
        })
    }

    fn handle_set(&self, args: Vec<&str>) -> Result<Value, String> {
        let key = args.get(0).ok_or("SET: Key is not specified")?;
        let value = args.get(1).ok_or("SET: Value is not specified")?;
        let ttl = args.get(2).unwrap_or(&"-1");
        let ttl = i32::from_str(ttl).expect("TTL is not a number");
        let server_expire_time = self.time_manager.get_expire_time(ttl);

        self.set(key, value, server_expire_time)
    }

    pub fn set(&self, key: &str, value: &str, ttl: i32) -> Result<Value, String> {
        let mut map = self.data.write().expect("RwLock poisoned");
        map.insert(
            key.to_string(),
            Value {
                data: value.to_string(),
                ttl,
            },
        );

        Ok(Value {
            data: "Ok".to_string(),
            ttl: -1,
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
        let value = {
            let map = self.data.read().expect("RwLock poisoned");
            map.get(key).cloned().unwrap_or(Value {
                data: "(null)".to_string(),
                ttl: -1,
            })
        };

        if self.time_manager.is_expire_time(value.ttl) && value.data != "(null)" {
            self.del(key)?;
            return Ok(Value {
                data: "(null)".to_string(),
                ttl: -1,
            });
        }

        Ok(value)
    }
}
