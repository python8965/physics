pub mod object;
pub mod state;
pub mod template;

use crate::app::Float;
use tracing::info;

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

//noinspection ALL
#[allow(non_snake_case)]
fn physics_system(Δt: Float, obj: &mut CSObject) {
    info!("{:?}", obj.state.position);

    obj.state.position = {
        // ΣF
        // ΣF = ma
        // a = ΣF / m
        // Δv = a * Δt
        // Δp = ΣF * Δt
        // Δs = v * Δt

        let Σa = obj.state.acceleration(); // Σa

        let Δv = Σa * Δt;

        let v = obj.state.velocity;

        let Δs = v * Δt;
        let Δs_error = (Δv * Δt) / 2.0; // 등가속도 운동에서의 보정.
        let Δs = Δs + Δs_error;
        info!("Δs: {:?}", Δs);
        // Δs = v * Δt

        obj.state.velocity += Δv;

        obj.state.position + Δs
    };
}
