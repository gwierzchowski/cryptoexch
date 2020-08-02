/*!
 * Implementation of "BitBay" module.
 * 
 * This module is able to download data in JSON format from [BitBay](https://bitbay.net) service and save in file.
 * List of supported APIs is as described in sub-modules list.
 */
use std::any::Any;

use actix::prelude::*;

use anyhow::Result;

use super::common::{ConfigTask, create_filters, resolve_filename};

mod trading_ticker;
mod trading_stats;
mod trading_orderbook;

/////////////////////////////////////////////////////////
// Task runner

const URL:&str = "https://api.bitbay.net/rest/";

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
        Box::pin(async move {
            let mut url = match task.Url {
                Some(url) => url,
                None => URL.to_owned(),
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
                    "trading/orderbook" | "trading/orderbook-limited" => {
                        data = Box::new(trading_orderbook::get_data(&url, &filters).await?);
                        if data_out.is_none() {
                            data_out = trading_orderbook::output_data_for(&task.Format);
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
            let filename = resolve_filename(&task.OutPathMask, file_cnt);
            data_out.unwrap().save(&filename).await?;
            Ok(())
        })
    }
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
