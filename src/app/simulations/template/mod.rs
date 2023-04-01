use std::ops::Mul;

use egui::plot::{Line, PlotPoints};
use nalgebra::Vector2;

use crate::app::graphics::define::{PlotDrawItem};
use crate::app::simulations::object::{
    ClassicSimulationObject, ClassicSimulationObjectBuilder, ObjectState,
};
use crate::app::simulations::state::SimulationState;
use crate::app::simulations::template::init::{BasicSimInit, SimInit};
use crate::app::NVec2;

pub mod init;

#[derive(Clone, Copy, Debug)]
pub enum ClassicSimulationType {
    BasicSim,
    BasicSimWithInit(BasicSimInit),
    ProjectileMotionSim,
    ProjectileMotionSim2,
}

impl ClassicSimulationType {
    pub fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn get_preset_with_ui(self) -> ClassicSimulationPreset {
        match self {
            ClassicSimulationType::BasicSim => basic_sim(),
            ClassicSimulationType::ProjectileMotionSim => projectile_motion_sim(),
            ClassicSimulationType::ProjectileMotionSim2 => projectile_motion_2_sim(),
            ClassicSimulationType::BasicSimWithInit(init) => basic_sim_init(init),
        }
    }

    pub fn get_data(self) -> Option<Box<dyn SimInit>> {
        match self {
            ClassicSimulationType::BasicSimWithInit(data) => Some(Box::new(data)),
            _ => None,
        }
    }
}

pub const SIM: &[ClassicSimulationType] = &[
    ClassicSimulationType::BasicSim,
    ClassicSimulationType::BasicSimWithInit(BasicSimInit {
        theta: 0.0,
        start_velocity_mul: 10.0,
        mass: 10.0,
    }),
    ClassicSimulationType::ProjectileMotionSim,
    ClassicSimulationType::ProjectileMotionSim2,
];

pub type PlotObjectFnVec = Vec<Box<dyn Fn(SimulationState) -> Vec<PlotDrawItem> + Sync + Send>>;

pub struct ClassicSimulationPreset {
    pub simulation_objects: Vec<ClassicSimulationObject>,
    pub objects_fn: PlotObjectFnVec,
}

impl ClassicSimulationPreset {
    fn new(sim_obj: Vec<ClassicSimulationObject>, obj: PlotObjectFnVec) -> ClassicSimulationPreset {
        ClassicSimulationPreset {
            simulation_objects: sim_obj,
            objects_fn: obj,
        }
    }
}

fn basic_sim_init(data: BasicSimInit) -> ClassicSimulationPreset {
    // value have any item
    // let force = value.theta * 5.0;
    // force_list.push(force) // how to?
    let velocity = data.theta.to_radians().sin_cos();
    let velocity = Vector2::new(velocity.0, velocity.1) * data.start_velocity_mul;

    let a = ClassicSimulationObjectBuilder::new()
        .state(ObjectState {
            position: NVec2::new(1.0, 0.0),
            momentum: velocity * data.mass,
            mass: data.mass,
            ..ObjectState::default()
        })
        .get();

    ClassicSimulationPreset::new(vec![a], vec![])
}

fn basic_sim() -> ClassicSimulationPreset {
    let a = ClassicSimulationObjectBuilder::new()
        .state(ObjectState {
            position: NVec2::new(1.0, 0.0),
            ..ObjectState::default()
        })
        .get();

    ClassicSimulationPreset::new(vec![a], vec![])
}

fn projectile_motion_sim() -> ClassicSimulationPreset {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| {
            ClassicSimulationObjectBuilder::new()
                .state(ObjectState {
                    momentum: NVec2::new(*x, *x).mul(mass),

                    mass,
                    position: NVec2::new(1.0, 0.0),
                    ..ObjectState::default()
                })
                .get()
        })
        .collect::<Vec<_>>();

    ClassicSimulationPreset::new(sim, vec![])
}

fn projectile_motion_2_sim() -> ClassicSimulationPreset {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| {
            ClassicSimulationObjectBuilder::new()
                .state(ObjectState {
                    momentum: NVec2::new(*x, *x).mul(mass),

                    mass,
                    position: NVec2::new(1.0, 0.0),
                    ..ObjectState::default()
                })
                .get()
        })
        .collect::<Vec<_>>();

    let graph = Box::new(|_state: SimulationState| {
        vec![PlotDrawItem::Line(Line::new(
            PlotPoints::from_explicit_callback(|x| x * x, 0.0..=50.0, 50),
        ))]
    });

    ClassicSimulationPreset::new(sim, vec![graph])
}
