/*!
 * Implementation of Protocol Buffers output format of "trading/candle/history" API from "Zonda" module.
 */
use std::any::Any;

use anyhow::Result;

use async_trait::async_trait;

use protobuf::{CodedOutputStream, Message};

mod trading_candle;

type Candle      = trading_candle::Candles_Candle;

/// Output object implementation.
pub type Candles = trading_candle::Candles;

#[async_trait]
impl crate::common::OutputData for Candles {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::CandlesIn>() {
            for item in data.items {
                let mut candle = Candle::new();
                candle.set_timestamp(item.0);
                candle.set_open(item.1.o);
                candle.set_close(item.1.c);
                candle.set_high(item.1.h);
                candle.set_low(item.1.l);
                candle.set_vol(item.1.v);
                candle.set_count(item.1.co);
                self.mut_candles().push(candle);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_candle::CandlesIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut file = file.into_std().await;
        let mut writer = CodedOutputStream::new(&mut file); // TODO: Check if this can be done in async way
        self.write_to(&mut writer)?;
        writer.flush()?;
        self.mut_candles().clear();
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
        file.write_all(include_bytes!("trading_candle.proto")).await?;
        Ok(())
    }
}
