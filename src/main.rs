use anyhow::Result;
use chrono::Utc;
use hidapi::HidApi;

const VENDOR_ID: u16 = 0x04D9;
const PRODUCT_ID: u16 = 0xA052;

const OP_CO2: u8 = 0x50;
const OP_TMP: u8 = 0x42;

enum Value {
    CO2(u32),
    Temp(f64),
}

fn main() -> Result<()> {
    let api = HidApi::new()?;
    let device = api.open(VENDOR_ID, PRODUCT_ID)?;

    // Wake-up the device
    let buf = [0u8; 9];
    device.send_feature_report(&buf)?;

    // Use blocking mode
    device.set_blocking_mode(true)?;

    let mut buf = [0u8; 8];
    loop {
        let n = device.read(&mut buf)?;
        debug_assert!(n == 8, "Not enough bytes are read");

        if let Some(value) = parse_data(&buf) {
            let time = Utc::now().to_rfc3339();
            let obj = match value {
                Value::CO2(val) => json::object! {
                    type: "co2",
                    value: val,
                    time: time,
                },
                Value::Temp(val) => json::object! {
                    type: "temp",
                    value: val,
                    time: time,
                },
            };
            println!("{}", obj);
        }
    }
}

fn is_valid_checksum(buf: &[u8; 8]) -> bool {
    let sum: u16 = buf[..3].iter().map(|v| *v as u16).sum();
    let sum = (sum & 0xFF) as u8;
    sum == buf[3]
}

fn parse_data(buf: &[u8; 8]) -> Option<Value> {
    if buf[4] != 0x0D || !is_valid_checksum(&buf) {
        panic!("Invalid bytes were read: {:02x?}", buf);
    }

    let op = buf[0];
    let val = buf[1] as u32;
    let val = val << 8 | (buf[2] as u32);

    if op == OP_CO2 {
        Some(Value::CO2(val))
    } else if op == OP_TMP {
        let val = val as f64 / 16.0 - 273.15;
        let val = (val * 10.0).round() / 10.0;
        Some(Value::Temp(val as f64))
    } else {
        None
    }
}
