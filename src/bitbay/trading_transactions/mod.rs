/*!
 * Implementation of "trading/transactions" API from "BitBay" module.
 * 
 * This module is able to download data in JSON format from _https://api.bitbay.net/rest/trading/transactions and save in file in several output formats.
 */
use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::{Result, Error};

use serde::{Deserialize, Serialize};

use crate::common::{OutputData, FilterFun, process_json_with_filters, process_json};


pub mod json;

#[cfg(feature = "out_csv")]
pub mod csv;

#[cfg(feature = "out_pb")]
pub mod pb;

/////////////////////////////////////////////////////////
// Input

/// Buy/Sell flag
#[derive(Deserialize, Debug)]
pub enum BuySell{
    Buy,
    Sell
}

/// Record of input data for this module.
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct TransactionIn {
    a: f32,
    id: String,
    r: f32,
    t: u64,
    ty: BuySell,
}

/// Input data for this module.
//pub type TransactionsIn = Vec<TransactionIn>;
#[derive(Deserialize, Debug)]
pub struct TransactionsIn {
    items: Vec<TransactionIn>,
}

/// Function which downloads and returns chunk of input data.
/// 
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<TransactionsIn> {
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
/*
pub async fn get_data(url: &str, filters:&HashMap<String, FilterFun>) -> Result<TransactionsIn> {
    let resp = reqwest::get(url).await?;
    if resp.status().is_success() {
        let json:serde_json::Value = resp.json().await?;
        let mut json = if filters.is_empty() {
            process_json(json)
        } else {
            process_json_with_filters("/", json, filters)
        };
        let mut trans = Vec::<TransactionIn>::new();
        if let Some(items) = json.get_mut("items") {
            let items = items.as_object_mut().ok_or(anyhow!("'items' key in response is not JSON object"))?;
            for it in items.values_mut() {
                trans.push(serde_json::from_value(it.take())?);
            }
        } else {
            bail!("Unsupported input JSON: no 'items' key in response")
        }
        Ok(trans)
    } else {
        bail!(resp.status())
    }
}
*/
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
            let mut json = json::TransactionsOut::new();
            json.print_pretty = format == "json_pretty";
            Some(Box::new(json))
        },
        #[cfg(feature = "out_csv")]
        "csv" => Some(Box::new(csv::TransactionsOut::new())),
        #[cfg(feature = "out_pb")]
        "pb" => Some(Box::new(pb::Transactions::new())),
        #[cfg(feature = "out_pb")]
        "pb_proto" => Some(Box::new(pb::ProtoOut)),
        _ => None
    }
}

/// Record of output object.
/// Output object depends on output format and is defined in respective sub-module.
#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct TransactionOut {
    amt: f32,
    id: String,
    rate: f32,
    timestamp: u64,
    sell_flg: u8, // 0 - buy, 1 - sell
}

impl TryFrom<&TransactionIn> for TransactionOut {
    type Error = Error;

    fn try_from(tin: &TransactionIn) -> std::result::Result<Self, Self::Error> {
        Ok(
            TransactionOut {
                amt: tin.a,
                id: tin.id.clone(),
                rate: tin.r,
                timestamp: tin.t,
                sell_flg: if let BuySell::Buy = tin.ty { 0 } else { 1 },
            }
        )
    }
}

