/*!
 * Implementation of JSON output format of trading/ticker" API from "BitBay" module.
 */
use std::any::Any;
use std::convert::{TryFrom, TryInto};

use anyhow::{Result, Error};

use async_trait::async_trait;

use serde::Serialize;

use super::TickOut;

/// Output object implementation.
#[derive(Serialize, Debug)]
pub struct TickAllOut {
    ticks: Vec<TickOut>,
    #[serde(skip)]
    pub print_pretty: bool,
}

impl TickAllOut {
    pub fn new() -> Self {
        TickAllOut { ticks: Vec::new(), print_pretty: false }
    }
}

impl TryFrom<&super::TickAllIn> for TickAllOut {
    type Error = Error;

    fn try_from(tin: &super::TickAllIn) -> std::result::Result<Self, Self::Error> {
        let mut vout = Vec::with_capacity(tin.len());
        for it in tin {
            vout.push(it.try_into()?);
        }
        Ok(TickAllOut { ticks:vout, print_pretty:false })
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

    // fn save(&mut self, path: &str) -> Result<()> {
    //     use std::fs::File;
    //     use std::io::prelude::*;
    //     let mut file = File::create(path)?;
    //     let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
    //     file.write_all(js.as_bytes())?;
    //     self.ticks.clear();
    //     Ok(())
    // }
    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
        file.write_all(js.as_bytes()).await?;
        self.ticks.clear();
        Ok(())
    }
}