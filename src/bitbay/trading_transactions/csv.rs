/*!
 * Implementation of CSV output format of trading/orderbook" API from "BitBay" module.
 */
use std::any::Any;
use std::convert::TryInto;

use anyhow::Result;

use async_trait::async_trait;

use serde::Serialize;

/// Output object implementation.
#[derive(Serialize, Debug)]
pub struct TransactionsOut {
    items: Vec<super::TransactionOut>,
}

impl TransactionsOut {
    pub fn new() -> Self {
        TransactionsOut { items: Vec::new() }
    }
}

#[async_trait]
impl crate::common::OutputData for TransactionsOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TransactionsIn>() {
            for ref d in data.items {
                self.items.push(d.try_into()?);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_transactions::TransactionsIn type")
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