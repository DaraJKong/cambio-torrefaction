use iced::{
    Alignment, Element,
    Length::Fill,
    Subscription, Task, task,
    time::{self, milliseconds},
    widget::{column, container, horizontal_space, row, text},
};
use std::time::Instant;

use crate::sensor;
use sensor::{Error, TempData};

#[derive(Clone, Debug)]
pub struct Roasting {
    sensors: Vec<TempSensor>,
    last_id: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    SensorUpdated(usize, Update),
    TryReconnect(Instant)
}

impl Roasting {
    pub fn new_sensor(&mut self, label: &str, channel: i32) -> Task<Update> {
        let id = self.last_id;
        self.last_id += 1;
        self.sensors.push(TempSensor::new(id, label, 0, 572104, channel));
        self.sensors[id].connect()
    }

    pub fn boot() -> (Self, Task<Message>) {
        let mut roasting = Self {
            sensors: Vec::new(),
            last_id: 0
        };
       
        let bean_task = roasting.new_sensor("Bean:", 0);
        let exhaust_task = roasting.new_sensor("Exhaust:", 1);

        (
            roasting,
            Task::batch([
                bean_task.map(|update| Message::SensorUpdated(0, update)),
                exhaust_task.map(|update| Message::SensorUpdated(1, update)),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SensorUpdated(id, update) => {
                let _ = self.sensors[id].update(update);
                Task::none()
            }
            Message::TryReconnect(_) => {
                Task::batch(self.sensors.iter_mut().enumerate().map(|(i, s)| {
                    match s.state {
                        State::Disconnected | State::Errored(_) => s.connect().map(move |update| Message::SensorUpdated(i, update)),
                        _ => Task::none()
                    }
                }))
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(self.sensors.iter().map(|s| s.subscription()))
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Roasting").size(30);

        let sensors = column(self.sensors.iter().map(|s| s.view()));

        let roasting = column![title, sensors].spacing(20);

        container(roasting.max_width(800).spacing(20))
            .center_x(Fill)
            .padding(20)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Update {
    Reading(TempData),
    Disconnected(Result<(), Error>),
}

#[derive(Debug, Default, Clone)]
struct TempSensor {
    id: usize,
    label: String,
    hub_port: i32,
    serial_number: i32,
    channel: i32,
    state: State,
}

#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    Created,
    Connected {
        temp_data: TempData,
        _task: task::Handle,
    },
    Disconnected,
    Errored(Error),
}

impl TempSensor {
    fn new(id: usize, label: &str, hub_port: i32, serial_number: i32, channel: i32) -> Self {
        Self {
            id,
            label: label.to_string(),
            hub_port,
            serial_number,
            channel,
            state: State::default(),
        }
    }

    fn connect(&mut self) -> Task<Update> {
        match self.state {
            State::Created | State::Disconnected | State::Errored(_) => {
                let (task, handle) = Task::sip(
                    sensor::connect_temperature(self.hub_port, self.serial_number, self.channel),
                    Update::Reading,
                    Update::Disconnected,
                )
                .abortable();

                self.state = State::Connected {
                    temp_data: TempData::default(),
                    _task: handle.abort_on_drop(),
                };

                task
            }
            State::Connected { .. } => Task::none(),
        }
    }

    fn update(&mut self, update: Update) -> Task<Update> {
        if let State::Connected { temp_data, .. } = &mut self.state {
            match update {
                Update::Reading(t) => {
                    *temp_data = t;
                    Task::none()
                }
                Update::Disconnected(result) => {
                    self.state = match result {
                        Ok(_) => State::Disconnected,
                        Err(error) => State::Errored(error),
                    };
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Disconnected | State::Errored(_) => {
                time::every(milliseconds(100)).map(Message::TryReconnect)
            }
            _ => Subscription::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let temp = match &self.state {
            State::Created => text("Loading...").style(text::base),
            State::Connected { temp_data, _task } => {
                text(format!("{:.1} °C", temp_data.temp)).style(text::success)
            }
            State::Disconnected => text("Disconnected!").style(text::danger),
            State::Errored(error) => text(format!("Error! {}", error)).style(text::danger),
        };

        row![text(self.label.clone()), horizontal_space(), temp.size(25)]
            .width(200)
            .align_y(Alignment::Center)
            .into()
    }
}
