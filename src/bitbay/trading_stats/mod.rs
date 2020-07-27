/*!
 * Implementation of "trading/stats" API from "BitBay" module.
 * 
 * This module is able to download data in JSON format from _https://api.bitbay.net/rest/trading/stats_ and save in file.
 */
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::SystemTime;

use anyhow::{Result, Error};

use serde::{Deserialize, Serialize};

use crate::common::{OutputData, FilterFun, process_json_with_filters, process_json};


pub mod json;

#[cfg(feature = "csv_out")]
pub mod csv;

#[cfg(feature = "pb_out")]
pub mod pb;

/////////////////////////////////////////////////////////
// Input

/// Record of input data for this module.
#[derive(Deserialize, Debug)]
pub struct StatIn {
    h: Option<f32>,
    l: Option<f32>,
    m: String,
    r24h: f32,
    v: f32,
}

/// Input data for this module.
pub type StatsAllIn = Vec<StatIn>;

/// Function which downloads and returns chunk of input data.
/// 
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<StatsAllIn> {
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let json:serde_json::Value = resp.json().await?;
        let json = if filters.is_empty() {
            process_json(json)
        } else {
            process_json_with_filters("/", json, filters)
        };
        let items = json.get("items").ok_or(anyhow!("No items key in response"))?;
        let items = items.as_object().ok_or(anyhow!("items key in response is not JSON object"))?;
        let mut stats = Vec::<StatIn>::new();
        for it in items.values() {
            stats.push(serde_json::from_value(it.clone())?);
        }
        Ok(stats)
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
            let mut json = json::StatsAllOut::new();
            json.print_pretty = format == "json_pretty";
            Some(Box::new(json))
        },
        #[cfg(feature = "csv_out")]
        "csv" => Some(Box::new(csv::StatsAllOut::new())),
        #[cfg(feature = "pb_out")]
        "pb" => Some(Box::new(pb::StatsAllOut::new())),
        #[cfg(feature = "pb_out")]
        "pb_proto" => Some(Box::new(pb::ProtoOut)),
        _ => None
    }
}

/// Record of output object.
/// Output object depends on output format and is defined in respective sub-module.
#[derive(Serialize, Debug)]
struct StatOut {
    timestamp: u64,
    market1: String,
    market2: String,
    vol: f32,
    hi: f32,
    lo: f32,
    r24h: f32,
}

impl TryFrom<&StatIn> for StatOut {
    type Error = Error;

    fn try_from(sin: &StatIn) -> std::result::Result<Self, Self::Error> {
        let market: Vec<_> = sin.m.split('-').collect();
        if market.len() < 2 { bail!("market name does not contain '-' separator"); }
        Ok(
            StatOut {
                timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                market1: market[0].to_owned(),
                market2: market[1].to_owned(),
                vol: sin.v,
                hi: sin.h.unwrap_or_default(),
                lo: sin.l.unwrap_or_default(),
                r24h: sin.r24h,
            }
        )
    }
}

