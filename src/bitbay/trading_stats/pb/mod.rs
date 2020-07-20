use std::any::Any;
use std::convert::{TryFrom, TryInto};
use std::time::SystemTime;

use anyhow::{Result, Error};

use async_trait::async_trait;

use protobuf::{CodedOutputStream, Message};

mod trading_stats;

type StatOut         = trading_stats::TradingStatsAll_TradingStat;
pub type StatsAllOut = trading_stats::TradingStatsAll;

impl TryFrom<&super::StatsAllIn> for StatsAllOut {
    type Error = Error;

    fn try_from(sin: &super::StatsAllIn) -> std::result::Result<Self, Self::Error> {
        let mut stats_out = StatsAllOut::new();
        for stat in sin {
            stats_out.mut_stats().push(stat.try_into()?);
        }
        Ok(stats_out)
    }
}

impl TryFrom<&super::StatIn> for StatOut {
    type Error = Error;

    fn try_from(sin: &super::StatIn) -> std::result::Result<Self, Self::Error> {
        let market: Vec<_> = sin.m.split('-').collect();
        if market.len() < 2 { bail!("market name does not contain '-' separator"); }
        let mut rec_out = StatOut::new();
        rec_out.set_timestamp(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs());
        rec_out.set_market1(market[0].to_owned());
        rec_out.set_market2(market[1].to_owned());
        rec_out.set_vol(sin.v);
        if let Some(h) = sin.h {
            rec_out.set_hi(h);
        }
        if let Some(l) = sin.l {
            rec_out.set_lo(l);
        }
        rec_out.set_r24h(sin.r24h);
        Ok(rec_out)
    }
}

#[async_trait]
impl super::super::OutputData for StatsAllOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::StatsAllIn>() {
            for ref d in *data {
                self.mut_stats().push(d.try_into()?);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_stats::StatsAllIn type")
        }
    }

    // fn save(&mut self, path: &str) -> Result<()> {
    //     use std::io::prelude::*;
    //     let mut file = std::fs::File::create(path)?;
    //     let mut writer = CodedOutputStream::new(&mut file);
    //     self.write_to(&mut writer)?;
    //     writer.flush()?;
    //     self.mut_stats().clear();
    //     Ok(())
    // }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut file = file.into_std().await;
        let mut writer = CodedOutputStream::new(&mut file); // TODO: Check if this can be done in async way
        self.write_to(&mut writer)?;
        writer.flush()?;
        self.mut_stats().clear();
        Ok(())
    }
}

pub struct ProtoOut;

#[async_trait]
impl super::super::OutputData for ProtoOut {
    fn add_data(&mut self, _data: Box<dyn Any>) -> Result<()> {
        Ok(())
    }

    // fn save(&mut self, path: &str) -> Result<()> {
    //     let mut file = File::create(path)?;
    //     file.write_all(include_bytes!("trading_stats.proto"))?;
    //     Ok(())
    // }

    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(include_bytes!("trading_stats.proto")).await?;
        Ok(())
    }
}
