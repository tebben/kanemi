# knmi-nowcast

Nowcast precipitation forecast up to 2 hours ahead, per 5 minutes, over the Netherlands. The forecast contains 25 time steps: +0 minutes to +120 minutes. Forecasted is the precipitation sum per 5 minutes, on a grid of 1x1 km. The forecast is made with an operational KNMI implementation of pySTEPS. The forecast is initiated with the KNMI 5-minute real-time precipitation accumulation product: RTCOR-5m.

The goals is to provide a library and tools to fetch, process and visualize KNMI nowcast precipitation forecast data.

> Warning: LOOK MA! I'M DOING RUST!!!! (Rust learning project)

## Running

Install libhdf5-dev, create .env file with your KNMI API key,  see .env.example

```bash
sudo apt install libhdf5-dev
cargo build
cargo run
```
