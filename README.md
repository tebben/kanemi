<p align="center" style="width: 100%;background-color: hotpink;">
  <img height="128" src="https://github.com/tebben/knmi-nowcast/blob/main/static/logo.png?raw=true">
</p>

# knmi-nowcast

Nowcast precipitation forecast up to 2 hours ahead, per 5 minutes, over the Netherlands. The forecast contains 25 time steps: +0 minutes to +120 minutes. Forecasted is the precipitation sum per 5 minutes, on a grid of 1x1 km. The forecast is made with an operational KNMI implementation of pySTEPS. The forecast is initiated with the KNMI 5-minute real-time precipitation accumulation product: RTCOR-5m.

The goal is to provide a library to fetch and work with the KNMI nowcast precipitation forecast data and some tools to use it.

> Warning: LOOK MA! I'M DOING RUST!!!!

## Testimonials

- "I'm not sure what this is, but it looks cool" - Mom
- "Since I started using knmi-nowcast, my wife and dog moved back in with me" - TheDude22
- "Knmi-nowcast told me it was going to rain. I went out without an umbrella. It didn’t rain. So... thanks for making me look like an idiot." - StevenSeagul
- "Knmi-nowcast: Finally, a way to make precipitation predictions as accurate as flipping a coin." - NotImpressed44
- "LET'S GO!!! PRICIPITATION FORECASTING WITH KNMI-NOWCAST, BABY!!!" - SteveBall

## Not frequently asked questions

### Is the KNMI nowcast precipitation forecast better than buienradar/buienalarm?

I have no idea which data those services are using, this is not clear from their websites. If you have a rain sensor in your backyard you can measure and let me know.

### What is the quality of the code?

This is my first venture into rust so it's probably not great. I'm open to suggestions and PRs. But I'm also lazy so I might not do anything with them.

## Features

- Fetch latest file from KNMI Data Platform
- Load dataset from file
- Projection conversion between the HDF5 grid and EPSG:4326 (both directions)
- Read image data and their time attribute
- Get pixel values and mm/hr for a specific location (xy/lonlat)
- Simple CLI tool for demonstration

## Roadmap

- Full fletched CLI tool to do more stuff
- Live dataset manager to keep up to date trough KNMI MQTT service
- TUI tool to visualize the forecast (with live updates)
- Web server to provide the forecast data as a service
- Export GeoTIFF files

## Prerequisites

### HDF5

HDF5 library is required to run and build knmi-nowcast. Install it with:

```bash
sudo apt install libhdf5-dev
```

### KNMI API key

To fetch the nowcast data, you need an API key from the KNMI. You can find more information [here](https://developer.dataplatform.knmi.nl/open-data-api#token). An anonymous key is available on the previous mentioned page aswell.

Place the API key in a file called `.env` in the root of the project, see `.env.example` for an example.

## CLI Tool

For now there is only a small example to get the precipitation forecast for a specific location.

```bash
A CLI tool to get KNMI precipitation forecasts

Usage: knmi-cli [OPTIONS] --api-key <API_KEY> --location <LOCATION>

Options:
  -a, --api-key <API_KEY>        API Key for accessing the service [env: KNMI_API_KEY=]
  -l, --location <LOCATION>      Location as a comma-separated string "longitude,latitude" [env: KNMI_LOCATION=]
  -o, --output-dir <OUTPUT_DIR>  Output directory for storing downloaded files [env: KNMI_OUTPUT_DIR=] [default: ./output]
  -i, --input-file <INPUT_FILE>  Input file to load, new file will be downloaded if not provided [env: KNMI_INPUT_FILE=]
  -h, --help                     Print help
  -V, --version                  Print version
```

### Running the CLI

This will download the latest forecast and return the precipitation forecast for the given location.

```bash
cargo run -- --api-key <your-api-key> --location "<longitude>,<latitude>"
```

### Building the CLI

```bash
cargo build --release
```
