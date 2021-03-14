[![crates.io](https://img.shields.io/crates/v/co2-mini-monitor.svg)](https://crates.io/crates/co2-mini-monitor)
[![docs.rs](https://docs.rs/co2-mini-monitor/badge.svg)](https://docs.rs/co2-mini-monitor)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Build](https://github.com/lambdalisue/rs-co2-mini-monitor/actions/workflows/build.yml/badge.svg)](https://github.com/lambdalisue/rs-co2-mini-monitor/actions/workflows/build.yml)
[![Test](https://github.com/lambdalisue/rs-co2-mini-monitor/actions/workflows/test.yml/badge.svg)](https://github.com/lambdalisue/rs-co2-mini-monitor/actions/workflows/test.yml)
[![Audit](https://github.com/lambdalisue/rs-co2-mini-monitor/actions/workflows/audit.yml/badge.svg)](https://github.com/lambdalisue/rs-co2-mini-monitor/actions/workflows/audit.yml)

# co2-mini-monitor

An CUI application to log output of [CO2Mini Indoor AirQuality Monitor][] or [CUSTOM CO2 MONITOR CO2-mini][] to stdout.

[CO2Mini Indoor AirQuality Monitor]: https://www.co2meter.com/collections/desktop/products/co2mini-co2-indoor-air-quality-monitor
[CUSTOM CO2 MONITOR CO2-mini]: https://www.kk-custom.co.jp/emp/CO2-mini.html

```
$ co2-mini-monitor
{"time":"2021-03-13T12:41:26.632033+00:00","type":"co2","value":1100}
{"time":"2021-03-13T12:41:29.271876+00:00","type":"temp","value":21.7}
{"time":"2021-03-13T12:41:31.647808+00:00","type":"co2","value":1100}
{"time":"2021-03-13T12:41:34.295843+00:00","type":"temp","value":21.7}
{"time":"2021-03-13T12:41:36.671795+00:00","type":"co2","value":1104}
{"time":"2021-03-13T12:41:39.319682+00:00","type":"temp","value":21.7}
{"time":"2021-03-13T12:41:41.687658+00:00","type":"co2","value":1104}
{"time":"2021-03-13T12:41:44.335507+00:00","type":"temp","value":21.7}
{"time":"2021-03-13T12:41:46.711509+00:00","type":"co2","value":1108}
```

Note that this one does NOT handle packet decryption while it seems the packet is no longer encrypted in recent models. Use alternatives if you cannot get correct result due to the encryption. 

## Install

```
cargo install co2-mini-monitor
```

You may need to install the following to build [hidapi-rs](https://github.com/ruabmbua/hidapi-rs).

- libusb-1.0-0-dev
- libudev-dev
- libhidapi-dev

## Usage example

### InfluxDB

Use [jq][] to parse and format data and curl to send data to [influxdb][] like

[jq]: https://stedolan.github.io/jq/
[influxdb]: https://www.influxdata.com/

```sh
#!/bin/bash
HOSTNAME="localhost:8086"
ORG="YOUR ORG"
BUCKET="YOUR BUCKET"
TOKEN="YOUR TOKEN"
/usr/local/bin/co2-mini-monitor \
  | /usr/local/bin/jq -r --unbuffered '"\(.type) value=\(.value) \(.time | fromdate)"' \
  | xargs -I{} curl --request POST \
    "http://$HOSTNAME/api/v2/write?org=$ORG&bucket=$BUCKET&precision=s" \
    --header "Authorization: Token $TOKEN" \
    --data-raw {}
```


## See also

- [co2monitor](https://github.com/maddindeiss/co2-monitor) - CUI tool which handle packet decryption
- [Reverse-Engineering a low-cost USB CO2 monitor](https://hackaday.io/project/5301-reverse-engineering-a-low-cost-usb-co-monitor) - Project to reveal how the packet was encrypted

# License

The code follows MIT license written in [LICENSE](./LICENSE). Contributors need
to agree that any modifications sent in this repository follow the license.
