/*!
The `cryptoexch` program allows users to collect data from several RESTFull services.
Special handlers are provided for collecting data from [BitBay](https://bitbay.net) Crypto-currency Exchange 
service thru their exposed API described [here](https://docs.bitbay.net/reference).

# Brief overview

The program reads file `program.yaml` located in current folder and executes configured tasks in concurrent way.
Each task connects to configured URL and downloads data in JSON format returned by service.
Task can be configured to wait given number of seconds and query for next pack of data to the same URL.
After configured number of loops task can save collected data to the file and clear the buffer.
Data can be saved in: JSON, CSV or Google Proto Buffers format for BitBay service or JSON format for any other service.
Task can be configured to end after given number of loops or run infinitely.
If task encounters any error it is being ended.
The program finish when all tasks are finished or keep running if at least one of the tasks is configured to run in infinite loop.

# Usage

The program currently does not accept any command line parameters.
*/

#[macro_use] extern crate anyhow;
#[macro_use] extern crate lazy_static;

use std::fs::File;

use actix::prelude::*;

use anyhow::Result;

mod common;
mod generic_json;

#[cfg(feature = "bitbay")]
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
