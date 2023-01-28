## Example 1

### Configuration file:
```yaml
Tasks:
  - Module: GenericJson
    Url: http://api.nbp.pl/api
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
### Result:  

Task will create 1 file (`samples/exchangerates_2020-08-17.json`) with exchange rates for 3 selected currencies.  
```json
  [
    {
      "effectiveDate": "2020-08-17",
      "no": "159/A/NBP/2020",
      "rates": [
        {
          "code": "USD",
          "currency": "dolar amerykański",
          "mid": 3.7076
        },
        {
          "code": "EUR",
          "currency": "euro",
          "mid": 4.3892
        },
        {
          "code": "CHF",
          "currency": "frank szwajcarski",
          "mid": 4.0828
        }
      ],
      "table": "A"
    }
  ]
```

## Example 2

### Configuration file:
```yaml
Tasks:
  - Api: trading/stats
    Module: Zonda
    Format: csv
    OutPathMask: samples/stats_ETH_%$_%s.csv
    StopAfter: 2
    Frequency: 3
    Filters:
      - - items
        - rhai
        - |
          if key.contains("ETH") { return true; }
          if val.h == () || val.l == () { return false; }
          let h = val.h.parse_float();
          let l = val.l.parse_float();
          if h == 0.0 || l == 0.0 { return false; }
          (h - l) / h > 0.1
```

### Format of data returned by service:
```json
{
  "status": "Ok",
  "items": {
    "AMLT-PLN": {
      "h": "0.03",
      "r24h": "0.02",
      "v": "15723.07101974",
      "m": "AMLT-PLN",
      "l": "0.02"
    },
    "BCC-PLN": {
      "h": "1063.08",
      "r24h": "970.37",
      "v": "1274.07302208",
      "m": "BCC-PLN",
      "l": "970.36"
    },
    "BSV-USD": {
      "h": null,
      "r24h": "151.97",
      "v": "0",
      "m": "BSV-USD",
      "l": null
    }
  }
}
```

### Result:  

Task will create 1 file (`samples/stats_ETH_1_1597697102.csv`) with statistics for markets related to ETH currency and those for which difference between highest and lowest rate is greater than 10%.  
First few rows of output file:
```csv
timestamp,market1,market2,vol,hi,lo,r24h
1597697099,ALG,BTC,562.3446,0.0000135,0.00001098,0.0000125
1597697099,ALG,PLN,176912.31,0.6,0.53,0.53
1597697099,AMLT,PLN,56352.8,0.03,0.02,0.03
1597697099,BAT,PLN,166706.0,1.25,1.09,1.13
1597697099,BCC,EUR,25.63087,265.0,180.0,261.02
```

## Example 3

### Configuration file:
```yaml
Tasks:
  - Api: trading/transactions
    Module: Zonda
    PathParams: ETH-PLN
    QueryParams:
      limit: 50
      fromTime: |
        ((now_utc() - 480) * 1000).to_string()
    Format: csv
    OutPathMask: samples/transactions_ETH_PLN.csv
    StopAfter: 2
    Frequency: 4
```

### Result:  

Task will create 1 file (`samples/transactions_ETH_PLN.csv`) with transactions on market `ETH-PLN` from last 8 minutes.  
First few rows of output file:
```csv
amt,id,rate,timestamp,sell_flg
2.5,20ca9c00-e19f-11ea-91c1-0242ac110009,1593.4,1597788415442,0
14.109076,970dd61a-e19e-11ea-91c1-0242ac110009,1590.11,1597788184357,1
2.5,20ca9c00-e19f-11ea-91c1-0242ac110009,1593.4,1597788415442,0
14.109076,970dd61a-e19e-11ea-91c1-0242ac110009,1590.11,1597788184357,1
```

## Example 4

### Format of data returned by service:
```json
{
  "orders": [
    {
      "buy": [
        {
          "co": 3,
          "ra": 1485.0,
          "ca": 0.9754209,
          "pa": 0.9754209,
          "sa": 0.9754209
        },
        {
          "co": 2,
          "ra": 1486.0,
          "ca": 0.9045109,
          "pa": 0.9045109,
          "sa": 1.5648721
        }
      ],
      "sell": [
        {
          "co": 1,
          "ra": 1557.27,
          "ca": 5.3006864,
          "pa": 5.3006864,
          "sa": 5.3006864
        },
        {
          "co": 1,
          "ra": 1558.74,
          "ca": 29.31,
          "pa": 29.31,
          "sa": 29.31
        },
      ],
      "seqNo": 38997921,
      "timestamp": 1597954462056
    }
  ]
}
```

### Configuration file:
```yaml
Tasks:
  - Module: Zonda
    Api: trading/orderbook-limited
    PathParams: ETH-PLN/10
    Format: csv
    OutPathMask: samples/orderbook_ETH_PLN_%$.csv
    StopAfter: 1
    Filters:
      - - buy
        - rhai
        - |
          val.ca.parse_float() >= 7.0
      - - sell
        - rhai
        - |
          val.ca.parse_float() >= 7.0
```

### Result:  

Task will create 1 file (`samples/orderbook_ETH_PLN_1.csv`) with orders with amount greater or equal to 7 ETH of 10 depth orderbook from market `ETH-PLN`.  
Sample output file:
```csv
timestamp,sell_flg,count,rate,curr_amt,prev_amt,start_amt
1597954980982,0,1,1540.68,18.7038,18.7038,18.7038
1597954980982,0,1,1544.01,161.791,161.791,161.791
1597954980982,0,1,1546.49,37.045895,37.045895,37.045895
1597954980982,0,1,1548.01,16.203,16.203,16.203
1597954980982,1,1,1550.0,11.674605,11.674605,11.868155
1597954980982,1,2,1551.41,77.873856,77.873856,77.873856
1597954980982,1,1,1552.69,7.954104,7.954104,7.954104
1597954980982,1,1,1559.99,29.3,29.3,29.3
1597954980982,1,1,1561.76,16.203,16.203,16.203
1597954980982,1,1,1565.54,12.0,12.0,12.0
1597954980982,1,1,1565.55,30.12,30.12,30.12
1597954980982,1,1,1565.99,161.791,161.791,161.791
```

## Example 5

### Format of data returned by service:
```json
  {
    "items": {
      "ALG-BTC": {
        "highestBid": 0.00001113,
        "lowestAsk": 0.00001396,
        "market": {
          "code": "ALG-BTC",
          "first": {
            "currency": "ALG",
            "minOffer": 0.91,
            "scale": 8
          },
          "second": {
            "currency": "BTC",
            "minOffer": 9.9e-6,
            "scale": 8
          }
        },
        "previousRate": 0.00001099,
        "rate": 0.00001098,
        "time": 1597953085808
      },
      "ALG-PLN": {
        "highestBid": 0.54,
        "lowestAsk": 0.55,
        "market": {
          "code": "ALG-PLN",
          "first": {
            "currency": "ALG",
            "minOffer": 8.1,
            "scale": 8
          },
          "second": {
            "currency": "PLN",
            "minOffer": 5,
            "scale": 2
          }
        },
        "previousRate": 0.54,
        "rate": 0.55,
        "time": 1597950086348
      }
    },
    "status": "Ok"
  }
```

### Configuration file:
```yaml
Tasks:
  - Module: Zonda
    Api: trading/ticker
    Format: pb_proto
    OutPathMask: samples/ticker.proto
    StopAfter: 1
  - Module: Zonda
    Api: trading/ticker
    Format: pb
    OutPathMask: samples/ticker_ETH_%$.pb
    StopAfter: 4
    NewFileAfter: 1
    Frequency: 5
    Filters:
      - ["items", "by_key_re", ".*ETH.*"]
```

### Result:  

Tasks will create file `ticker.proto` with format definition and 4 files `ticker_ETH_1.pb`, `ticker_ETH_2.pb`, `ticker_ETH_3.pb`, `ticker_ETH_4.pb` with data from markets related to ETH taken in 5 second intervals.  
Proto file:
```proto
syntax = "proto3";

message TradingTickAll {
  message TradingTick {
    uint64 time = 1;
    float lowestAsk = 2;
    float previousRate = 3;
    float rate = 4;
    float highestBid = 5;
    uint32 scale1 = 6;
    string currency1 = 7;
    float minOffer1 = 8;
    uint32 scale2 = 9;
    string currency2 = 10;
    float minOffer2 = 11;
  }
  repeated TradingTick ticks = 1;
}
```
Note: if there were no `NewFileAfter: 1` key in configuration file, then all data from 4 runs would be saved in one pb file.
