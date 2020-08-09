/*!
 * Implementation of Protocol Buffers output format of trading/transactions" API from "BitBay" module.
 */
use std::any::Any;

use anyhow::Result;

use async_trait::async_trait;

use protobuf::{CodedOutputStream, Message};

mod trading_transactions;

type Transaction      = trading_transactions::Transactions_Transaction;
type TranType         = trading_transactions::Transactions_Transaction_TranType;

/// Output object implementation.
pub type Transactions = trading_transactions::Transactions;

#[async_trait]
impl crate::common::OutputData for Transactions {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TransactionsIn>() {
            for item in data.items {
                let mut tra = Transaction::new();
                tra.set_timestamp(item.t);
                tra.set_ttype(if let super::BuySell::Buy = item.ty { TranType::BYE } else { TranType::SELL });
                tra.set_rate(item.r);
                tra.set_amt(item.a);
                tra.set_id(item.id);
                self.mut_trans().push(tra);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_transactions::TransactionsIn type")
        }
    }

    async fn save(&mut self, path: &str) -> Result<()> {
        let file = tokio::fs::File::create(path).await?;
        let mut file = file.into_std().await;
        let mut writer = CodedOutputStream::new(&mut file); // TODO: Check if this can be done in async way
        self.write_to(&mut writer)?;
        writer.flush()?;
        self.mut_trans().clear();
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
        file.write_all(include_bytes!("trading_transactions.proto")).await?;
        Ok(())
    }
}
