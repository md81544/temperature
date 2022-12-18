use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CurrentWeather {
    temperature: f32,
    windspeed: f32,
    winddirection: f32,
    weathercode: i32,
    time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultData {
    latitude: f32,
    longitude: f32,
    generationtime_ms: f32,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f32,
    current_weather: CurrentWeather,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocationData {
    query: String,
    status: String,
    country: String,
    #[serde(rename = "countryCode")]
    country_code: String,
    region: String,
    #[serde(rename = "regionName")]
    region_name: String,
    city: String,
    zip: String,
    lat: f32,
    lon: f32,
    timezone: String,
    isp: String,
    org: String,
    #[serde(rename = "as")]
    as_: String,
}

#[tokio::main]
async fn main() {
    // First get current location. Using IP geolocation which won't be super-accurate.
    let rsp_location = reqwest::get("http://ip-api.com/json/").await.unwrap();
    let location_json = rsp_location.json::<LocationData>().await.unwrap();

    let query = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true",
        location_json.lat, location_json.lon
    );

    let rsp = reqwest::get(query).await.unwrap();
    let foo = rsp.json::<ResultData>().await.unwrap();
    println!(
        "The current temperature for lat {}, lon {} ({}) is {}°C",
        location_json.lat, location_json.lon, location_json.city, foo.current_weather.temperature
    );
}
