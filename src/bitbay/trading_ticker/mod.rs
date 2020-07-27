/*!
 * Implementation of "trading/ticker" API from "BitBay" module.
 * 
 * This module is able to download data in JSON format from _https://api.bitbay.net/rest/trading/ticker_ and save in file.
 */
use std::collections::HashMap;
use std::convert::TryFrom;

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
    lowestAsk: f32,
    previousRate: f32,
    rate: f32,
    highestBid: f32,
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
        let json = if filters.is_empty() {
            process_json(json)
        } else {
            process_json_with_filters("/", json, filters)
        };
        let items = json.get("items").ok_or(anyhow!("No items key in response"))?;
        let items = items.as_object().ok_or(anyhow!("items key in response is not JSON object"))?;
        let mut ticks = Vec::<TickIn>::new();
        for it in items.values() {
            ticks.push(serde_json::from_value(it.clone())?);
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
        #[cfg(feature = "csv_out")]
        "csv" => Some(Box::new(csv::TickAllOut::new())),
        #[cfg(feature = "pb_out")]
        "pb" => Some(Box::new(pb::TickAllOut::new())),
        #[cfg(feature = "pb_out")]
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
    lowestAsk: f32,
    previousRate: f32,
    rate: f32,
    highestBid: f32,
    scale1: u16,
    currency1: String,
    minOffer1: f32,
    scale2: u16,
    currency2: String,
    minOffer2: f32,
}

impl TryFrom<&TickIn> for TickOut {
    type Error = Error;

    fn try_from(tin: &TickIn) -> std::result::Result<Self, Self::Error> {
        Ok(
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
        )
    }
}

