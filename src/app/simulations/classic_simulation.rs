use crate::app::simulations::object::ClassicSimulationObject;
use crate::app::{Float};


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
    obj.state.position = {
        // ΣF
        // ΣF = ma
        // a = ΣF / m
        // Δv = a * Δt
        // Δp = ΣF * Δt
        // Δs = v * Δt
        let sigma_force = obj.state.sigma_force(); // a

        let delta_momentum = sigma_force * dt;

        obj.state.momentum += delta_momentum;
        // Δs = v * Δt

        let delta_position = obj.state.velocity() * dt;
        obj.state.position + delta_position
    };
}
