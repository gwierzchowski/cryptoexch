/*!
 * Implementation of JSON output format of trading/orderbook" API from "BitBay" module.
 */
use std::any::Any;

use anyhow::Result;

use async_trait::async_trait;

use serde::Serialize;


/// Output object implementation.
// #[cfg(feature = "with_debug")]
#[derive(Serialize, Debug)]
pub struct OrderbooksOut {
    orders: Vec<super::OrderbookIn>,
    #[serde(skip)]
    pub print_pretty: bool,
}

impl OrderbooksOut {
    pub fn new() -> Self {
        OrderbooksOut { orders: Vec::new(), print_pretty: false }
    }
}


#[async_trait]
impl crate::common::OutputData for OrderbooksOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::OrderbookIn>() {
            self.orders.push(*data);
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_ticker::TickAllIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
        file.write_all(js.as_bytes()).await?;
        self.orders.clear();
        Ok(())
    }
}