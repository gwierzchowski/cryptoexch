/*!
 * Implementation of JSON output format of "trading/candle/history" API from "BitBay" module.
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
    #[serde(skip)]
    pub print_pretty: bool,
}

impl CandlesOut {
    pub fn new() -> Self {
        CandlesOut { items: Vec::new(), print_pretty: false }
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
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
        file.write_all(js.as_bytes()).await?;
        self.items.clear();
        Ok(())
    }
}