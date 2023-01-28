/*!
 * Implementation of Protocol Buffers output format of trading/ticker" API from "Zonda" module.
 */
use std::any::Any;

use anyhow::Result;

use async_trait::async_trait;

use protobuf::{CodedOutputStream, Message};

mod trading_orderbook;

type Order          = trading_orderbook::OrderBooks_Order;
type OrderType      = trading_orderbook::OrderBooks_Order_OrderType;

/// Output object implementation.
pub type OrderBooks = trading_orderbook::OrderBooks;

#[async_trait]
impl crate::common::OutputData for OrderBooks {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::OrderbookIn>() {
            for ord in data.buy {
                let mut order = Order::new();
                order.set_timestamp(data.timestamp);
                order.set_otype(OrderType::BYE);
                order.set_count(ord.co as u32);
                order.set_rate(ord.ra);
                order.set_curr_amt(ord.ca);
                order.set_prev_amt(ord.pa);
                order.set_start_amt(ord.sa);
                self.mut_orders().push(order);
            }
            for ord in data.sell {
                let mut order = Order::new();
                order.set_timestamp(data.timestamp);
                order.set_otype(OrderType::SELL);
                order.set_count(ord.co as u32);
                order.set_rate(ord.ra);
                order.set_curr_amt(ord.ca);
                order.set_prev_amt(ord.pa);
                order.set_start_amt(ord.sa);
                self.mut_orders().push(order);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_orderbook::OrderbookIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut file = file.into_std().await;
        let mut writer = CodedOutputStream::new(&mut file); // TODO: Check if this can be done in async way
        self.write_to(&mut writer)?;
        writer.flush()?;
        self.mut_orders().clear();
        Ok(())
    }
}

pub struct ProtoOut;

#[async_trait]
impl crate::common::OutputData for ProtoOut {
    fn add_data(&mut self, _data: Box<dyn Any>) -> Result<()> {
        Ok(())
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(include_bytes!("trading_orderbook.proto")).await?;
        Ok(())
    }
}
