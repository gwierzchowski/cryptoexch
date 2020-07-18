use std::collections::HashMap;

use actix::prelude::*;

use anyhow::Result;

use regex::Regex;

use serde::Deserialize;

//////////////////////////////////////////////////////////
/// Config

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ConfigConfig {
    Module: String,
}

pub type ConfigFilters = Vec<(String, String, String)>; // pointer, operation, argument

#[derive(Message)]
#[rtype(result = "Result<()>")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ConfigTask {
    pub Api: String,
    pub Format: String,
    pub OutPathMask: String,
    pub NewFileAfter: Option<usize>,
    pub RecycleAfter: Option<usize>,
    pub StopAfter: Option<usize>,
    pub Frequency: Option<u64>,
    pub Filters: Option<ConfigFilters>,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub Config: ConfigConfig,
    pub Tasks: Vec<ConfigTask>,
}


pub fn process_json(json_val: serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
    use std::str::FromStr;
    match json_val {
        Value::Null => Value::Null,
        Value::Bool(b) => Value::Bool(b),
        Value::Number(n) => Value::Number(n),
        Value::String(s) => {
            if let Ok(n) = serde_json::Number::from_str(&s) {
                Value::Number(n)
            } else {
                Value::String(s)
            }
        },
        Value::Array(arr) => {
            let mut v = Vec::with_capacity(arr.len());
            for el in arr {
                v.push(process_json(el));
            }
            Value::Array(v)
        }
        Value::Object(map) => {
            let mut m = serde_json::map::Map::with_capacity(map.len());
            for el in map {
                m.insert(el.0, process_json(el.1));
            }
            Value::Object(m)
        }
    }
}

pub type FilterFun = Box<dyn Fn(&str, &serde_json::Value) -> bool>;

pub fn process_json_with_filters(key:&str, json_val: serde_json::Value, filters:&HashMap<String, FilterFun>) -> serde_json::Value {
    use serde_json::Value;
    use std::str::FromStr;
    match json_val {
        Value::Null => Value::Null,
        Value::Bool(b) => Value::Bool(b),
        Value::Number(n) => Value::Number(n),
        Value::String(s) => {
            if let Ok(n) = serde_json::Number::from_str(&s) {
                Value::Number(n)
            } else {
                Value::String(s)
            }
        },
        Value::Array(arr) => {
            let mut v = Vec::with_capacity(arr.len());
            for el in arr {
                v.push(process_json_with_filters(key, el, filters));
            }
            Value::Array(v)
        }
        Value::Object(map) => {
            let mut m = serde_json::map::Map::new();
            for el in map {
                if let Some(flt) = filters.get(key) {
                    if flt(&el.0, &el.1) {
                        let json = process_json_with_filters(&el.0, el.1, filters);
                        m.insert(el.0, json);
                    }
                } else {
                    let json = process_json_with_filters(&el.0, el.1, filters);
                    m.insert(el.0, json);
                }
            }
            Value::Object(m)
        }
    }
}

pub fn create_filters(filters: &Option<ConfigFilters>, engine: &'static rhai::Engine) -> Result<HashMap<String, FilterFun>> {
    let mut ret_filters = HashMap::new();
    if let Some(cfg_filters) = filters {
        for flt in cfg_filters {
            match flt.1.as_ref() {
                "by_key" => {
                    let key = String::from(&flt.2);
                    ret_filters.insert(flt.0.clone(), Box::new(move |k:&str, _v:&serde_json::Value| k == key) as FilterFun);
                },
                "by_key_re" => {
                    let re = Regex::new(&flt.2)?;
                    ret_filters.insert(flt.0.clone(), Box::new(move |k:&str, _v:&serde_json::Value| re.is_match(k)) as FilterFun);
                },
                "rhai" => {
                    let script = String::from(&flt.2);
                    ret_filters.insert(flt.0.clone(), Box::new(move |k:&str, v:&serde_json::Value| {
                        match engine.parse_json(&v.to_string(), true) {
                            Ok(v_map) => {
                                let mut scope = rhai::Scope::new();
                                scope.push("key", k.to_string());
                                scope.push("val", v_map);
                                match engine.eval_with_scope::<bool>(&mut scope, &script) {
                                    Ok(res) => res,
                                    Err(e) => {
                                        eprintln!("eval_with_scope error: {}", e);
                                        false 
                                    }
                                }
                            },
                            Err(e) => { 
                                eprintln!("parse_json error: {}", e);
                                false 
                            }
                        }
                    }) as FilterFun);
                },
                _ => {bail!("Invalid Filter type: {}", flt.1);}
            }
        }
    }
    Ok(ret_filters)
}

pub fn resolve_filename(template: &str, counter: usize) -> String {
    use chrono::prelude::*;
    if template.contains('%') {
        let filename = template.replace("%$", counter.to_string().as_ref());
        if filename.contains('%') {
            Utc::now().format(&filename).to_string()
        } else {
            filename
        }
    } else {
        template.to_owned()
    }
}

pub async fn handle_task(task: ConfigTask) {
    let handler = super::bitbay::TaskRunner::new().start();
    let task_name = task.Api.clone();
    match handler.send(task).await {
        Err(e) => eprintln!("Task '{}': Dispatch error: {}", task_name, e),
        Ok(Err(e)) => eprintln!("Task '{}': Run error: {}", task_name, e),
        _ => {}
    }
}
