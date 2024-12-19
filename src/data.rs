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
}

#[derive(Clone, Debug)]
pub struct Step {
    checkpoint: Checkpoint,
    step_type: StepType,
}

#[derive(Clone, Debug)]
pub enum Checkpoint {
    Time(Duration),
    Temp(f32),
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

pub fn on_off(temp: f32, time: u64) -> Step {
    Step {
        checkpoint: Checkpoint::Temp(temp),
        step_type: StepType::DurationOnOffGas(Duration::from_secs(time)),
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
