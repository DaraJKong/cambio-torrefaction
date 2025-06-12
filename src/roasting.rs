use iced::{
    Alignment, Element,
    Length::Fill,
    Task, task,
    widget::{column, container, horizontal_space, row, text},
};

use crate::sensor;
use sensor::{Error, TempData};

#[derive(Clone, Debug)]
pub struct Roasting {
    bean_sensor: TempSensor,
    exhaust_sensor: TempSensor,
}

#[derive(Debug, Clone)]
pub enum Message {
    BeanUpdated(Update),
    ExhaustUpdated(Update),
    TryReconnect,
}

impl Roasting {
    pub fn boot() -> (Self, Task<Message>) {
        let mut bean_sensor = TempSensor::new("Bean:");
        let bean_task = bean_sensor.connect(0);

        let mut exhaust_sensor = TempSensor::new("Exhaust:");
        let exhaust_task = exhaust_sensor.connect(1);

        (
            Self {
                bean_sensor,
                exhaust_sensor,
            },
            Task::batch([
                bean_task.map(Message::BeanUpdated),
                exhaust_task.map(Message::ExhaustUpdated),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BeanUpdated(update) => {
                self.bean_sensor.update(update);
                Task::none()
            }
            Message::ExhaustUpdated(update) => {
                self.exhaust_sensor.update(update);
                Task::none()
            }
            Message::TryReconnect => {
                if self.bean_sensor.state {
                    
                }
                
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Roasting").size(30);

        let temperatures = column![self.bean_sensor.view(), self.exhaust_sensor.view()];

        let roasting = column![title, temperatures].spacing(20);

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
    TryReconnect
}

#[derive(Debug, Default, Clone)]
struct TempSensor {
    label: String,
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
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            state: State::default(),
        }
    }

    fn connect(&mut self, channel: i32) -> Task<Update> {
        match self.state {
            State::Created | State::Disconnected | State::Errored(_) => {
                let (task, handle) = Task::sip(
                    sensor::connect_temperature(0, 572104, channel),
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

    fn update(&mut self, update: Update) {
        if let State::Connected { temp_data, .. } = &mut self.state {
            match update {
                Update::Reading(t) => {
                    *temp_data = t;
                }
                Update::Disconnected(result) => {
                    self.state = match result {
                        Ok(_) => State::Disconnected,
                        Err(error) => State::Errored(error),
                    };
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        if matches!(self.bean_sensor.state, State::Disconnected | State::Errored) || matches!(self.exhaust_sensor.state, State::Disconnected | State::Errored) {
            return time::every(milliseconds(100)).map(Message::TryReconnect);
        }

        Subscription::none()
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
