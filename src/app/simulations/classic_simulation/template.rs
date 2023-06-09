
use nalgebra::Vector2;
use std::ops::IndexMut;

use crate::app::graphics::CSPlotObjects;
use crate::app::simulations::classic_simulation::object::builder::CSimObjectBuilder;
use crate::app::simulations::classic_simulation::object::state::{CSObjectState, ForceIndex};
use crate::app::simulations::classic_simulation::template::init::{
    BasicSimInitObjData, SimulationInit, ThetaThrowSimInit,
};
use crate::app::simulations::classic_simulation::template::stamp::{
    CSObjectStamp, CSObjectStampResult,
};
use crate::app::simulations::classic_simulation::CSimObject;
use crate::app::NVec2;

pub mod init;
pub mod stamp;

#[derive(Clone, Debug)]
pub enum CSTemplate {
    ThetaThrowSimInit(ThetaThrowSimInit),
    DefaultSim,
    CircleSim,
}

impl CSTemplate {
    pub fn get_name(&self) -> String {
        format!("{:?}", self).split('(').collect::<Vec<&str>>()[0].to_string()
    }

    pub fn get_preset_with_ui(self) -> CSPreset {
        match self {
            CSTemplate::DefaultSim => default_sim(),
            CSTemplate::ThetaThrowSimInit(init) => theta_throw(init),
            CSTemplate::CircleSim => circle_sim(),
        }
    }

    pub fn get_data(&self) -> Option<Box<dyn SimulationInit>> {
        match self {
            CSTemplate::ThetaThrowSimInit(data) => Some(Box::new(data.clone())),
            _ => None,
        }
    }
}

pub fn get_sim_list() -> [CSTemplate; 3] {
    [
        CSTemplate::ThetaThrowSimInit(ThetaThrowSimInit {
            objects: vec![
                BasicSimInitObjData {
                    mass: 5.0,
                    theta: 30.0,
                    start_velocity_mul: 20.0,
                },
                BasicSimInitObjData {
                    mass: 5.0,
                    theta: 60.0,
                    start_velocity_mul: 20.0,
                },
                BasicSimInitObjData {
                    mass: 5.0,
                    theta: 15.0,
                    start_velocity_mul: 20.0,
                },
                BasicSimInitObjData {
                    mass: 5.0,
                    theta: 75.0,
                    start_velocity_mul: 20.0,
                },
            ],
        }),
        CSTemplate::DefaultSim,
        CSTemplate::CircleSim,
    ]
}

pub struct CSPreset {
    pub simulation_objects: Vec<CSimObject>,
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

fn theta_throw(data: ThetaThrowSimInit) -> CSPreset {
    // value have any item
    // let force = value.theta * 5.0;
    // force_list.push(force) // how to?
    let objects = data
        .objects
        .iter()
        .map(|obj| {
            let velocity = obj.theta.to_radians().sin_cos();
            let velocity = Vector2::new(velocity.0, velocity.1) * obj.start_velocity_mul;

            CSimObjectBuilder::new(CSObjectState {
                velocity,
                mass: obj.mass,
                ..CSObjectState::default()
            })
            .build()
        })
        .collect::<Vec<_>>();

    let func = |state: &CSObjectState, time: f64| {
        if state.position.y < 0.0 {
            Some(
                CSObjectStampResult::default()
                    .label("WHEN pos < 0.1")
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
        simulation_objects: objects,
        plot_objects,
        ..CSPreset::default()
    }
}

fn circle_sim() -> CSPreset {
    let mass = 5.0;

    let sim = vec![5.0]
        .iter()
        .map(|x| {
            CSimObjectBuilder::new(CSObjectState {
                velocity: NVec2::new(*x, *x),

                mass,
                position: NVec2::new(1.0, 0.0),

                ..CSObjectState::default()
            })
            .attached(|obj| {
                let _ = std::mem::replace(obj.acc_list.index_mut(ForceIndex::Attached as usize), {
                    let mut vector = obj.velocity.yx();
                    vector.y *= -1.0;
                    vector
                });
            })
            .build()
        })
        .collect::<Vec<_>>();

    CSPreset {
        simulation_objects: sim,

        ..CSPreset::default()
    }
}

fn default_sim() -> CSPreset {
    CSPreset::default()
}
