use std::any::Any;
use std::collections::HashMap;

use actix::prelude::*;

use async_trait::async_trait;

use anyhow::Result;

use serde_json::Value;

use crate::common::{
    ConfigTask, create_filters, resolve_filename,
    FilterFun, process_json_with_filters, process_json,
    OutputData
};

//////////////////////////////////////////////////////////
/// Task runner

pub struct TaskRunner;

impl TaskRunner {
    pub fn new() -> Self { TaskRunner{} }
}

impl Actor for TaskRunner {
    type Context = Context<Self>;
}

impl Handler<ConfigTask> for TaskRunner {
   type Result = ResponseFuture<Result<()>>;
    
    fn handle(&mut self, task: ConfigTask, _ctx: &mut Context<Self>) -> Self::Result {
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
            loop {
                data = Box::new(get_data(&url, &filters).await?);
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
                            let filename = resolve_filename(&task.OutPathMask, file_cnt);
                            data_out.save(&filename).await?;
                        }
                        this_cnt = 1usize;
                        file_cnt = file_cnt.wrapping_add(1);
                    } else {
                        this_cnt = this_cnt.wrapping_add(1);
                    }
                }
                if let Some(frequency) = task.Frequency {
                    actix::clock::delay_for(std::time::Duration::from_secs(frequency)).await;
                    // std::thread::sleep(std::time::Duration::from_secs(frequency));
                }
            }
            let filename = resolve_filename(&task.OutPathMask, file_cnt);
            data_out.unwrap().save(&filename).await?;
            Ok(())
        })
    }
}

//////////////////////////////////////////////////////////
/// Input

type InputChunk = Value;

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

//////////////////////////////////////////////////////////
/// Output

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