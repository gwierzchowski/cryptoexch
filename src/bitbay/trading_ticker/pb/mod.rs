use std::any::Any;
use std::convert::{TryFrom, TryInto};

use anyhow::{Result, Error};

use async_trait::async_trait;

use protobuf::{CodedOutputStream, Message};

mod trading_ticker;

type TickOut        = trading_ticker::TradingTickAll_TradingTick;
pub type TickAllOut = trading_ticker::TradingTickAll;

impl TryFrom<&super::TickAllIn> for TickAllOut {
    type Error = Error;

    fn try_from(tin: &super::TickAllIn) -> std::result::Result<Self, Self::Error> {
        let mut ticks_out = TickAllOut::new();
        for ticks in tin {
            ticks_out.mut_ticks().push(ticks.try_into()?);
        }
        Ok(ticks_out)
    }
}

impl TryFrom<&super::TickIn> for TickOut {
    type Error = Error;

    fn try_from(tin: &super::TickIn) -> std::result::Result<Self, Self::Error> {
        let mut rec_out = TickOut::new();
        rec_out.set_time(tin.time);
        rec_out.set_lowestAsk(tin.lowestAsk);
        rec_out.set_previousRate(tin.previousRate);
        rec_out.set_rate(tin.rate);
        rec_out.set_highestBid(tin.highestBid);
        rec_out.set_scale1(tin.market.first.scale as u32);
        rec_out.set_currency1(tin.market.first.currency.clone());
        rec_out.set_minOffer1(tin.market.first.minOffer);
        rec_out.set_scale2(tin.market.second.scale as u32);
        rec_out.set_currency2(tin.market.second.currency.clone());
        rec_out.set_minOffer2(tin.market.second.minOffer);
        Ok(rec_out)
    }
}

#[async_trait]
impl crate::common::OutputData for TickAllOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TickAllIn>() {
            for ref d in *data {
                self.mut_ticks().push(d.try_into()?);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_ticker::TickAllIn type")
        }
    }

    // fn save(&mut self, path: &str) -> Result<()> {
        // use std::fs::File;
        // use std::io::prelude::*;
    //     let mut file = File::create(path)?;
    //     let mut writer = CodedOutputStream::new(&mut file);
    //     self.write_to(&mut writer)?;
    //     writer.flush()?;
    //     self.mut_ticks().clear();
    //     Ok(())
    // }
    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut file = file.into_std().await;
        let mut writer = CodedOutputStream::new(&mut file); // TODO: Check if this can be done in async way
        self.write_to(&mut writer)?;
        writer.flush()?;
        self.mut_ticks().clear();
        Ok(())
    }
}

pub struct ProtoOut;

#[async_trait]
impl crate::common::OutputData for ProtoOut {
    fn add_data(&mut self, _data: Box<dyn Any>) -> Result<()> {
        Ok(())
    }

    // fn save(&mut self, path: &str) -> Result<()> {
    //     let mut file = File::create(path)?;
    //     file.write_all(include_bytes!("trading_ticker.proto"))?;
    //     Ok(())
    // }
    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(include_bytes!("trading_ticker.proto")).await?;
        Ok(())
    }
}
