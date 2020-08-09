/*!
 * Implementation of "GenericJson" module.
 * 
 * This module is able to download data in JSON format and save as JSON file.
 * Downloaded data can be accumulated and saved as array of JSON values.
 */
use std::any::Any;
use std::collections::HashMap;

use actix::prelude::*;

use async_trait::async_trait;

use anyhow::Result;

use serde_json::Value;

use crate::common::{
    ConfigTask, create_filters, resolve_value,
    FilterFun, process_json_with_filters, process_json,
    OutputData
};

/////////////////////////////////////////////////////////
// Task runner

/// Empty struct that implements actix Actor and Handler traits.
pub struct TaskRunner;

impl TaskRunner {
    pub fn new() -> Self { TaskRunner{} }
}

impl Actor for TaskRunner {
    type Context = Context<Self>;
}

impl Handler<ConfigTask> for TaskRunner {
    type Result = ResponseFuture<Result<()>>;
    
    /// Based on passed task configuration this function gets data and saves them in the file.
    fn handle(&mut self, task: ConfigTask, _ctx: &mut Context<Self>) -> Self::Result {
        use std::borrow::Cow;
        debug!("Scheduling task: {:?}", &task);
        Box::pin(async move {
            let mut url = match task.Url {
                Some(url) => url,
                None => "".to_owned(),
            };
            if !url.is_empty() && !url.ends_with('/') {
                url.push_str("/");
            }
            url.push_str(&task.Api);
            let filters = create_filters(&task.Filters)?;
            let mut run_cnt = 1usize;
            let mut this_cnt = 1usize;
            let mut file_cnt = 1usize;
            let mut data:Box<dyn Any>;
            let mut data_out = None;
            if let Some(path_params) = task.PathParams.as_ref() {
                if !path_params.starts_with('/') {
                    url.push_str("/");
                }
                url.push_str(path_params);
            }
            loop {
                let mut url_full = Cow::from(&url);
                if let Some(query_params) = task.QueryParams.as_ref() {
                    url_full.to_mut().push_str("?");
                    for (k, v) in query_params.iter() {
                        if !url_full.ends_with('?') {
                            url_full.to_mut().push_str("&");
                        }
                        url_full.to_mut().push_str(k);
                        url_full.to_mut().push_str("=");
                        url_full.to_mut().push_str(&resolve_value(v, run_cnt, file_cnt));
                    }
                }
                debug!("Sending request: {}", &url_full);
                
                data = Box::new(get_data(&url_full, &filters).await?);
                if data_out.is_none() {
                    data_out = output_data_for(&task.Format);
                }
                match data_out {
                    None => bail!("Not supported Format: {} for Api: {}", task.Format, task.Api),
                    Some(ref mut data_out) => data_out.add_data(data)?,
                }
                if let Some(stop_after) = task.StopAfter {
                    if run_cnt >= stop_after { break; }
                    run_cnt = run_cnt.wrapping_add(1);
                }
                if let Some(new_after) = task.NewFileAfter {
                    if this_cnt >= new_after { 
                        if let Some(ref mut data_out) = data_out {
                            let filename = resolve_value(&task.OutPathMask, run_cnt, file_cnt);
                            debug!("Saving data to file: {}", &filename);
                            data_out.save(&filename).await?;
                        }
                        this_cnt = 1usize;
                        file_cnt = file_cnt.wrapping_add(1);
                    } else {
                        this_cnt = this_cnt.wrapping_add(1);
                    }
                }
                if let Some(file_cnt_max) = task.CounterMax {
                    if file_cnt > file_cnt_max {
                        file_cnt = 1;
                    }
                }
                if let Some(frequency) = task.Frequency {
                    actix::clock::delay_for(std::time::Duration::from_secs(frequency)).await;
                    // std::thread::sleep(std::time::Duration::from_secs(frequency));
                }
            }
            let filename = resolve_value(&task.OutPathMask, run_cnt, file_cnt);
            data_out.unwrap().save(&filename).await?;
            Ok(())
        })
    }
}

/////////////////////////////////////////////////////////
// Input

/// Input data for this module - it is JSON::Value object.
type InputChunk = Value;

/// Function which downloads and returns chunk of input data.
/// 
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<InputChunk> {
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let json:Value = resp.json().await?;
        let json = if filters.is_empty() {
            process_json(json)
        } else {
            process_json_with_filters("/", json, filters)
        };
        Ok(json)
    } else {
        bail!(resp.status())
    }
}

/////////////////////////////////////////////////////////
// Output

/// Output object factory.
/// 
/// Function returns output object appropriate for given format or `None` if format is not supported.
/// Currently supported formats:
/// - `json` (formatted in compact way - for computers)
/// - `json_pretty` (formatted in readable way - for humans)
pub fn output_data_for(format: &str) -> Option<Box<dyn OutputData>> {
    match format {
        "json" | "json_pretty" => {
            let mut json = OutputDataImpl::new();
            json.print_pretty = format == "json_pretty";
            Some(Box::new(json))
        },
        _ => None
    }
}

/// Output object implementation.
pub struct OutputDataImpl {
    data: Value,
    print_pretty: bool,
}

impl OutputDataImpl {
    pub fn new() -> Self {
        OutputDataImpl { data: Value::Array(vec![]), print_pretty: false }
    }
}

#[async_trait]
impl OutputData for OutputDataImpl {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<InputChunk>() {
            if let Value::Array(ref mut arr) = self.data {
                arr.push(*data);
                Ok(())
            } else {
                bail!("Logical program error: self.data should be of serde_json::Value::Array type")
            }
        } else {
            bail!("Logical program error: data should be of generic_json::InputChunk type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        let js = if self.print_pretty { format!("{:#}", self.data) } else { format!("{}", self.data) };
        file.write_all(js.as_bytes()).await?;
        if let Value::Array(ref mut arr) = self.data {
            arr.clear();
        }
        Ok(())
    }
}