# Weather
Project shows weather, using [openweathermap](https://openweathermap.org/) and [weatherbit](https://www.weatherbit.io/).

## Usage
```bash
OPENWEATHER_APIKEY='openweather_key' -e WEATHERBIT_APIKEY='weatherbit_key' cargo run
```

## Docker usage
```bash
docker build -t weather .
docker run -p 8080:8080 -e OPENWEATHER_APIKEY='openweather_key' -e WEATHERBIT_APIKEY='weatherbit_key' weather
```
