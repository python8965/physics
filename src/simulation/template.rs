use crate::simulation::engine::{BasicSim, Simulation};
use crate::simulation::object::SimulationObject;
use crate::simulation::{DrawShapeType, Vec2};
use nalgebra::Vector2;
use std::ops::Mul;

pub const SIM: &[SimulationType] = &[SimulationType::FreeFall, SimulationType::ProjectileMotion];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SimulationType {
    FreeFall,
    ProjectileMotion,
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
            SimulationType::None => "None",
        }
    }

    pub fn as_func(&self) -> Box<dyn Simulation> {
        Box::new(match self {
            SimulationType::ProjectileMotion => projectile_motion_sim(),
            SimulationType::FreeFall => basic_sim(),
            SimulationType::None => panic!("None"),
        })
    }
}

fn basic_sim() -> BasicSim {
    let a = SimulationObject {
        mass: 5.0,
        shape: DrawShapeType::Box,
        scale: None,
        force_list: vec![Vector2::new(0.0, -9.8)],
        position: Vector2::new(1.0, 0.0),
        ..SimulationObject::default()
    };

    BasicSim::from(vec![a])
}

fn projectile_motion_sim() -> BasicSim {
    let mass = 5.0;

    let sim = vec![2.0, 8.0, 20.0, 30.0, 40.0]
        .iter()
        .map(|x| SimulationObject {
            mass,
            shape: DrawShapeType::Box,
            scale: None,
            momentum: Vec2::new(*x, 0.0).mul(mass),
            force_list: vec![Vec2::new(0.0, -9.8)],
            position: Vec2::new(1.0, 0.0),
            ..SimulationObject::default()
        })
        .collect::<Vec<_>>();

    BasicSim::from(sim)
}
