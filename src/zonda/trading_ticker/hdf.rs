/*!
 * Implementation of CSV output format of trading/ticker" API from "Zonda" module.
 */
use std::any::Any;
use std::convert::{TryFrom, TryInto};

use anyhow::Result;

use async_trait::async_trait;

use hdf5::types::FixedAscii;
// use hdf5::types::{VarLenArray, FixedAscii};

const CURR_NAME_LEN:usize = 3;

/// Record of output object - spefcific for HDF bacause of special string types treatment.
/// Output object depends on output format and is defined in respective sub-module.
#[allow(non_snake_case)]
#[derive(hdf5::H5Type)]
#[repr(C)]
struct TickOut {
    time: u64,
    lowestAsk: f32,
    // lowestAsk: VarLenArray<f32>,
    previousRate: f32,
    rate: f32,
    highestBid: f32,
    // highestBid: VarLenArray<f32>,
    scale1: u16,
    currency1: FixedAscii<CURR_NAME_LEN>,
    // currency1: VarLenAscii,
    minOffer1: f32,
    scale2: u16,
    currency2: FixedAscii<CURR_NAME_LEN>,
    // currency2: VarLenAscii,
    minOffer2: f32,
}

#[allow(non_snake_case)]
impl TryFrom<&super::TickIn> for TickOut {
    type Error = anyhow::Error;

    fn try_from(tin: &super::TickIn) -> Result<Self, Self::Error> {
        // Truncate and pad to exactly CURR_NAME_LEN characters
        let currency1 = FixedAscii::from_ascii(&[
            tin.market.first.currency.as_str(), 
            " ".repeat(CURR_NAME_LEN).as_str()]
            .join("")
            .as_bytes()[..CURR_NAME_LEN])?;
        let currency2 = FixedAscii::from_ascii(&[
            tin.market.second.currency.as_str(), 
            " ".repeat(CURR_NAME_LEN).as_str()]
            .join("")
            .as_bytes()[..CURR_NAME_LEN])?;
        let lowestAsk = match tin.lowestAsk {
            Some(la) => la,
            None => f32::default()
        };
        let highestBid = match tin.highestBid {
            Some(hb) => hb,
            None => f32::default()
        };
        Ok(TickOut {
            time: tin.time,
            lowestAsk: lowestAsk,
            previousRate: tin.previousRate,
            rate: tin.rate,
            highestBid: highestBid,
            scale1: tin.market.first.scale,
            currency1: currency1,
            minOffer1: tin.market.first.minOffer,
            scale2: tin.market.second.scale,
            currency2: currency2,
            minOffer2: tin.market.second.minOffer,
        })
    }
}

/// Output object implementation.
pub struct TickAllOut {
    ticks: Vec<TickOut>,
}

// impl TickAllOut {
//     pub fn new() -> Self {
//         TickAllOut { ticks: Vec::new() }
//     }
// }

impl TryFrom<&super::TickAllIn> for TickAllOut {
    type Error = anyhow::Error;

    fn try_from(tin: &super::TickAllIn) -> Result<Self, Self::Error> {
        let mut vout = Vec::with_capacity(tin.len());
        for el in tin {
            vout.push(el.try_into()?);
        }
        Ok(TickAllOut { ticks:vout })
    }
}

#[async_trait]
impl crate::common::OutputData for TickAllOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TickAllIn>() {
            for ref d in *data {
                self.ticks.push(d.try_into()?);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_ticker::TickAllIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = hdf5::File::create(path)?;
        let ticks = file.new_dataset::<TickOut>().create("trading_ticker")?;
        ticks.write(&self.ticks)?;
        self.ticks.clear();
        Ok(())
        // std::future::ready(Ok(()))
    }

    // async fn save(&mut self, path: &str) -> Result<()> {
    //     let file = tokio::fs::File::create(path).await?;
    //     let mut wri = csv_async::AsyncSerializer::from_writer(file); 
    //     for rec in &self.ticks {
    //         wri.serialize(rec).await?;
    //     }
    //     self.ticks.clear();
    //     Ok(())
    //}
}