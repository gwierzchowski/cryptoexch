/*!
 * Implementation of JSON output format of trading/stats" API from "Zonda" module.
 */
use std::any::Any;
use std::convert::{TryFrom, TryInto};

use anyhow::{Result, Error};

use async_trait::async_trait;

use serde::Serialize;

use super::StatOut;

/// Output object implementation.
#[derive(Serialize, Debug)]
pub struct StatsAllOut {
    stats: Vec<StatOut>,
    #[serde(skip)]
    pub print_pretty: bool,
}

impl StatsAllOut {
    pub fn new() -> Self {
        StatsAllOut { stats: Vec::new(), print_pretty: false }
    }
}

impl TryFrom<&super::StatsAllIn> for StatsAllOut {
    type Error = Error;

    fn try_from(sin: &super::StatsAllIn) -> std::result::Result<Self, Self::Error> {
        let mut vout = Vec::with_capacity(sin.len());
        for it in sin {
            vout.push(it.try_into()?);
        }
        Ok(StatsAllOut { stats:vout, print_pretty:false })
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
    //     let mut file = File::create(path)?;
    //     let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
    //     file.write_all(js.as_bytes())?;
    //     self.stats.clear();
    //     Ok(())
    // }
    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
        file.write_all(js.as_bytes()).await?;
        self.stats.clear();
        Ok(())
    }
}