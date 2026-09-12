
/*
    Монитор событий
*/

use serde_json::{json, Value as JsonValue};
use std::fs;

use crate::State;



/*
    Команды монитора
*/
pub enum Command
{
    Set { name: String, value: JsonValue, once: bool },
    Add { name: String, value: f64 },
    Sum { name: String, from: String },
    Stop { name: String },
    Min { name: String, operand: String },
    Max { name: String, operand: String },
}



/*
    Монитор
*/
pub struct Monitor
{
    data: JsonValue,
    cmds: Vec<Command>,
}




impl Monitor
{
    pub fn create() -> Self
    {
        Self
        {
            data: json!({}),
            cmds: Vec::new()
        }
    }


    pub fn set
    (
        &mut self,
        name: &str,
        value: JsonValue,
        once: bool
    )
    -> &mut Self
    {
        self.cmds.push(Command::Set
        {
            name: name.to_string(),
            value,
            once,
        });
        self
    }



    pub fn add
    (
        &mut self,
        name: &str,
        value: f64
    )
    -> &mut Self
    {
        self.cmds.push(Command::Add
        {
            name: name.to_string(),
            value,
        });
        self
    }



    pub fn sum(&mut self, name: &str, from: &str) -> &mut Self
    {
        self.cmds.push(Command::Sum
        {
            name: name.to_string(),
            from: from.to_string(),
        });
        self
    }



    pub fn start(&mut self, name: &str) -> &mut Self
    {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        self.set(name, json!(now), false)
    }



    pub fn stop(&mut self, name: &str) -> &mut Self
    {
        self.cmds.push(Command::Stop
        {
            name: name.to_string(),
        });
        self
    }



    pub fn min
    (
        &mut self,
        name: &str,
        operand: &str
    )
    -> &mut Self
    {
        self.cmds.push(Command::Min
        {
            name: name.to_string(),
            operand: operand.to_string(),
        });
        self
    }



    pub fn max(&mut self, name: &str, operand: &str) -> &mut Self
    {
        self.cmds.push(Command::Max
        {
            name: name.to_string(),
            operand: operand.to_string(),
        });
        self
    }



    fn get(&self, name: &str) -> JsonValue
    {
        let keys: Vec<&str> = name.split('/').collect();
        let mut current = &self.data;

        for key in keys
        {
            if let Some(obj) = current.as_object()
            {
                if let Some(val) = obj.get(key)
                {
                    current = val;
                }
                else
                {
                    return JsonValue::Null;
                }
            }
            else
            {
                return JsonValue::Null;
            }
        }

        current.clone()
    }



    fn set_param
    (
        &mut self,
        name: &str,
        value: JsonValue
    )
    {
        let keys: Vec<&str> = name.split('/').collect();
        if keys.is_empty()
        {
            return;
        }

        // Рекурсивная функция для установки значения по пути
        fn set_recursive(current: &mut JsonValue, keys: &[&str], value: JsonValue)
        {
            if keys.is_empty()
            {
                return;
            }

            if keys.len() == 1
            {
                let key = keys[0];
                if let Some(obj) = current.as_object_mut()
                {
                    obj.insert(key.to_string(), value);
                }
                else
                {
                    let mut new_obj = serde_json::Map::new();
                    new_obj.insert(key.to_string(), value);
                    *current = json!(new_obj);
                }
                return;
            }

            let key = keys[0];
            let rest = &keys[1..];

            // Получаем mutable ссылку на следующий уровень
            let next = if let Some(obj) = current.as_object_mut()
            {
                if obj.contains_key(key)
                {
                    obj.get_mut(key).unwrap()
                }
                else
                {
                    obj.insert(key.to_string(), json!({}));
                    obj.get_mut(key).unwrap()
                }
            }
            else
            {
                let mut new_obj = serde_json::Map::new();
                new_obj.insert(key.to_string(), json!({}));
                *current = json!(new_obj);
                current.as_object_mut().unwrap().get_mut(key).unwrap()
            };

            // Рекурсивно обрабатываем остаток пути
            set_recursive(next, rest, value);
        }

        set_recursive(&mut self.data, &keys, value);
    }




    fn run(&mut self)
    {
        let cmds = std::mem::take(&mut self.cmds);

        for cmd in cmds
        {
            match cmd
            {
                Command::Set { name, value, once } =>
                {
                    if !once || self.get(&name).is_null()
                    {
                        self.set_param(&name, value);
                    }
                }
                Command::Add { name, value } =>
                {
                    let current = self.get(&name);
                    let current_num = current.as_f64().unwrap_or(0.0);
                    self.set_param(&name, json!(current_num + value));
                }
                Command::Sum { name, from } =>
                {
                    let current = self.get(&name);
                    let from_val = self.get(&from);
                    let current_num = current.as_f64().unwrap_or(0.0);
                    let from_num = from_val.as_f64().unwrap_or(0.0);
                    self.set_param(&name, json!(current_num + from_num));
                }
                Command::Stop { name } =>
                {
                    let start = self.get(&name);
                    let start_num = start.as_f64().unwrap_or(0.0);
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs_f64();
                    self.set_param(&name, json!(now - start_num));
                }
                Command::Min { name, operand } =>
                {
                    let current = self.get(&name);
                    let op = self.get(&operand);
                    let current_num = current.as_f64().unwrap_or(f64::INFINITY);
                    let op_num = op.as_f64().unwrap_or(f64::INFINITY);
                    self.set_param(&name, json!(current_num.min(op_num)));
                }
                Command::Max { name, operand } =>
                {
                    let current = self.get(&name);
                    let op = self.get(&operand);
                    let current_num = current.as_f64().unwrap_or(f64::NEG_INFINITY);
                    let op_num = op.as_f64().unwrap_or(f64::NEG_INFINITY);
                    self.set_param(&name, json!(current_num.max(op_num)));
                }
            }
        }
    }



    pub fn flush
    (
        &mut self,
        file: &str,
        state: &mut State
    )
    {
        self.run();

        match serde_yaml::to_string(&self.data)
        {
            Ok(yaml) =>
            {
                match std::fs::write(file, yaml)
                {
                    Ok(_) =>
                    {
                        state.set_ok();
                    }
                    Err(e) =>
                    {
                        state.set_state
                        (
                            "monitor-write-error",
                            json!({ "file": file, "error": e.to_string() })
                        );
                    }
                }
            }
            Err(e) =>
            {
                state.set_state
                (
                    "monitor-serialize-error",
                    json!({ "file": file, "error": e.to_string() })
                );
            }
        }
    }



    pub fn read(&mut self, file: &str) -> Result<(), String>
    {
        let content = fs::read_to_string(file)
            .map_err(|e| format!("Read error: {}", e))?;

        self.data = serde_yaml::from_str(&content)
            .map_err(|e| format!("Parse error: {}", e))?;

        Ok(())
    }



    pub fn data(&self) -> &JsonValue
    {
        &self.data
    }



    pub fn to_yaml(&self) -> String
    {
        serde_yaml::to_string(&self.data)
            .unwrap_or_else(|e| format!("Error: {}", e))
    }
}
