use iced::task::{Straw, sipper};

use std::sync::mpsc;
use std::time::Instant;

pub use phidget::errors::Error;

use phidget::{Phidget, TIMEOUT_DEFAULT, devices::TemperatureSensor};

pub fn connect_temperature(
    hub_port: i32,
    serial_number: i32,
    channel: i32,
) -> impl Straw<(), Event, Error> {
    sipper(async move |mut event| {
        let mut sensor = TemperatureSensor::new();

        sensor.set_hub_port(hub_port)?;
        sensor.set_serial_number(serial_number)?;
        sensor.set_channel(channel)?;

        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        let tx2 = tx.clone();

        sensor.set_on_temperature_change_handler(move |_, t: f64| {
            tx.send(Event::Change(TempData::new(t))).unwrap();
        })?;

        sensor.set_on_attach_handler(move |_| {
            tx1.send(Event::Attach).unwrap();
        })?;

        sensor.set_on_detach_handler(move |_| {
            tx2.send(Event::Detach).unwrap();
        })?;

        sensor.open_wait(TIMEOUT_DEFAULT)?;

        while let Ok(ev) = rx.recv() {
            match ev {
                Event::Detach => {
                    event.send(ev).await;
                    break;
                }
                _ => {
                    event.send(ev).await;
                }
            }
        }

        Ok(())
    })
}

#[derive(Debug, Clone)]
pub enum Event {
    Change(TempData),
    Attach,
    Detach,
}

#[derive(Debug, Clone)]
pub struct TempData {
    pub temp: f64,
    pub time: Instant,
}

impl TempData {
    fn new(temp: f64) -> Self {
        Self {
            temp,
            time: Instant::now(),
        }
    }
}

impl Default for TempData {
    fn default() -> Self {
        Self::new(0.0)
    }
}
