use std::any::Any;
use std::convert::{TryFrom, TryInto};

use anyhow::{Result, Error};

use super::TickOut;

pub struct TickAllOut {
    ticks: Vec<TickOut>,
}

impl TickAllOut {
    pub fn new() -> Self {
        TickAllOut { ticks: Vec::new() }
    }
}

impl TryFrom<&super::TickAllIn> for TickAllOut {
    type Error = Error;

    fn try_from(tin: &super::TickAllIn) -> std::result::Result<Self, Self::Error> {
        let mut vout = Vec::with_capacity(tin.len());
        for el in tin {
            vout.push(el.try_into()?);
        }
        Ok(TickAllOut { ticks:vout })
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
        // let file = std::fs::OpenOptions::new().create(true).append(true).open(path)?; // TODO: Change to non blocking
        // let mut wri = csv::Writer::from_writer(file); 
        let mut wri = csv::Writer::from_path(path)?; // TODO: Change to non blocking
        for rec in &self.ticks {
            wri.serialize(rec)?;
        }
        self.ticks.clear();
        Ok(())
    }
}