/*!
 * Helper structures and functions common for entire application.
 */
use std::any::Any;
use std::collections::HashMap;
use std::str::FromStr;

use actix::prelude::*;

use anyhow::Result;

use async_trait::async_trait;

use lazy_static;

use regex::Regex;

use serde::Deserialize;

/////////////////////////////////////////////////////////
// Config

/// This structure maps keys contained in `Config` key of main configuration file
/// 
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ConfigConfig {
    PIDFile: Option<String>,
}

/// This type maps filters definition of configured task (`Tasks: Filters` key)
/// It is defined as vector of triples: (key, operation, argument)
/// 
pub type ConfigFilters = Vec<(String, String, String)>;

/// This structure maps keys of items contained in `Tasks` key of main configuration file
/// i.e. it maps task configuration.
/// 
#[derive(Message)]
#[rtype(result = "Result<()>")]
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct ConfigTask {
    pub Api: String,
    pub Module: String,
    pub Url: Option<String>,
    pub Format: String,
    pub OutPathMask: String,
    pub NewFileAfter: Option<usize>,
    pub RecycleAfter: Option<usize>,
    pub StopAfter: Option<usize>,
    pub Frequency: Option<u64>,
    pub Filters: Option<ConfigFilters>,
}

/// This structure maps settings in main configuration file
/// 
#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Config {
    pub Config: ConfigConfig,
    pub Tasks: Vec<ConfigTask>,
}

/////////////////////////////////////////////////////////
// Script Engine

#[doc(hidden)]
fn to_float(s: rhai::ImmutableString) -> rhai::FLOAT {
    rhai::FLOAT::from_str(s.as_str()).unwrap_or_default()
}

#[doc(hidden)]
fn to_int(s: rhai::ImmutableString) -> rhai::INT {
    rhai::INT::from_str(s.as_str()).unwrap_or_default()
}

lazy_static! {
    #[doc(hidden)]
    static ref SCRIPT_ENGINE: rhai::Engine = {
        use rhai::{Engine, RegisterFn};
        let mut engine = Engine::new();
        engine.register_fn("to_float", to_float);
        engine.register_fn("to_int", to_int);
        engine
    };
}

/////////////////////////////////////////////////////////
// Output Data Trait

/// Trait that must be implemented by type representing output data 
/// provided by particular modules APIs.
/// 
#[async_trait]
pub trait OutputData {
    /// Adds new data to already collected ones.
    /// Typically stores data in some internal buffer until they are saved in file.
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()>;
    
    /// Save collected data to file and clears collected data buffer
    async fn save(&mut self, path: &str) -> Result<()>;
}

/////////////////////////////////////////////////////////
// Process Json

/// Function which recursively process input JSON value when filters are not applied.
/// It tries to convert string representation of numbers into JSON native numbers.
/// 
pub fn process_json(json_val: serde_json::Value) -> serde_json::Value {
    use serde_json::Value;
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

/// Type which represents filter function.
pub type FilterFun = Box<dyn Fn(&str, &serde_json::Value) -> bool>;

/// Function which recursively process input JSON value when filters are applied.
/// It tries to convert string representation of numbers into JSON native numbers and
/// on Object items runs filter function if it is configured for given key.
/// 
pub fn process_json_with_filters(key:&str, json_val: serde_json::Value, filters:&HashMap<String, FilterFun>) -> serde_json::Value {
    use serde_json::Value;
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

/// Function which transforms configuration of filters into actual filters hash map ready to be called by `process_json_with_filters` function.
/// It determines what kind of filters are implemented in application.
/// 
pub fn create_filters(filters: &Option<ConfigFilters>) -> Result<HashMap<String, FilterFun>> {
    let engine = &*SCRIPT_ENGINE;
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

/////////////////////////////////////////////////////////
// Other Helpers

/// Function which transforms meta-characters contained in output file specification
/// into actual values based on sequential counter or current date-time.
/// 
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

/// Task handler function.
/// This function determines which modules are implemented, and based on configured module:
/// - creates respective task object which implements actix Handler interface 
/// - sends task configuration to this handler, which causes task object to perform respective actions
/// 
pub async fn handle_task(task: ConfigTask) {
    let task_mod = task.Module.clone();
    let task_api = task.Api.clone();
    match task_mod.as_ref() {
        "GenericJson" => {
            let handler = super::generic_json::TaskRunner::new().start();
            match handler.send(task).await {
                Err(e) => eprintln!("Task '{}/{}': Dispatch error: {}", task_mod, task_api, e),
                Ok(Err(e)) => eprintln!("Task '{}/{}': Run error: {}", task_mod, task_api, e),
                _ => {}
            }
        },
        #[cfg(feature = "bitbay")]
        "BitBay" => {
            let handler = super::bitbay::TaskRunner::new().start();
            match handler.send(task).await {
                Err(e) => eprintln!("Task '{}/{}': Dispatch error: {}", task_mod, task_api, e),
                Ok(Err(e)) => eprintln!("Task '{}/{}': Run error: {}", task_mod, task_api, e),
                _ => {}
            }
        },
        _ => eprintln!("Unsupported Module: {}", task_mod)
    }
}
