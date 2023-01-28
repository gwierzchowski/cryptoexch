/*!
 * Implementation of JSON output format of "trading/transactions" API from "Zonda" module.
 */
use std::any::Any;
use std::convert::Into;

use anyhow::Result;

use async_trait::async_trait;

use serde::Serialize;


/// Output object implementation.
#[derive(Serialize, Debug)]
pub struct TransactionsOut {
    items: Vec<super::TransactionOut>,
    #[serde(skip)]
    pub print_pretty: bool,
}

impl TransactionsOut {
    pub fn new() -> Self {
        TransactionsOut { items: Vec::new(), print_pretty: false }
    }
}


#[async_trait]
impl crate::common::OutputData for TransactionsOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TransactionsIn>() {
            for ref d in data.items {
                self.items.push(d.into());
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_transactions::TransactionsIn type")
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