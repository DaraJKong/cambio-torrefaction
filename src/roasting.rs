use core::panic;
use iced::{
    futures::{SinkExt, Stream},
    stream,
    widget::{column, container, text},
    Element,
    Length::Fill,
    Subscription,
};
use phidget::{
    devices::{temperature_sensor::THERMOCOUPLE_TYPE_J, TemperatureSensor},
    Error, Phidget,
};
use std::time::Duration;

const TIMEOUT: Duration = phidget::TIMEOUT_DEFAULT;

#[derive(Debug, Clone)]
pub enum WorkerEvent {
    Error(Error),
    Received(SensorData),
    Disconnected,
}

#[derive(Debug, Clone)]
pub enum SensorData {
    Bean(f64),
    Exhaust(f64),
}

pub fn phidgets_worker() -> impl Stream<Item = WorkerEvent> {
    stream::channel(100, |mut output| async move {
        let mut bean_sensor = TemperatureSensor::new();
        let mut exhaust_sensor = TemperatureSensor::new();

        // Set sensor channels
        if let Err(e) = bean_sensor.set_channel(0) {
            output.send(WorkerEvent::Error(e)).await.unwrap();
        }
        if let Err(e) = exhaust_sensor.set_channel(1) {
            output.send(WorkerEvent::Error(e)).await.unwrap();
        }

        // Open sensor channels
        if let Err(e) = bean_sensor.open_wait(TIMEOUT) {
            output.send(WorkerEvent::Error(e)).await.unwrap();
        }
        if let Err(e) = exhaust_sensor.open_wait(TIMEOUT) {
            output.send(WorkerEvent::Error(e)).await.unwrap();
        }

        // Set thermocouple types
        if let Err(e) = bean_sensor.set_thermocouple_type(THERMOCOUPLE_TYPE_J) {
            output.send(WorkerEvent::Error(e)).await.unwrap();
        }
        if let Err(e) = exhaust_sensor.set_thermocouple_type(THERMOCOUPLE_TYPE_J) {
            output.send(WorkerEvent::Error(e)).await.unwrap();
        }

        // Log port successes
        if let Ok(port) = bean_sensor.hub_port() {
            println!("Bean sensor opened on hub port: {}", port);
        }
        if let Ok(port) = exhaust_sensor.hub_port() {
            println!("Exhaust sensor opened on hub port: {}", port);
        }

        let (tx1, rx1) = std::sync::mpsc::channel();
        if bean_sensor
            .set_on_temperature_change_handler(move |_, t: f64| {
                tx1.send(t).unwrap();
            })
            .is_err()
        {
            output.send(WorkerEvent::Disconnected).await.unwrap();
        }
        let (tx2, rx2) = std::sync::mpsc::channel();
        if exhaust_sensor
            .set_on_temperature_change_handler(move |_, t: f64| {
                tx2.send(t).unwrap();
            })
            .is_err()
        {
            output.send(WorkerEvent::Disconnected).await.unwrap();
        }

        loop {
            if let Ok(t) = rx1.try_recv() {
                output
                    .send(WorkerEvent::Received(SensorData::Bean(t)))
                    .await
                    .unwrap();
            }
            if let Ok(t) = rx2.try_recv() {
                output
                    .send(WorkerEvent::Received(SensorData::Exhaust(t)))
                    .await
                    .unwrap();
            }
        }
    })
}

#[derive(Default, Clone, Debug)]
pub struct Roasting {
    bean_temp: f64,
    exhaust_temp: f64,
}

#[derive(Debug, Clone)]
pub enum Message {
    Event(WorkerEvent),
}

impl Roasting {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Event(WorkerEvent::Error(return_code)) => {
                panic!("Errored with ReturnCode: {return_code}")
            }
            Message::Event(WorkerEvent::Received(sensor_data)) => match sensor_data {
                SensorData::Bean(t) => self.bean_temp = t,
                SensorData::Exhaust(t) => self.exhaust_temp = t,
            },
            Message::Event(WorkerEvent::Disconnected) => (),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Roasting").size(30);

        let temperatures = column![text(self.bean_temp), text(self.exhaust_temp)];

        let roasting = column![title, temperatures].spacing(20);

        container(roasting.max_width(800).spacing(20))
            .center_x(Fill)
            .padding(20)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(phidgets_worker).map(Message::Event)
    }
}
