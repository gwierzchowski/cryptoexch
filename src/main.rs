#[macro_use] extern crate anyhow;
#[macro_use] extern crate lazy_static;

use std::fs::File;

use actix::prelude::*;

use anyhow::Result;

mod common;
mod bitbay;

#[actix_rt::main]
async fn main() -> Result<()> {
    let file = File::open("program.yaml")?;
    let conf = serde_yaml::from_reader::<_,common::Config>(file)?;
    //println!("conf = {:?}", conf);
    //return Ok(());
    // let system = System::new("cryptoexch");
    for task in conf.Tasks {
        actix::spawn(common::handle_task(task));
    }
    // system.run();
    // System::current().arbiter().join();
    Arbiter::local_join().await;
    System::current().stop();
    Ok(())
}
