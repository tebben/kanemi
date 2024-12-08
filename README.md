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

## Prerequisites

### HDF5

HDF5 library is required to run and build knmi-nowcast. Install it with:

```bash
sudo apt install libhdf5-dev
```

### KNMI API key

To fetch the nowcast data, you need an API key from the KNMI. You can find more information [here](https://developer.dataplatform.knmi.nl/open-data-api#token). An anonymous key is available on the previous mentioned page aswell.

Place the API key in a file called `.env` in the root of the project, see `.env.example` for an example.
