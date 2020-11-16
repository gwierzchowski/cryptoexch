# Cryptoexch

Data downloader from REST services.

I'm using this program in following data workflow:

- The program `cryptoexch` is deployed and run in cloud and collects data to cloud based storage
- Agent on my PC downloads those data when I turn on computer
- Then AI based program (which I constantly improve) analyze the data and when appropriate came up with signals pointing to particular markers to look at.

One of possible use cases for other users would be to clone program and clone `BitBay` module and modify for web service of choice (very easy task) or use module `GenericJson`. Main purpose of program is to constantly download new data and provide than in format more convenient for data analysis (like csv) or in more compact form (proto buffers) easier to manage.

## Features

- Downloads JSON data from configured Urls at given time intervals
- Can add flexible (e.g. time dependent) Url query parameters
- After given number of downloads saves collected data in the JSON file, whose name may contain auto-counter or date
- Can filter downloaded data
- Additional features for selected services:
  * [BitBay](https://bitbay.net) - additional output formats: csv, protocol buffers

## Cargo Features
Features which can be enabled / disabled during program build.

| Feature       | Default | Description |
|---------------|---------|-------------|
| `script_rhai` | off | Enables possibility to use [rhai](https://schungx.github.io/rhai/about/index.html) scripting language in configuration file |
| `out_csv`     | off | Enables CSV output file format |
| `out_pb`      | off | Enables Google Protocol Buffers output file format |
| `mod_bitbay`  | off | Enables module to support [BitBay](https://bitbay.net) service  |
|               |     |   |

## Usage

Program invocation:
```
USAGE:
    cryptoexch [CONF]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <CONF>    Configuration file with tasks specification (default: program.yaml)
```

Sample configuration file (program.yaml):
```yaml
Config:
  LogConf: log4rs.yml

Tasks:
  - Module: BitBay
    Api: trading/candle/history
    PathParams: ETH-PLN/60
    QueryParams:
      from: |
        (now_utc_millis() - 900*1000).to_string()
      to: |
        now_utc_millis().to_string()
    Format: csv
    OutPathMask: samples/candle_ETH_PLN_%$.csv
    Frequency: 900
    StopAfter: 3
    NewFileAfter: 1
  - Module: GenericJson
    Url: "http://api.nbp.pl/api"
    Api: exchangerates/tables
    PathParams: A
    QueryParams:
      format: json
    Format: json_pretty
    OutPathMask: samples/exchangerates_%F.json
    StopAfter: 1
    Filters:
      - - rates
        - rhai
        - |
          val.code == "USD" || 
          val.code == "EUR" || 
          val.code == "CHF"
```
Program will make 3 queries with ETH-PLN rates in 15 minutes (900 seconds) intervals and save them in files: `candle_ETH_PLN_1.csv`, `candle_ETH_PLN_2.csv`, `candle_ETH_PLN_3.csv`. In parallel it will get today's rates for USD, EUR and CHF from Polish National Bank and save it in file: `exchangerates_2020-08-11.json`. Program exits when last task finish. 

## Ideas / plans for further development

Program is in very initial state of development and is work in progress. Until 1.0.0 version is released some incompatible changes are possible including changes of format of main configuration file. Generally [semver](https://semver.org/) convention is used in program versioning.

Following are some ideas I consider to develop. Checked items are more likely to be added or ones for which development has started.

- [x] Implement `Config: PIDFile` option  
- [ ] Process `PathParams` key in similar way as `QueryParams`  
- [ ] Optimize parameters' processing - if there are no replacements, move parameter outside loop
- [ ] Data save procedures for some formats (pb) use blocking, sync functions
- [ ] Graceful shutdown
- [ ] To avoid races, files should be created / opened with .tmp extension and renamed to final name when completely done.
- [ ] Currently entire task exits when some invocation fails; make it more flexible - e.g. allow for next trials.
- [ ] Improve rhai scripts capabilities: e.g. introduce some container for state variables.
- [ ] Support for APIs that require authorization key.
- [ ] Support for Kraken crypto exchange.
- [ ] Special purpose module which allows user to run custom command (e.g. zip all created files).
- [ ] Support for more output formats - e.g. some compact binary formats compatible with Python pandas package, HDF5 or even direct injection to database.
- [ ] Integration tests
