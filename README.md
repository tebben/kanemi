<p align="center" style="width: 100%;background-color: hotpink;">
  <img height="90" src="https://github.com/tebben/knmi-nowcast/blob/main/static/logo.png?raw=true">
</p>

The goal of kanemi is to provide libraries and tools to work more easily with KNMI data.

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

KNMI collects observations from the automatic weather stations situated in the Netherlands and BES islands on locations such as aerodromes and North Sea platforms. In addition, wind data from KNMI wind poles are included. The weather stations report every 10 minutes meteorological parameters such as temperature, relative humidity, wind, air pressure, visibility, precipitation and cloud cover. The number of parameters differs per station. The file for the past 10-minutes is available a few minutes later. It is possible that a station's observations may not be immediately available. Files are updated with missing data up to 4 hours later.

## Features

This repo consists of multiple libraries and tools. Everythng is work in progress.

### Library kanemi

- Working with KNMI Data Platform
- Receiving notifications from the KNMI notification service
- Nowcast precipitation forecast
  - Load dataset from HDF5
  - Projection conversion between HDF5 grid and EPSG:4326 (both directions)
  - Read image data and their time attribute
  - Get pixel values and mm/hr for a specific location (xy/lonlat)
  - Get 2hr precipitation forecast for a specific location (xy/lonlat)
- Actual synoptic observations
  - Load dataset from netCDF
  - Get all stations and their observations
  - Get closest station with observations and distance for a given location (lonlat)

### Library pdok_geocoder

- Geocode free text search
- Reverse geocode lonlat/rd

### CLI tool kanecli

- Download KNMI data from the KNMI Data Platform
- Get notifications from the KNMI notification service
- Get precipitation forecast for a specific location
- Geocode and reverse geocode locations in the Netherlands

### App

- Desktop app using Tauri and Svelte (work in progress)

## Roadmap

- Desktop app using Tauri and Svelte
- Make CLI better and add more features
- Geocoder: Lookup, suggets
- More datasets
- Webservice/API

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

For now the CLI only contains 3 simple test commands to test the notification service, get precipitation forecast and geocoding. Every command accepts it's own set of args, check the args with the `--help` flag on an option. `kanecli forecast --help` for example.

```bash
A CLI tool to work with KNMI data and maybe some other stuff

Usage: kanecli <COMMAND>

Commands:
  download       Download KNMI data from the KNMI Data Platform
  forecast       Print the precipitation forecast from input file or a newly downloaded KNMI dataset
  notifications  Test the notification service
  geocoder       Geocode or reverse geocode a location using the PDOK Locatieserver
  help           Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Installation

To install kanecli, clone the repository and build the project with cargo. This will install the binary in the cargo bin directory (usually `~/.cargo/bin`).

```bash
git clone https://github.com/tebben/kanemi.git
cd kanemi
cargo install --path kanecli
```

### Examples

Download the latest forecast and return the precipitation forecast for the given location.

```bash
kanecli forecast -a <your-api-key> -l "<longitude>,<latitude>"
```

Get notification messages for the dataset radar_forecast version 2.0.

```bash
kanecli notifications -a <your-api-key> --dataset-name radar_forecast --dataset-version 2.0
```

Geocode a location using the PDOK Locatieserver, filter on address type and return only the best match.

```bash
kanecli geocoder free -q "museumstraat 1 Amsterdam" --fq "type:adres" --best-match
```

Reverse geocode, find the 5 best adres matches for the given lonlat within 15 meters.

```bash
kanecli geocoder reverse --lonlat "4.887295127944717,52.36849110206849" --rows 5 --distance 15
```

### Building the CLI

```bash
cargo build --release
```

## App

Testing creating an app using Tauri with Svelte

```sh
cd app
cargo tauri dev
```
