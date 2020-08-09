/*!
 * Implementation of "trading/candle/history" API from "BitBay" module.
 * 
 * This module is able to download data in JSON format from _https://api.bitbay.net/rest/trading/candle/history and save in file in several output formats.
 */
use std::collections::HashMap;
// use std::convert::TryFrom;
use std::convert::From;

use anyhow::Result;
// use anyhow::{Result, Error};

use serde::{Deserialize, Serialize};

use crate::common::{OutputData, FilterFun, process_json_with_filters, process_json};


pub mod json;

#[cfg(feature = "out_csv")]
pub mod csv;

#[cfg(feature = "out_pb")]
pub mod pb;

/////////////////////////////////////////////////////////
// Input

/// Individual candle item.
#[derive(Deserialize, Debug)]
pub struct CandleIn {
    o: f32,
    c: f32,
    h: f32,
    l: f32,
    v: f32,
    co: u32,
}

/// Record of input data for this module.
/// A pair: timestamp, CandleIn
#[derive(Deserialize, Debug)]
pub struct CandleRecIn(u64, CandleIn);

/// Input data for this module.
//pub type TransactionsIn = Vec<TransactionIn>;
#[derive(Deserialize, Debug)]
pub struct CandlesIn {
    items: Vec<CandleRecIn>,
}

/// Function which downloads and returns chunk of input data.
/// 
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<CandlesIn> {
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
            let mut json = json::CandlesOut::new();
            json.print_pretty = format == "json_pretty";
            Some(Box::new(json))
        },
        #[cfg(feature = "out_csv")]
        "csv" => Some(Box::new(csv::CandlesOut::new())),
        #[cfg(feature = "out_pb")]
        "pb" => Some(Box::new(pb::Candles::new())),
        #[cfg(feature = "out_pb")]
        "pb_proto" => Some(Box::new(pb::ProtoOut)),
        _ => None
    }
}

/// Record of output object.
/// Output object depends on output format and is defined in respective sub-module.
#[derive(Serialize, Debug)]
pub struct CandleOut {
    timestamp: u64,
    open: f32,
    close: f32,
    high: f32,
    low: f32,
    vol: f32,
    count: u32,
}

impl From<&CandleRecIn> for CandleOut {
    fn from(cin: &CandleRecIn) -> Self {
        CandleOut {
            timestamp: cin.0,
            open: cin.1.o,
            close: cin.1.c,
            high: cin.1.h,
            low: cin.1.l,
            vol: cin.1.v,
            count: cin.1.co,
        }
    }
}

