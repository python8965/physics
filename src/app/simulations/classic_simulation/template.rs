use std::ops::Mul;

use egui::plot::{Line, PlotPoints};
use nalgebra::Vector2;

use crate::app::graphics::define::PlotDrawItem;
use crate::app::graphics::CSPlotObjects;
use crate::app::simulations::classic_simulation::object::CSObjectState;
use crate::app::simulations::classic_simulation::template::init::{BasicSimInit, SimulationInit};
use crate::app::simulations::classic_simulation::template::stamp::{
    CSObjectStamp, CSObjectStampResult,
};
use crate::app::simulations::classic_simulation::CSObject;
use crate::app::NVec2;

mod classic;
pub mod init;
pub mod stamp;

#[derive(Clone, Copy, Debug)]
pub enum CSTemplate {
    BasicSim,
    BasicSimWithInit(BasicSimInit),
    ProjectileMotionSim,
    ProjectileMotionSim2,
}

impl CSTemplate {
    pub fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn get_preset_with_ui(self) -> CSPreset {
        match self {
            CSTemplate::BasicSim => basic_sim(),
            CSTemplate::ProjectileMotionSim => projectile_motion_sim(),
            CSTemplate::ProjectileMotionSim2 => projectile_motion_2_sim(),
            CSTemplate::BasicSimWithInit(init) => basic_sim_init(init),
        }
    }

    pub fn get_data(self) -> Option<Box<dyn SimulationInit>> {
        match self {
            CSTemplate::BasicSimWithInit(data) => Some(Box::new(data)),
            _ => None,
        }
    }
}

pub const SIM: &[CSTemplate] = &[
    CSTemplate::BasicSim,
    CSTemplate::BasicSimWithInit(BasicSimInit {
        theta: 0.0,
        start_velocity_mul: 10.0,
        mass: 10.0,
    }),
    CSTemplate::ProjectileMotionSim,
    CSTemplate::ProjectileMotionSim2,
];

pub struct CSPreset {
    pub simulation_objects: Vec<CSObject>,
    pub plot_objects: CSPlotObjects,
}

impl Default for CSPreset {
    fn default() -> Self {
        CSPreset {
            simulation_objects: vec![],
            plot_objects: CSPlotObjects::default(),
        }
    }
}

fn basic_sim_init(data: BasicSimInit) -> CSPreset {
    // value have any item
    // let force = value.theta * 5.0;
    // force_list.push(force) // how to?
    let velocity = data.theta.to_radians().sin_cos();
    let velocity = Vector2::new(velocity.0, velocity.1) * data.start_velocity_mul;

    let a = CSObject::new().state(CSObjectState {
        position: NVec2::new(1.0, 0.0),
        momentum: velocity * data.mass,
        mass: data.mass,
        ..CSObjectState::default()
    });

    let func = |state: &CSObjectState, time: f64| {
        if (0.0..=0.1).contains(&state.velocity().norm()) {
            Some(
                CSObjectStampResult::default()
                    .label("WHEN |velocity| < 0.1")
                    .state(state.clone())
                    .time(time),
            )
        } else {
            None
        }
    };

    let stamp = CSObjectStamp::new(func, 0..=0);

    let plot_objects = CSPlotObjects::default().add_stamp(stamp);

    CSPreset {
        simulation_objects: vec![a],
        plot_objects,
        ..CSPreset::default()
    }
}

fn basic_sim() -> CSPreset {
    let a = CSObject::new().state(CSObjectState {
        position: NVec2::new(1.0, 0.0),
        ..CSObjectState::default()
    });

    CSPreset {
        simulation_objects: vec![a],
        ..CSPreset::default()
    }
}

fn projectile_motion_sim() -> CSPreset {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| {
            CSObject::new().state(CSObjectState {
                momentum: NVec2::new(*x, *x).mul(mass),

                mass,
                position: NVec2::new(1.0, 0.0),
                ..CSObjectState::default()
            })
        })
        .collect::<Vec<_>>();

    CSPreset {
        simulation_objects: sim,
        ..CSPreset::default()
    }
}

fn projectile_motion_2_sim() -> CSPreset {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0, 60.0, 100.0]
        .iter()
        .map(|x| {
            CSObject::new().state(CSObjectState {
                momentum: NVec2::new(*x, *x).mul(mass),

                mass,
                position: NVec2::new(1.0, 0.0),
                ..CSObjectState::default()
            })
        })
        .collect::<Vec<_>>();

    let plot_objects = CSPlotObjects::default().add_static_item(|| {
        vec![PlotDrawItem::Line(Line::new(
            PlotPoints::from_explicit_callback(|x| x * x, 0.0..=50.0, 50),
        ))]
    });

    CSPreset {
        simulation_objects: sim,
        plot_objects,
        ..CSPreset::default()
    }
}
