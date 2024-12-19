use iced::{
    border,
    widget::{container, row, text},
    Alignment, Background, Element, Theme,
};
use once_cell::sync::Lazy;
use std::{fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct Recipe {
    name: String,
    steps: Vec<Step>,
}

impl fmt::Display for Recipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Recipe {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn steps(&self) -> &Vec<Step> {
        &self.steps
    }
}

#[derive(Clone, Debug)]
pub enum Checkpoint {
    Time(Duration),
    Temp(f32),
}

impl Checkpoint {
    pub fn view<'a, Message>(&self) -> Element<'a, Message> {
        match self {
            Checkpoint::Time(duration) => {
                let secs = duration.as_secs_f32();
                text(format!("{}:{}", (secs / 60.).floor(), secs % 60.))
            }
            Checkpoint::Temp(temp) => text(format!("{} °C", temp)),
        }
        .into()
    }
}

#[derive(Clone, Debug)]
pub enum StepType {
    Start,
    End,
    AdjustAirflow(f32),
    SwitchGas(bool),
    AdjustGas(f32),
    DurationOnOffGas(Duration),
    DeltaTempOnOffGas(f32),
    SwitchCooling(bool),
    SwitchMixing(bool),
}

impl StepType {
    pub fn view<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        match self {
            StepType::Start => text("Start").style(text::success).into(),
            StepType::End => text("End").style(text::danger).into(),
            StepType::AdjustAirflow(airflow) => container(text(format!("{}", airflow)).center())
                .padding([2.5, 10.])
                .style(|theme: &Theme| {
                    container::background(Background::Color(
                        theme.extended_palette().secondary.base.color,
                    ))
                    .border(border::rounded(100))
                })
                .into(),
            StepType::SwitchGas(gas) => {
                let label = if *gas { "ON" } else { "OFF" };
                container(text(label).center())
                    .padding([2.5, 10.])
                    .style(|theme: &Theme| {
                        let style = container::background(Background::Color(if *gas {
                            theme.palette().success
                        } else {
                            theme.palette().danger
                        }));
                        style
                            .border(border::rounded(100))
                            .color(theme.palette().background)
                    })
                    .into()
            }
            StepType::AdjustGas(gas) => text(format!("{}", gas)).style(text::primary).into(),
            StepType::DurationOnOffGas(duration) => {
                text(format!("o/o {} secs", duration.as_secs()))
                    .style(text::primary)
                    .into()
            }
            StepType::DeltaTempOnOffGas(delta) => text(format!("o/o {} °C", delta))
                .style(text::primary)
                .into(),
            StepType::SwitchCooling(cooling) => {
                let label = if *cooling {
                    "Turn ON the cooling"
                } else {
                    "Turn OFF the cooling"
                };
                text(label).style(text::secondary).into()
            }
            StepType::SwitchMixing(mixing) => {
                let label = if *mixing {
                    "Turn ON the mixing"
                } else {
                    "Turn OFF the mixing"
                };
                text(label).style(text::secondary).into()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Step {
    checkpoint: Checkpoint,
    step_type: StepType,
}

impl Step {
    pub fn view<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        row![self.checkpoint.view(), self.step_type.view()]
            .height(35)
            .align_y(Alignment::Center)
            .spacing(20)
            .into()
    }
}

pub fn start(temp: f32) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::Start,
    }
}

pub fn end(temp: f32) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::End,
    }
}

pub fn adjust_airflow(temp: f32, airflow: f32) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::AdjustAirflow(airflow),
    }
}

pub fn switch_gas(temp: f32, to: bool) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::SwitchGas(to),
    }
}

pub fn on_off(temp: f32, secs: u64) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::DurationOnOffGas(Duration::from_secs(secs)),
    }
}

pub fn temp_on_off(temp: f32, delta: f32) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::DeltaTempOnOffGas(delta),
    }
}

pub fn switch_cooling(temp: f32, to: bool) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::SwitchCooling(to),
    }
}

pub fn switch_mixing(temp: f32, to: bool) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::SwitchMixing(to),
    }
}

pub fn t_start(time: (u64, u64)) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::Start,
    }
}

pub fn t_end(time: (u64, u64)) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::End,
    }
}

pub fn t_adjust_airflow(time: (u64, u64), airflow: f32) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::AdjustAirflow(airflow),
    }
}

pub fn t_switch_gas(time: (u64, u64), to: bool) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::SwitchGas(to),
    }
}

pub fn t_on_off(time: (u64, u64), secs: u64) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::DurationOnOffGas(Duration::from_secs(secs)),
    }
}

pub fn t_temp_on_off(time: (u64, u64), delta: f32) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::DeltaTempOnOffGas(delta),
    }
}

pub fn t_switch_cooling(time: (u64, u64), to: bool) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::SwitchCooling(to),
    }
}

pub fn t_switch_mixing(time: (u64, u64), to: bool) -> Step {
    Step {
        checkpoint: Checkpoint::Time(Duration::from_secs(time.0 * 60 + time.1)),
        step_type: StepType::SwitchMixing(to),
    }
}

pub static DUMB_RECIPE: Lazy<Recipe> = Lazy::new(|| Recipe {
    name: "Nicaragua".to_string(),
    steps: vec![
        start(200.),
        adjust_airflow(100., 0.333),
        adjust_airflow(160., 0.667),
        on_off(165., 0),
        on_off(170., 0),
        on_off(174., 0),
        on_off(178., 0),
        on_off(182., 2),
        temp_on_off(185., 1.),
        on_off(188., 3),
        switch_gas(190., false),
        switch_gas(192., true),
        switch_gas(194., false),
        switch_gas(196., true),
        on_off(201., 0),
        switch_gas(207., false),
        switch_gas(209., true),
        switch_gas(210., false),
        on_off(211., 3),
        on_off(214., 2),
        on_off(216., 2),
        on_off(218., 2),
        on_off(220., 2),
        on_off(222., 2),
        on_off(224., 0),
        end(226.),
    ],
});

pub static TIME_RECIPE: Lazy<Recipe> = Lazy::new(|| Recipe {
    name: "Guatemala Natural".to_string(),
    steps: vec![
        start(190.),
        adjust_airflow(100., 0.5),
        t_switch_cooling((2, 10), false),
        t_switch_mixing((3, 15), false),
        on_off(160., 0),
        on_off(170., 0),
        on_off(175., 0),
        on_off(180., 0),
        on_off(182., 2),
        on_off(185., 3),
        switch_gas(187., false),
        switch_gas(189., true),
        switch_gas(192., false),
        switch_gas(193., true),
        on_off(195., 2),
        switch_gas(208., false),
        on_off(209., 3),
        on_off(210., 3),
        on_off(212., 2),
        on_off(214., 2),
        on_off(216., 2),
        switch_cooling(216.5, true),
        switch_mixing(216.5, true),
        t_end((12, 38)),
    ],
});
