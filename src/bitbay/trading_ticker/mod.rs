/*!
 * Implementation of "trading/ticker" API from "BitBay" module.
 * 
 * This module is able to download data in JSON format from _https://api.bitbay.net/rest/trading/ticker_ and save in file in several output formats.
 */
use std::collections::HashMap;
use std::convert::From;

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

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TickInCurrency {
    scale: u16,
    currency: String,
    minOffer: f32,
}

#[derive(Deserialize, Debug)]
pub struct TickInMarket {
    code: String,
    first: TickInCurrency,
    second: TickInCurrency,
}

/// Record of input data for this module.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TickIn {
    lowestAsk: Option<f32>,
    previousRate: f32,
    rate: f32,
    highestBid: Option<f32>,
    time: u64,
    market: TickInMarket,
}

/// Input data for this module.
pub type TickAllIn = Vec<TickIn>;

/// Function which downloads and returns chunk of input data.
/// 
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<TickAllIn> {
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let json:serde_json::Value = resp.json().await?;
        let mut json = if filters.is_empty() {
            process_json(json)
        } else {
            process_json_with_filters("/", json, filters)
        };
        let mut ticks = Vec::<TickIn>::new();
        if let Some(items) = json.get_mut("items") {
            let items = items.as_object_mut().ok_or(anyhow!("'items' key in response is not JSON object"))?;
            for it in items.values_mut() {
                ticks.push(serde_json::from_value(it.take())?);
            }
        } else if let Some(item) = json.get_mut("ticker") {
            ticks.push(serde_json::from_value(item.take())?);
        } else {
            bail!("Unsupported input JSON: no 'items' nor 'ticker' keys in response")
        }
        Ok(ticks)
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
            let mut json = json::TickAllOut::new();
            json.print_pretty = format == "json_pretty";
            Some(Box::new(json))
        },
        #[cfg(feature = "out_csv")]
        "csv" => Some(Box::new(csv::TickAllOut::new())),
        #[cfg(feature = "out_pb")]
        "pb" => Some(Box::new(pb::TickAllOut::new())),
        #[cfg(feature = "out_pb")]
        "pb_proto" => Some(Box::new(pb::ProtoOut)),
        _ => None
    }
}

/// Record of output object.
/// Output object depends on output format and is defined in respective sub-module.
#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
struct TickOut {
    time: u64,
    lowestAsk: Option<f32>,
    previousRate: f32,
    rate: f32,
    highestBid: Option<f32>,
    scale1: u16,
    currency1: String,
    minOffer1: f32,
    scale2: u16,
    currency2: String,
    minOffer2: f32,
}

impl From<&TickIn> for TickOut {
    fn from(tin: &TickIn) -> Self {
        TickOut {
            time: tin.time,
            lowestAsk: tin.lowestAsk,
            previousRate: tin.previousRate,
            rate: tin.rate,
            highestBid: tin.highestBid,
            scale1: tin.market.first.scale,
            currency1: tin.market.first.currency.clone(),
            minOffer1: tin.market.first.minOffer,
            scale2: tin.market.second.scale,
            currency2: tin.market.second.currency.clone(),
            minOffer2: tin.market.second.minOffer,
        }
    }
}

