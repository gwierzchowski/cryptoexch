/*!
 * Implementation of "trading/orderbook" API from "BitBay" module.
 * 
 * This module is able to download data in JSON format from _https://api.bitbay.net/rest/trading/orderbook and save in file.
 */
use std::collections::HashMap;

use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::common::{OutputData, FilterFun, process_json_with_filters, process_json};


pub mod json;

#[cfg(feature = "out_csv")]
pub mod csv;

#[cfg(feature = "out_pb")]
pub mod pb;

/////////////////////////////////////////////////////////
// Input

/// Record of input data for this module.
#[allow(non_snake_case)]
#[derive(Serialize ,Deserialize, Debug)]
pub struct OrderIn {
    co: u16,
    ra: f32,
    ca: f32,
    pa: f32,
    sa: f32,
}

/// Input data for this module.
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderbookIn {
    buy:  Vec<OrderIn>,
    sell: Vec<OrderIn>,
    seqNo: u64,
    timestamp: u64,
}

/// Function which downloads and returns chunk of input data.
/// 
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<OrderbookIn> {
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let json:serde_json::Value = resp.json().await?;
        let json = if filters.is_empty() {
            process_json(json)
        } else {
            process_json_with_filters("/", json, filters)
        };
        Ok(serde_json::from_value(json)?)
    } else {
        bail!(resp.status())
    }
}

/////////////////////////////////////////////////////////
// Output - common

/// Output object factory.
/// 
/// Function returns output object appropriate for given format or `None` if format is not supported.
/// Currently supported formats:
/// - `json` (formatted in compact way - for computers)
/// - `json_pretty` (formatted in readable way - for humans)
/// - `csv` (with `,` as separator)
/// - `pb` - Google Protocol Buffers format
/// - `pb_proto` - saves definition file (.proto) for `pb` format
pub fn output_data_for(format: &str) -> Option<Box<dyn OutputData>> {
    match format {
        "json" | "json_pretty" => {
            let mut json = json::OrderbooksOut::new();
            json.print_pretty = format == "json_pretty";
            Some(Box::new(json))
        },
        #[cfg(feature = "out_csv")]
        "csv" => Some(Box::new(csv::OrderbooksOut::new())),
        #[cfg(feature = "out_pb")]
        "pb" => Some(Box::new(pb::OrderBooks::new())),
        #[cfg(feature = "out_pb")]
        "pb_proto" => Some(Box::new(pb::ProtoOut)),
        _ => None
    }
}

