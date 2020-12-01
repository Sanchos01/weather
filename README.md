# Weather
Project shows weather, using [openweathermap](https://openweathermap.org/) and [weatherbit](https://www.weatherbit.io/).

## Usage
```bash
OPENWEATHER_APIKEY='openweather_key' -e WEATHERBIT_APIKEY='weatherbit_key' cargo run
```

## Example
```console
foo@bar:~$ curl "localhost:8080/day?city=Moscow&day=2"
-5.63
foo@bar:~$ curl "localhost:8080/forecast?city=Moscow"
[-1.53,-2.58,-5.63,-4.92,-5.09]
foo@bar:~$ curl "localhost:8080/day?city=Moscow&day=7"
{"kind":"Client","message":"day can't be more than 5"}
```

## Docker usage
```bash
docker build -t weather .
docker run -p 8080:8080 -e OPENWEATHER_APIKEY='openweather_key' -e WEATHERBIT_APIKEY='weatherbit_key' weather
```
