pub mod object;
pub mod state;
pub mod template;

use crate::app::Float;

pub use object::CSObject;

pub trait Simulation: Send + Sync {
    fn step(&mut self, dt: Float);

    fn get_children(&mut self) -> &mut Vec<CSObject>;
}

#[derive()]
pub struct ClassicSimulation {
    pub children: Vec<CSObject>,
}

impl From<Vec<CSObject>> for ClassicSimulation {
    fn from(object: Vec<CSObject>) -> Self {
        ClassicSimulation { children: object }
    }
}

impl Simulation for ClassicSimulation {
    fn step(&mut self, dt: f64) {
        for child in &mut self.children {
            physics_system(dt, child);
        }
    }

    fn get_children(&mut self) -> &mut Vec<CSObject> {
        &mut self.children
    }
}

fn physics_system(dt: Float, obj: &mut CSObject) {
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
