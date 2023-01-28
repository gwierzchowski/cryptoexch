/*!
 * Implementation of CSV output format of "trading/candle/history" API from "Zonda" module.
 */
use std::any::Any;
use std::convert::Into;

use anyhow::Result;

use async_trait::async_trait;

use serde::Serialize;

/// Output object implementation.
#[derive(Serialize, Debug)]
pub struct CandlesOut {
    items: Vec<super::CandleOut>,
}

impl CandlesOut {
    pub fn new() -> Self {
        CandlesOut { items: Vec::new() }
    }
}

#[async_trait]
impl crate::common::OutputData for CandlesOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::CandlesIn>() {
            for ref d in data.items {
                self.items.push(d.into());
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_candle::CandlesIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut wri = csv_async::AsyncSerializer::from_writer(file); 
        for rec in &self.items {
            wri.serialize(rec).await?;
        }
        self.items.clear();
        Ok(())
    }
}