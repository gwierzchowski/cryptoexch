use std::any::Any;
use std::str::FromStr;

use actix::prelude::*;

use async_trait::async_trait;

use anyhow::Result;

use lazy_static;

use super::common::{ConfigTask, create_filters, resolve_filename};

mod trading_ticker;
mod trading_stats;

//////////////////////////////////////////////////////////
/// Task runner

const URL:&str = "https://api.bitbay.net/rest/";

fn to_float(s: rhai::ImmutableString) -> rhai::FLOAT {
    rhai::FLOAT::from_str(s.as_str()).unwrap_or_default()
}

fn to_int(s: rhai::ImmutableString) -> rhai::INT {
    rhai::INT::from_str(s.as_str()).unwrap_or_default()
}

lazy_static! {
    static ref SCRIPT_ENGINE: rhai::Engine = {
        use rhai::{Engine, RegisterFn};
        let mut engine = Engine::new();
        engine.register_fn("to_float", to_float);
        engine.register_fn("to_int", to_int);
        engine
    };
}

pub struct TaskRunner {
}

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
            let url = URL.to_owned() + &task.Api;
            let filters = create_filters(&task.Filters, &*SCRIPT_ENGINE)?;
            let mut run_cnt = 1usize;
            let mut this_cnt = 1usize;
            let mut file_cnt = 1usize;
            let mut data:Box<dyn Any>;
            let mut data_out = None;
            loop {
                match task.Api.as_ref() {
                    "trading/ticker" => {
                        data = Box::new(trading_ticker::get_data(&url, &filters).await?);
                        if data_out.is_none() {
                            data_out = trading_ticker::output_data_for(&task.Format);
                        }
                    },
                    "trading/stats" => {
                        data = Box::new(trading_stats::get_data(&url, &filters).await?);
                        if data_out.is_none() {
                            data_out = trading_stats::output_data_for(&task.Format);
                        }
                    },
                    _ => bail!("Not supported Tasks:Api: {}", task.Api)
                };
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

#[async_trait]
pub trait OutputData {
    /// Adds new data to already collected ones
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()>;
    
    /// Save collected data to file and clears collected data buffer
    // fn save(&mut self, path: &str) -> Result<()>;
    async fn save(&mut self, path: &str) -> Result<()>;
}

// Following handle would be more elegant, but it can not be compiled (rustc 1.43):
// 80 |                 let data = get_data().await?;
//    |                            ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
/*
fn handle(&mut self, task: ConfigTask, _ctx: &mut Context<Self>) -> Self::Result {
    Box::pin(async move {
        let url = URL.to_owned() + &task.Api;
        let filters = create_filters(&task.Filters, &*SCRIPT_ENGINE)?;
        let (get_data, mut data_out) = match task.Api.as_ref() {
            "trading/ticker" => (Box::new(|| async {
                    let data = trading_ticker::get_data(&url, &filters).await?;
                    Ok::<_,anyhow::Error>(Box::new(data) as Box<dyn Any>)
                }) as Box<dyn Fn() -> dyn Future<Output=Result<Box<dyn Any>>>>, 
                trading_ticker::output_data_for(&task.Format)
            ),
            "trading/stats" => (Box::new(|| async {
                    let data = trading_stats::get_data(&url, &filters).await?;
                    Ok::<_,anyhow::Error>(Box::new(data) as Box<dyn Any>)
                }) as Box<dyn Fn() -> dyn Future<Output=Result<Box<dyn Any>>>>, 
                trading_stats::output_data_for(&task.Format)
            ),
            _ => bail!("Not supported Tasks:Api: {}", task.Api)
        };
        let mut data_out = match data_out {
            None => bail!("Not supported Format: {} for Api: {}", task.Format, task.Api),
            Some(d) => d
        };
        let mut run_cnt = 1usize;
        loop {
            let data = get_data().await?;
            data_out.add_data(data)?;
            std::thread::sleep(std::time::Duration::from_secs(2)); // TODO: Change to respective async function
            if let Some(stop_after) = task.StopAfter {
                if run_cnt >= stop_after { break; }
                run_cnt += 1; // TODO: handle overflow
            }
        }
        data_out.save(&task.OutPathMask)?;
        Ok(())
    })
}
*/
