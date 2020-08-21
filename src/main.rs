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

## Cargo Features
Features which can be enabled / disabled during program build.

| Feature       | Default | Description |
|---------------|---------|-------------|
| `script_rhai` | off | Enables possibility to use [rhai](https://schungx.github.io/rhai/about/index.html) scripting language in configuration file |
| `out_csv`     | off | Enables CSV output file format |
| `out_pb`      | off | Enables Google Protocol Buffers output file format |
| `mod_bitbay`  | off | Enables module to support [BitBay](https://bitbay.net) service  |
|               |     |   |

# Usage

The program currently does not accept any command line parameters.
*/

#[macro_use] extern crate anyhow;
#[macro_use] extern crate clap;
#[macro_use] extern crate log;
// #[macro_use] extern crate lazy_static;

use std::fs::File;

use actix::prelude::*;

use anyhow::Result;

use clap::{Arg, App};

mod common;
mod generic_json;

#[cfg(feature = "mod_bitbay")]
mod bitbay;

#[actix_rt::main]
async fn main() -> Result<()> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("CONF")
            .help("Configuration file with tasks specification (default: program.yaml)")
            .required(false)
            .index(1))
        .get_matches();
    let conf_path = match matches.value_of("CONF") {
        Some(conf_path) => conf_path,
        None => "program.yaml"
    };
    let file = File::open(conf_path)?;
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
    // let system = System::new("cryptoexch");
    for task in conf.Tasks {
        actix::spawn(common::handle_task(task));
    }
    debug!("Tasks started");
    // system.run();
    // System::current().arbiter().join();
    Arbiter::local_join().await;
    debug!("All tasks finished");
    System::current().stop();
    info!("Service stopped");
    Ok(())
}
