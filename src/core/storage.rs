use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use strum_macros::{Display, EnumString, IntoStaticStr};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Display, EnumString, IntoStaticStr)]  // strum macros.
enum AvailableCommand {
    #[strum(serialize = "SET")]
    SET,
    #[strum(serialize = "GET")]
    GET,

}

#[derive(Clone)]
pub struct Storage {
    data: Arc<RwLock<HashMap<String, String>>>
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn process_input(&self, input: &str) -> Result<String, String> {
        self.process_command(input)
    }

    fn process_command(&self, input: &str) -> Result<String, String> {
        let cmd = input.split_whitespace().next().unwrap_or("");
        let command_enum = AvailableCommand::from_str(cmd)
            .map_err(|_| format!("Unknown command: {}", cmd))?;

        let handler = match command_enum {
            AvailableCommand::SET => Self::handle_set,
            AvailableCommand::GET => Self::handle_get,
        };

        handler(self, input.split_whitespace().skip(1).collect())
            .map_err(|error| format!("Error: {error:?}"))
    }

    fn handle_set(&self, args: Vec<&str>) -> Result<String, String> {

        let key = args.get(0).ok_or("SET: Key is not specified")?;
        let value = args.get(1).ok_or("SET: Value is not specified")?;
        Ok(self.set(key, value).expect("Failed to set value"))


    }

    pub fn set(&self, key: &str, value: &str) -> Result<String, String> {

        let mut map = self.data.write().expect("RwLock poisoned");
        map.insert(key.to_string(), value.to_string());
        Ok("Ok".to_string())
    }

    fn handle_get(&self, args: Vec<&str>) -> Result<String, String> {

        let key = args.get(0).ok_or("GET: Key is not specified")?;

        let value = self.get(key);

        match value {
            Ok(value) => Ok(value.to_string()),
            Err(error) => Err(error)
        }

    }

    pub fn get(&self, key: &str) -> Result<String, String> {
        let map = self.data.read().expect("RwLock poisoned");
        Ok(map.get(key).cloned().unwrap_or("null".to_string()))


    }
}
