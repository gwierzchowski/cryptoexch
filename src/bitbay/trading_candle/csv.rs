/*!
 * Implementation of CSV output format of "trading/candle/history" API from "BitBay" module.
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
        let file = file.into_std().await; // csv does not currently support async write (TODO: Maybe contribute)
        let mut wri = csv::Writer::from_writer(file); 
        for rec in &self.items {
            wri.serialize(rec)?;
        }
        self.items.clear();
        Ok(())
    }
}