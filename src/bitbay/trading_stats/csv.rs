/*!
 * Implementation of CSV output format of trading/stats" API from "BitBay" module.
 */
use std::any::Any;
use std::convert::{TryFrom, TryInto};

use anyhow::{Result, Error};

use async_trait::async_trait;

use super::StatOut;

/// Output object implementation.
pub struct StatsAllOut {
    stats: Vec<StatOut>,
}

impl StatsAllOut {
    pub fn new() -> Self {
        StatsAllOut { stats: Vec::new() }
    }
}

impl TryFrom<&super::StatsAllIn> for StatsAllOut {
    type Error = Error;

    fn try_from(sin: &super::StatsAllIn) -> std::result::Result<Self, Self::Error> {
        let mut vout = Vec::with_capacity(sin.len());
        for it in sin {
            vout.push(it.try_into()?);
        }
        Ok(StatsAllOut { stats:vout })
    }
}

#[async_trait]
impl crate::common::OutputData for StatsAllOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::StatsAllIn>() {
            for ref d in *data {
                self.stats.push(d.try_into()?);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_stats::StatsAllIn type")
        }
    }

    // fn save(&mut self, path: &str) -> Result<()> {
    //     // let file = std::fs::OpenOptions::new().create(true).append(true).open(path)?; // TODO: Change to non blocking
    //     // let mut wri = csv::Writer::from_writer(file); 
    //     let mut wri = csv::Writer::from_path(path)?; // TODO: Change to non blocking
    //     for rec in &self.stats {
    //         wri.serialize(rec)?;
    //     }
    //     self.stats.clear();
    //     Ok(())
    // }

    async fn save(&mut self, path: &str) -> Result<()> {
        // let file = std::fs::OpenOptions::new().create(true).append(true).open(path)?; // TODO: Change to non blocking
        let file = tokio::fs::File::create(path).await?;
        let file = file.into_std().await; // csv does not currently support async write (TODO: Maybe contribute)
        let mut wri = csv::Writer::from_writer(file); 
        // let mut wri = csv::Writer::from_path(path)?; // TODO: Change to non blocking
        for rec in &self.stats {
            wri.serialize(rec)?;
        }
        self.stats.clear();
        Ok(())
    }
}