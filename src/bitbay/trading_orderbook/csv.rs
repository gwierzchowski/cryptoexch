/*!
 * Implementation of CSV output format of trading/orderbook" API from "BitBay" module.
 */
use std::any::Any;

use anyhow::Result;

use async_trait::async_trait;

use serde::Serialize;

/// Record of output object.
/// Output object depends on output format and is defined in respective sub-module.
#[derive(Serialize, Debug)]
struct OrderOut {
    timestamp: u64,
    sell_flg: u8, // use more compact 0/1 instead of bool which is represented in text false/true form by CSV serializer 
    count: u16,
    rate: f32,
    curr_amt: f32,
    prev_amt: f32,
    start_amt: f32,
}

/// Output object implementation.
#[derive(Serialize, Debug)]
pub struct OrderbooksOut {
    orders: Vec<OrderOut>,
}

impl OrderbooksOut {
    pub fn new() -> Self {
        OrderbooksOut { orders: Vec::new() }
    }
}

#[async_trait]
impl crate::common::OutputData for OrderbooksOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::OrderbookIn>() {
            for ord in data.buy {
                self.orders.push(
                    OrderOut {
                        timestamp: data.timestamp,
                        sell_flg: 0,
                        count: ord.co,
                        rate: ord.ra,
                        curr_amt: ord.ca,
                        prev_amt: ord.pa,
                        start_amt: ord.sa,
                    }
                );
            }
            for ord in data.sell {
                self.orders.push(
                    OrderOut {
                        timestamp: data.timestamp,
                        sell_flg: 1,
                        count: ord.co,
                        rate: ord.ra,
                        curr_amt: ord.ca,
                        prev_amt: ord.pa,
                        start_amt: ord.sa,
                    }
                );
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_orderbook::OrderbookIn type")
        }
    }
    
    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut wri = csv_async::AsyncSerializer::from_writer(file); 
        for rec in &self.orders {
            wri.serialize(rec).await?;
        }
        self.orders.clear();
        Ok(())
    }
}