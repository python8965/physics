use crate::app::simulations::object::ClassicSimulationObject;
use crate::app::{Float, NVec2};

pub trait Simulation: Send + Sync {
    fn step(&mut self, dt: Float);

    fn get_children(&mut self) -> &mut Vec<ClassicSimulationObject>;
}

#[derive()]
pub struct ClassicSimulation {
    pub children: Vec<ClassicSimulationObject>,
}

impl From<Vec<ClassicSimulationObject>> for ClassicSimulation {
    fn from(object: Vec<ClassicSimulationObject>) -> Self {
        ClassicSimulation { children: object }
    }
}

impl Simulation for ClassicSimulation {
    fn step(&mut self, dt: f64) {
        for child in &mut self.children {
            physics_system(dt, child);
        }
    }

    fn get_children(&mut self) -> &mut Vec<ClassicSimulationObject> {
        &mut self.children
    }
}

fn physics_system(dt: Float, obj: &mut ClassicSimulationObject) {
    obj.position = {
        let sigma_force: NVec2 = obj
            .force_list
            .iter()
            .fold(NVec2::zeros(), |acc, x| acc + *x); // ΣF

        // ΣF = Δp / Δt
        // 우리는 운동량 p를 원한다
        // Δp = ΣF * Δt

        let delta_momentum = sigma_force * dt;
        obj.momentum += delta_momentum;

        // Δs = v * Δt

        let delta_position = obj.velocity() * dt;

        obj.position + delta_position
    };
}
