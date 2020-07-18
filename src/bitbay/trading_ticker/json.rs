use std::any::Any;
use std::fs::File;
use std::io::prelude::*;
use std::convert::{TryFrom, TryInto};

use anyhow::{Result, Error};

use serde::Serialize;

use super::TickOut;

#[derive(Serialize, Debug)]
pub struct TickAllOut {
    ticks: Vec<TickOut>,
    #[serde(skip)]
    pub print_pretty: bool,
}

impl TickAllOut {
    pub fn new() -> Self {
        TickAllOut { ticks: Vec::new(), print_pretty: false }
    }
}

impl TryFrom<&super::TickAllIn> for TickAllOut {
    type Error = Error;

    fn try_from(tin: &super::TickAllIn) -> std::result::Result<Self, Self::Error> {
        let mut vout = Vec::with_capacity(tin.len());
        for it in tin {
            vout.push(it.try_into()?);
        }
        Ok(TickAllOut { ticks:vout, print_pretty:false })
    }
}

impl super::super::OutputData for TickAllOut {
    fn add_data(&mut self, data: Box<dyn Any>) -> Result<()> {
        if let Ok(data) = data.downcast::<super::TickAllIn>() {
            for ref d in *data {
                self.ticks.push(d.try_into()?);
            }
            Ok(())
        } else {
            bail!("Logical program error: data should be of trading_ticker::TickAllIn type")
        }
    }

    fn save(&mut self, path: &str) -> Result<()> {
        let mut file = File::create(path)?;
        let js = if self.print_pretty { serde_json::to_string_pretty(self)? } else { serde_json::to_string(self)? };
        file.write_all(js.as_bytes())?;
        self.ticks.clear();
        Ok(())
    }
}