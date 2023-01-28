/*!
 * Implementation of CSV output format of trading/ticker" API from "Zonda" module.
 */
use std::any::Any;
use std::convert::{From, Into};

use anyhow::Result;

use async_trait::async_trait;

use super::TickOut;

/// Output object implementation.
pub struct TickAllOut {
    ticks: Vec<TickOut>,
}

impl TickAllOut {
    pub fn new() -> Self {
        TickAllOut { ticks: Vec::new() }
    }
}

impl From<&super::TickAllIn> for TickAllOut {
    fn from(tin: &super::TickAllIn) -> Self {
        let mut vout = Vec::with_capacity(tin.len());
        for el in tin {
            vout.push(el.into());
        }
        TickAllOut { ticks:vout }
    }
}

#[async_trait]
impl crate::common::OutputData for TickAllOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TickAllIn>() {
            for ref d in *data {
                self.ticks.push(d.into());
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_ticker::TickAllIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut wri = csv_async::AsyncSerializer::from_writer(file); 
        for rec in &self.ticks {
            wri.serialize(rec).await?;
        }
        self.ticks.clear();
        Ok(())
    }
}