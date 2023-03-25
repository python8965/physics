use crate::app::graphics::{DrawShapeType, PlotDrawItem};
use crate::app::simulations::object::ClassicSimulationObject;
use crate::app::simulations::state::SimulationState;
use crate::app::NVec2;
use egui::plot::{Line, PlotPoints};
use nalgebra::Vector2;
use std::ops::Mul;

#[derive(Clone, Copy, Debug)]
pub enum ClassicSimulationType {
    BasicSim,
    ProjectileMotionSim,
    ProjectileMotionSim2,
}

impl ClassicSimulationType {
    pub fn get_name(&self) -> String {
        format!("{:?}", self)
    }

    pub fn get_preset(self) -> ClassicSimulationPreset {
        match self {
            ClassicSimulationType::BasicSim => basic_sim(),
            ClassicSimulationType::ProjectileMotionSim => projectile_motion_sim(),
            ClassicSimulationType::ProjectileMotionSim2 => projectile_motion_2_sim(),
        }
    }
}

pub const SIM: &[ClassicSimulationType] = &[
    ClassicSimulationType::BasicSim,
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

fn basic_sim() -> ClassicSimulationPreset {
    let a = ClassicSimulationObject {
        mass: 5.0,
        shape: DrawShapeType::Box,
        scale: None,
        force_list: vec![Vector2::new(0.0, -9.8)],
        position: Vector2::new(1.0, 0.0),
        ..ClassicSimulationObject::default()
    };

    ClassicSimulationPreset::new(vec![a], vec![])
}

fn projectile_motion_sim() -> ClassicSimulationPreset {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| ClassicSimulationObject {
            mass,
            shape: DrawShapeType::Box,
            scale: None,
            momentum: NVec2::new(*x, 0.0).mul(mass),
            force_list: vec![NVec2::new(0.0, -9.8)],
            position: NVec2::new(1.0, 0.0),
            ..ClassicSimulationObject::default()
        })
        .collect::<Vec<_>>();

    ClassicSimulationPreset::new(sim, vec![])
}

fn projectile_motion_2_sim() -> ClassicSimulationPreset {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| ClassicSimulationObject {
            mass,
            shape: DrawShapeType::Box,
            scale: None,
            momentum: NVec2::new(*x, *x).mul(mass),
            force_list: vec![NVec2::new(0.0, -9.8)],
            position: NVec2::new(1.0, 0.0),
            ..ClassicSimulationObject::default()
        })
        .collect::<Vec<_>>();

    let graph = Box::new(|_state: SimulationState| {
        vec![PlotDrawItem::Line(Line::new(
            PlotPoints::from_explicit_callback(|x| x * x, 0.0..=50.0, 50),
        ))]
    });

    ClassicSimulationPreset::new(sim, vec![graph])
}
