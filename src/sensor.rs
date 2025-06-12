use iced::task::{Straw, sipper};

use std::sync::mpsc;
use std::time::Instant;

pub use phidget::errors::Error;

use phidget::{Phidget, TIMEOUT_DEFAULT, devices::TemperatureSensor};

pub fn connect_temperature(
    hub_port: i32,
    serial_number: i32,
    channel: i32,
) -> impl Straw<(), TempData, Error> {
    sipper(async move |mut temp_data| {
        let mut sensor = TemperatureSensor::new();

        sensor.set_hub_port(hub_port)?;
        sensor.set_serial_number(serial_number)?;
        sensor.set_channel(channel)?;

        sensor.open_wait(TIMEOUT_DEFAULT)?;

        let _port = sensor.hub_port()?;
        let t = sensor.temperature()?;

        temp_data
            .send(TempData {
                temp: t,
                time: Instant::now(),
            })
            .await;

        let (tx, rx) = mpsc::channel();

        sensor
            .set_on_temperature_change_handler(move |_, t: f64| {
                tx.send(t).unwrap();
            })
            .unwrap();

        while let Ok(t) = rx.recv() {
            temp_data
                .send(TempData {
                    temp: t,
                    time: Instant::now(),
                })
                .await;
        }

        Ok(())
    })
}

#[derive(Debug, Clone)]
pub struct TempData {
    pub temp: f64,
    pub time: Instant,
}

impl Default for TempData {
    fn default() -> Self {
        Self {
            temp: 0.0,
            time: Instant::now(),
        }
    }
}
