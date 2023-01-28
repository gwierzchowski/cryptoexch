/*!
The `cryptoexch` program allows users to collect data from several REST services.
Special handlers are provided for collecting data from [Zonda](https://zondaglobal.com) Crypto-currency Exchange 
service using their exposed API described [here](https://docs.zonda.exchange/).

# Brief overview

The program reads file `program.yaml` located in current folder and executes configured tasks in concurrent way.
Each task connects to configured URL and downloads data in JSON format returned by service.
Task can be configured to wait given number of seconds and query for next pack of data to the same URL.
After configured number of loops task can save collected data to the file and clear the buffer.
Data can be saved in: JSON, CSV or Google Proto Buffers format for Zonda service or JSON format for any other service.
Task can be configured to end after given number of loops or run infinitely.
If task encounters any error it is being ended.
The program finish when all tasks are finished or keep running if at least one of the tasks is configured to run in infinite loop.

## Cargo Features
Features which can be enabled / disabled during program build.

| Feature       | Default | Description |
|---------------|---------|-------------|
| `script_rhai` | off | Enables possibility to use [rhai](https://schungx.github.io/rhai/about/index.html) scripting language in configuration file |
| `out_csv`     | off | Enables CSV output file format |
| `out_pb`      | off | Enables Google Protocol Buffers output file format |
| `mod_zonda`   | off | Enables module to support [Zonda](https://zondaglobal.com) service  |
|               |     |   |

# Usage

The program currently does not accept any command line parameters.
*/

#[macro_use] extern crate anyhow;
#[macro_use] extern crate log;

use std::fs::File;

use actix::prelude::*;
use anyhow::Result;
use clap::Parser;
use futures_util::future::join_all;

mod common;
mod generic_json;

#[cfg(feature = "mod_zonda")]
mod zonda;

/// Downloads data from Crypto Exchanges REST services.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Configuration file with tasks' specification
    #[arg(default_value = "program.yaml")]
    conf: std::path::PathBuf,
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    info!("Starting service");
    let file = File::open(args.conf)?;
    let conf = serde_yaml::from_reader::<_,common::Config>(file)?;
    if let Some(ref log_conf) = conf.Config.LogConf {
        log4rs::init_file(log_conf, Default::default())?;
    } else {
        use log::LevelFilter;
        use log4rs::append::console::ConsoleAppender;
        use log4rs::config::{Appender, Config, Root};
        let stdout = ConsoleAppender::builder().build();
        let config = Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(Root::builder().appender("stdout").build(LevelFilter::Warn))?;
        log4rs::init_config(config)?;
    }
    info!("Starting service");
    debug!("Config = {:?}", &conf);
    let mut join_handlers = Vec::with_capacity(conf.Tasks.len());
    for task in conf.Tasks {
        join_handlers.push(actix_rt::spawn(common::handle_task(task)));
    }
    debug!("Tasks started");
    join_all(join_handlers).await;
    debug!("All tasks finished");
    System::current().stop();
    info!("Service stopped");
    Ok(())
}
