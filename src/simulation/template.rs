use crate::simulation::engine::{BasicSim, ItemGetFn, SimState, Simulation};
use crate::simulation::object::DefaultSim;
use crate::simulation::{DrawShapeType, Float, PlotDrawItem, Vec2};
use egui::plot::{Line, PlotPoints};
use nalgebra::Vector2;
use std::ops::Mul;

pub const SIM: &[SimulationType] = &[
    SimulationType::FreeFall,
    SimulationType::ProjectileMotion,
    SimulationType::ProjectileMotion2,
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SimulationType {
    FreeFall,
    ProjectileMotion,
    ProjectileMotion2,
    None,
}

impl Default for SimulationType {
    fn default() -> Self {
        Self::None
    }
}

impl SimulationType {
    pub fn as_str(&self) -> &str {
        match self {
            SimulationType::FreeFall => "BaseSim",
            SimulationType::ProjectileMotion => "ProjectileMotion",
            SimulationType::ProjectileMotion2 => "ProjectileMotion2",
            SimulationType::None => "None",
        }
    }

    pub fn to_simulation(&self) -> Box<dyn Simulation> {
        Box::new(match self {
            SimulationType::ProjectileMotion => projectile_motion_sim(),
            SimulationType::ProjectileMotion2 => projectile_motion_2_sim(),
            SimulationType::FreeFall => basic_sim(),
            SimulationType::None => panic!("None"),
        })
    }
}

fn basic_sim() -> BasicSim {
    let a = DefaultSim {
        mass: 5.0,
        shape: DrawShapeType::Box,
        scale: None,
        force_list: vec![Vector2::new(0.0, -9.8)],
        position: Vector2::new(1.0, 0.0),
        ..DefaultSim::default()
    };

    BasicSim::from(vec![a])
}

fn projectile_motion_sim() -> BasicSim {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| DefaultSim {
            mass,
            shape: DrawShapeType::Box,
            scale: None,
            momentum: Vec2::new(*x, 0.0).mul(mass),
            force_list: vec![Vec2::new(0.0, -9.8)],
            position: Vec2::new(1.0, 0.0),
            ..DefaultSim::default()
        })
        .collect::<Vec<_>>();

    BasicSim::from(sim)
}

fn projectile_motion_2_sim() -> BasicSim {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| DefaultSim {
            mass,
            shape: DrawShapeType::Box,
            scale: None,
            momentum: Vec2::new(*x, *x).mul(mass),
            force_list: vec![Vec2::new(0.0, -9.8)],
            position: Vec2::new(1.0, 0.0),
            ..DefaultSim::default()
        })
        .collect::<Vec<_>>();

    BasicSim::from((sim, projectile_motion_2_static()))
}

fn projectile_motion_2_static() -> ItemGetFn {
    Box::new(|state: SimState| {
        vec![PlotDrawItem::Line(Line::new(
            PlotPoints::from_explicit_callback(|x| x * x, 0.0..=50.0, 50),
        ))]
    })
}
