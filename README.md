<p align="center" style="width: 100%;background-color: hotpink;">
  <img height="90" src="https://github.com/tebben/knmi-nowcast/blob/main/static/logo.png?raw=true">
</p>

The goal with kanemi is to provide a library and tools to work more easily with KNMI data.

> Warning: LOOK MA! I'M DOING RUST!!!! (This repo is a learning project for Rust, no guarantees for the quality)

## Testimonials

- "I'm not sure what this is, but it looks cool" - Mom
- "Since I started using kanemi, my wife and dog moved back in with me" - TheDude22
- "kanemi told me it was going to rain. I went out without an umbrella. It didn’t rain. So... thanks for making me look like an idiot." - StevenSeagul
- "kanemi: Finally, a way to make forecasts as accurate as flipping a coin." - NotImpressed44
- "LET'S GO!!! PRICIPITATION FORECASTING AND MUCH MORE WITH KANEMI, BABY!!!" - SteveBall

## Datasets

Kanemi tries to provide easy access to the following datasets

### Nowcast precipitation forecast

[Nowcast precipitation forecast](https://dataplatform.knmi.nl/dataset/radar-forecast-2-0) up to 2 hours ahead, per 5 minutes, over the Netherlands. The forecast contains 25 time steps: +0 minutes to +120 minutes. Forecasted is the precipitation sum per 5 minutes, on a grid of 1x1 km. The forecast is made with an operational KNMI implementation of pySTEPS. The forecast is initiated with the KNMI 5-minute real-time precipitation accumulation product: RTCOR-5m.

### HARMONIE-AROME Cy43 forecasts Netherlands

ToDo

### Meteo data - actual synoptic observations
> kanemi
ToDo

## Features

- Using the KNMI Data Platform
- Using the KNMI notification service (work in progress)
- Nowcast precipitation forecast
  - Load dataset from HDF5 files
  - Projection conversion between the HDF5 grid and EPSG:4326 (both directions)
  - Read image data and their time attribute
  - Get pixel values and mm/hr for a specific location (xy/lonlat)
- Simple CLI tool only for demonstration yet

## Roadmap

- CLI tool
- More datasets
- Webservice
- Converters

## Prerequisites

### HDF5

HDF5 library is required to run and build knmi-nowcast. Install it with:

```bash
sudo apt install libhdf5-dev
```

### KNMI API key

#### Open Data API
To fetch the nowcast data, you need an API key from the KNMI. You can find more information [here](https://developer.dataplatform.knmi.nl/open-data-api#token). An anonymous key is available on the mentioned page aswell.

#### Notifications

For the notifications service a different API key is needed and can be requested like the Open Data API key.

## kanecli - CLI Tool

For now the CLI only contains 2 simple test commands to test the notification service and to get the precipitation forecast. Every command accepts it's own set of args, check the args with the `--help` flag on an option. `cargo run -- forecase --help` for example. All args can also be set using environment variables trough the system or a `.env` file.

```bash
A CLI tool to work with KNMI data

Usage: kanemi <COMMAND>

Commands:
  forecast       Print the precipitation forecast from input file or a newly downloaded KNMI dataset
  notifications  Test the notification service
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Running the CLI

This will download the latest forecast and return the precipitation forecast for the given location.

```bash
cargo run -- forecast -a <your-api-key> -l "<longitude>,<latitude>"
```

### Building the CLI

```bash
cargo build --release
```
