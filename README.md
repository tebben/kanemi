<p align="center" style="width: 100%;background-color: hotpink;">
  <img height="90" src="https://github.com/tebben/knmi-nowcast/blob/main/static/logo.png?raw=true">
</p>

The goal of kanemi is to provide libraries and tools to work more easily with KNMI api's and data.

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

### HARMONIE-AROME Cy43 forecasts Netherlands (P1)

HARMONIE information in gridded form (regular lat-lon) of near surface and boundary layer (up to 300 m) parameters from the UWC-West HARMONIE-AROME Cy43 model. For this weather forecast model, KNMI works closely with Iceland, Denmark and Ireland on local short-term weather forecasts under the name "United Weather Centres-West" (UWC-West). An international project team is working on a joint Numerical Weather Prediction model (NWP), procurement and management of the HPC (supercomputer) and infrastructure. The output frequency is 1 hour.

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
  - Get 2hr precipitation forecast for a specific location (lonlat)
- Actual synoptic observations
  - Load dataset from netCDF
  - Get all stations and their observations
  - Get closest station with observations and distance for a given location (lonlat)
- Harmonie Cy43 P1
  - Grib v1 reader optimized for Cy43 P1
  - Load dataset from a list of GRIB files, a directory containing GRIB files or directly from a .tar file
  - Get forecast for location(s) with optional requested parameters and time horizon
  - Get available parameters and their units

### Library pdok_geocoder

- Geocode free text search
- Reverse geocode lonlat/rd

### CLI tool kanecli

- Download KNMI data from the KNMI Data Platform
- Get notifications from the KNMI notification service
- Get precipitation forecast for a specific location
- Get Harmonie Cy43 P1 forecast for a specific location
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

Every command accepts it's own set of args, check the args with the `--help` flag on an option.
The CLI tool contains some useful commands but not eveything from the libraries in this repo is available in the CLI tool.

To install kanecli, clone the repository and build the project with cargo. This will install the binary in the cargo bin directory (usually `~/.cargo/bin`).

```bash
git clone https://github.com/tebben/kanemi.git
cd kanemi
cargo install --path kanecli
kenecli -h
```

```bash
A CLI tool to work with KNMI api's and some datasets may contain some other stuff aswell

Usage: kanecli <COMMAND>

Commands:
  download               Download KNMI data from the Open Data API
  nowcast-precipitation  Dataset: Nowcast precipitation (2 hour precipitation forecast)
  harmonie-cy43p1        Dataset: Harmonie CY43 P1 (60 hours weather forecast for the Netherlands)
  notifications          Receive messages from the KNMI notification service on new data availability
  geocoder               Geocode or reverse geocode a location using the PDOK Locatieserver
  help                   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Download

Download a dataset from the KNMI Data Platform. The dataset name and version can be found on the KNMI Data Platform. The command will return a JSON message
with a success flag and the path to the downloaded file if downloaded successfully. At this moment only the latest dataset can be downloaded.
Soon an option will be added to keep listening for notifications and keep downloading datasets when they are available.

```bash
Download KNMI data from the Open Data API

Usage: kanecli download [OPTIONS] --api-key <API_KEY> --name <NAME> --version <VERSION>

Options:
  -a, --api-key <API_KEY>    API key for the KNMI Open Data API [env: KNMI_API_KEY_OPEN_DATA]
  -d, --dir <DIR>            Output directory for the forecast data [default: ./output]
  -f, --filename <FILENAME>  The name to use for saving the file, leave black to use original name
  -n, --name <NAME>          The name of the dataset as found on the KNMI Open Data API
  -v, --version <VERSION>    The version of the dataset as found on the KNMI Open Data API
  -h, --help                 Print help (see more with '--help')
```

#### Example: Download the latest nowcast precipitation forecast

```bash
kanecli download -a <your-api-key> -d ./output -n radar_forecast -v 2.0
```

```JSON
{
  "success": true,
  "data": "./output/RAD_NL25_RAC_FM_202501031440.h5"
}
```

### Nowcast precipitation forecast

```bash
Print the precipitation forecast from input file or a newly downloaded KNMI dataset

Usage: kanecli nowcast-precipitation [OPTIONS] --api-key <API_KEY> --location <LOCATION>

Options:
  -a, --api-key <API_KEY>        API key for the KNMI Open Data API [env: KNMI_API_KEY_OPEN_DATA=]
  -l, --location <LOCATION>      Location as a comma-separated string "longitude,latitude" [env: KNMI_LOCATION=]
  -o, --output-dir <OUTPUT_DIR>  Output directory for the forecast data [default: ./output]
  -i, --input-file <INPUT_FILE>  Input file to load, new file will be downloaded if not provided
  -h, --help                     Print help (see more with '--help')
```

#### Example: Load the nowcast precipitation h5 file from disk and get the forecast for a given location

```bash
kanecli nowcast-precipitation -i ./example_data/RAD_NL25_RAC_FM_202412222055.h5 -l 4.91978668,52.36648685
```

#### Example: Download the latest dataset to ./output and get the precipitation forecast for a location

Since the latest dataset is downloaded you need to provide the api-key.

```bash
kanecli nowcast-precipitation -a <your-api-key> -o ./output -l 4.91978668,52.36648685
```

### Harmonie CY43 P1

```bash
kanecli harmonie-cy43p1 -h

Dataset: Harmonie CY43 P1 (60 hours weather forecast for the Netherlands)
Usage: kanecli harmonie-cy43p1 <COMMAND>

Commands:
  forecast    Get forecast data for a location
  parameters  Get all available parameters
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```bash
kanecli harmonie-cy43p1 forecast -h

Get forecast data for a location
Usage: kanecli harmonie-cy43p1 forecast [OPTIONS] --input <INPUT> --locations <LOCATIONS>

Options:
  -i, --input <INPUT>            file or list of files, a directory or tar file
  -l, --locations <LOCATIONS>    Locations in the format 'longitude,latitude'
  -p, --parameters <PARAMETERS>  Parameters in the format 'name,level'
  -H, --hours <HOURS>            The max forecast hours to return
  -h, --help                     Print help (see more with '--help')
```

#### Example: Get all available parameters

When running forecast we can supply which parameters we want to get. With the parameters command we get some insight in what parameters are available.
To pass parameters to the forecast command we need to know the short_name and level of the parameter.

```bash
kanecli harmonie-cy43p1 parameters
```

```JSON
[
  {
    "code": 20,
    "short_name": "vis",
    "description": "Visibility",
    "units": "m",
    "level_type": "HeightAboveGround",
    "level": 0,
    "time_range_indicator": "Instant",
    "has_bmp": false,
    "byte_index": null
  },
  ...
]
```

#### Example: Get Harmonie Cy43 P1 forecast for a location.

A few examples how to get the forecast for a location.

```bash
# Load from directory containing GRIB files, get temperature at level 0 and 50 for the given location with a maximum of 2 hours ahead
kanecli harmonie-cy43p1 forecast -i ~/Downloads/grib -p tmp,0 -p tmp,50 -l 5.351926,51.7168 -H 2

# Mulitple parameters can be supplied within -p and locations can contain multiple lonlat pairs, remove -H to get all available hours
kanecli harmonie-cy43p1 forecast -i ~/Downloads/grib -p 'tmp,0 tmp,50 vis,0' -l '5.351926,51.7168 4.123,52.123'

# You can also supply a single grib file or multiple files instead of a folder
kanecli harmonie-cy43p1 forecast -i ./example_data/HA43_N20_202412221800_00000_GB -p tmp,0 -l 5.351926,51.7168
kanecli harmonie-cy43p1 forecast -i 'file1 file2 file3 file4' -p tmp,0 -l 5.351926,51.7168

# Or if you just download a fresh .tar file from the KNMI Data Platform you can load it directly
# Data will be extracted to a temporary directory and is cleaned up when the dataset is dropped
kanecli harmonie-cy43p1 forecast -i ~/Downloads/HARM43_V1_P1_2024122218.tar -p tmp,0 -l 5.351926,51.7168
```

```bash
{
  "results": [
    {
      "location": [
        4.123,
        52.123
      ],
      "parameters": [
        {
          "name": "tmp",
          "level": 0,
          "values": [
            {
              "datetime": "2024-12-22T18:00:00Z",
              "value": 281.86865234375
            },
            {
              "datetime": "2024-12-22T19:00:00Z",
              "value": 281.868408203125
            }
          ]
        },
        ...
      ]
    },
    ...
  ]
}
```

### Dataplatform notifications

```bash
Receive messages from the KNMI notification service on new data availability

Usage: kanecli notifications [OPTIONS] --api-key <API_KEY> --name <NAME> --version <VERSION>

Options:
  -a, --api-key <API_KEY>      API key for the KNMI Notification Service [env: KNMI_API_KEY_NOTIFICATION]
  -n, --name <NAME>            The name of the dataset as found on the KNMI Open Data API
  -v, --version <VERSION>      The version of the dataset as found on the KNMI Open Data API
  -c, --client-id <CLIENT_ID>  Unique client id for the notification service [env: KNMI_CLIENT_ID_NOTIFICATION=]
  -h, --help                   Print help (see more with '--help')
```

#### Example: Receive notifications for the radar_forecast dataset version 2.0 (nowcast precipitation)

It can take some time before the first notification is received depending on the dataset.
The connection stays open until closed by the user. On disconnect it will try to reconnect and a JSON message is returned with parameter success false.

```bash
kanecli notifications -a <your-api-key> --dataset-name radar_forecast --dataset-version 2.0
```

```JSON
{
  "success": true,
  "topic": "dataplatform/file/v1/radar_forecast/2.0/created",
  "data": {
    "specversion": "1.0",
    "type": "nl.knmi.dataplatform.file.created.v1",
    "source": "https://dataplatform.knmi.nl",
    "id": "4c7ae265-f55a-30f1-4a12-4c72f1cd9f5c",
    "time": "2025-01-03T14:57:16Z",
    "datacontenttype": "application/json",
    "data": {
      "datasetName": "radar_forecast",
      "datasetVersion": "2.0",
      "filename": "RAD_NL25_RAC_FM_202501031455.h5",
      "url": "https://api.dataplatform.knmi.nl/open-data/v1/datasets/radar_forecast/versions/2.0/files/RAD_NL25_RAC_FM_202501031455.h5/url"
    }
  }
}
```

### PDOK Geocoder

```bash
kanecli geocoder -h

Geocode or reverse geocode a location using the PDOK Locatieserver
Usage: kanecli geocoder <COMMAND>

Commands:
  free     Classic geocoding using free text, check kanecli geocoder free --help for more information
  reverse  Reverse geocoding
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

```bash
kanecli geocoder free -h

Classic geocoding using free text, check kanecli geocoder free --help for more information
Usage: kanecli geocoder free [OPTIONS] --q <Q>

Options:
  -q, --q <Q>            Search query. Example: `museumplein 1 amsterdam`
      --best-match       Get and return only the best match
      --fl <FL>          Comma separated list of fields to return
      --fq <FQ>          Query to filter results see --help for more information and examples
      --start <START>    Start row, default 0 used for pagination
      --rows <ROWS>      Amount of rows to return, default 10, max 100
      --lonlat <LONLAT>  Sorting will done on distance for given coordinates, format --lonlat "5.12, 52.09"
      --df <DF>          Default search field
      --sort <SORT>      Sort order of the results, default: score desc
      --bq <BQ>          Boost fields for score calculation, see --info for more info
  -h, --help             Print help (see more with '--help')
```

```bash
kanecli geocoder reverse -h

Reverse geocoding
Usage: kanecli geocoder reverse [OPTIONS]

Options:
      --lonlat <LONLAT>      Longitude and latitude coordinates to reverse geocode, format --lonlat "5.12, 52.09"
      --rd <RD>              The RD coordinates to reverse geocode, format --rd "122000, 487000"
      --best-match           Get and return only the best match
      --fl <FL>              Comma separated list of fields to return
      --fq <FQ>              Query to filter results see --help for more information and examples
      --start <START>        Start row, default 0 used for pagination
      --rows <ROWS>          Amount of rows to return, default 10, max 100
      --types <TYPES>        Types to return, see --help for more information
      --distance <DISTANCE>  Max search distance in meters
  -h, --help                 Print help (see more with '--help')
```

#### Example: Geocode a location using the PDOK Locatieserver, filter on address type and return only the best match.

```bash
kanecli geocoder free -q "museumstraat 1 Amsterdam" --fq "type:adres" --best-match
```

#### Example: Reverse geocode, find the 5 best adres matches for the given lonlat within 15 meters.

```bash
kanecli geocoder reverse --lonlat "4.887295127944717,52.36849110206849" --rows 5 --distance 15
```

### Extra's

#### Example: Chaining geocoding and cy43p1 forecast. Get a location and return the 5 hour cy43p1 forecast for the given location.

```bash
kanecli geocoder free -q "den bosch" --best-match | \
jq -r '.centroide_ll | "\(.x),\(.y)"' | \
xargs -I {} kanecli cy43p1 forecast -i ~/Downloads/HARM43_V1_P1_2024122218.tar -l '{}' -p 'tmp,0' -H 5
```

## Building the CLI

```bash
cargo build --release
```

## App

Testing creating an app using Tauri with Svelte

```sh
cd app
cargo tauri dev
```
