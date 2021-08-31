use env_logger::Env;
use log::{debug, info};
use rppal::gpio::Gpio;
use serde::Deserialize;
use std::env;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug)]
pub struct Measurement {
    unix_timestamp: u64,
    temperature_c: f64,
    pressure_pa: f64,
    humidity_relative: f64,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    sensor_path: String,
    relay_pin: u8,
    relay_duration_secs: u64,
    humidity_high: f64,
    temperature_low: f64,
}

#[tokio::main]
async fn main() {
    // Initialize logger
    let env = Env::default().default_filter_or("warn");
    env_logger::init_from_env(env);

    // Get configuration file location if present as command line argument
    let args: Vec<String> = env::args().collect();
    let config_path: &str = if args.len() > 1 {
        &args[1]
    } else {
        debug!("No configuration file specified, falling back to default.");
        "config.toml"
    };

    // Parse configuration file
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name(config_path))
        .expect("Error parsing configuration file.");
    let settings: Settings = config
        .try_into()
        .expect("Configuration file contains errors.");

    // Fetch measurement from the sensor
    let resp: Measurement = reqwest::get(settings.sensor_path)
        .await
        .expect("Failed to obtain weather data from the sensor")
        .json::<Measurement>()
        .await
        .expect("Failed to parse response into struct.");

    // Make sure humidity is not too high
    if resp.humidity_relative > settings.humidity_high {
        info!("Exiting due to too high humidity.");
        return;
    }

    // Make sure temperature is not too low
    if resp.temperature_c < settings.temperature_low {
        info!("Exiting due to too low temperature.");
        return;
    }

    // Initialize GPIO system
    let gpio: Gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut pin = gpio
        .get(settings.relay_pin)
        .expect("Failed to get GPIO pin")
        .into_output();

    // Pulse output pin on for 5 minutes
    pin.set_high();
    let sleep_duration: Duration = Duration::from_secs(settings.relay_duration_secs);
    thread::sleep(sleep_duration);
    pin.set_low();

    //Log successful relay activation
    info!("Relay successfully activated.")
}
