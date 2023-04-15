pub mod object;
pub mod state;
pub mod template;

use crate::app::{Float, NVec2};
use egui::Ui;
use nalgebra::{vector, SMatrix};


use crate::app::simulations::classic_simulation::object::CSObjectStateHistory;
pub use object::CSObject;

pub const GRAVITY: SMatrix<f64, 2, 1> = vector![0.0, -9.8];
pub const ZERO_FORCE: SMatrix<f64, 2, 1> = vector![0.0, 0.0];

#[repr(usize)]
pub enum GlobalForceSlot {
    Gravity = 0,
    MAX = 1,
}

pub trait Simulation: Send + Sync {
    fn step(&mut self, dt: Float);

    fn get_children(&mut self) -> &mut Vec<CSObject>;

    fn set_global_force(&mut self, index: GlobalForceSlot, force: NVec2);

    fn inspection_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("No inspection UI");
    }
}

#[derive()]
pub struct ClassicSimulation {
    pub objects: Vec<CSObject>,
    pub global_acc_list: Vec<NVec2>,
}

impl From<Vec<CSObject>> for ClassicSimulation {
    fn from(object: Vec<CSObject>) -> Self {
        ClassicSimulation {
            objects: object,
            global_acc_list: vec![GRAVITY],
        }
    }
}

impl Simulation for ClassicSimulation {
    fn step(&mut self, dt: f64) {
        for child in &mut self.objects {
            child
                .state_history
                .push(CSObjectStateHistory::new(child.state.clone(), dt));

            if let Some(attached_fn) = &child.attached {
                attached_fn(&mut child.state, dt);
            }

            physics_system(dt, child, self.global_acc_list.iter().sum());
        }
    }

    fn get_children(&mut self) -> &mut Vec<CSObject> {
        &mut self.objects
    }

    fn set_global_force(&mut self, index: GlobalForceSlot, force: NVec2) {
        self.global_acc_list[index as usize] = force;
    }

    fn inspection_ui(&mut self, ui: &mut Ui) {
        for i in self.objects.iter_mut().enumerate() {
            let (index, obj) = i;
            ui.collapsing(format!("Object {}", index), |ui| {
                ui.push_id(index, |ui| {
                    obj.inspection_ui(ui);
                });
            });
        }
    }
}

//noinspection ALL
#[allow(non_snake_case)]
fn physics_system(Δt: Float, obj: &mut CSObject, global_acc: NVec2) {
    let last_obj_state = obj.state_history.last().unwrap().state.clone();
    let last_dt = obj.state_history.last().unwrap().dt;

    obj.state.position = {
        // ΣF
        // ΣF = ma
        // a = ΣF / m
        // Δv = a * Δt
        // Δp = ΣF * Δt
        // Δs = v * Δt
        let current_acc = obj.state.acceleration();

        let Σa = current_acc + global_acc; // Σa
        let Δa = current_acc - last_obj_state.acceleration();

        let Δv = Σa * Δt; // 등가속도 운동에서의 보정.
        let Δv_error = (Δa * last_dt) / 2.0;
        let Δv = Δv + Δv_error;

        let v = obj.state.velocity;

        let Δs = v * Δt;
        let Δs_error = (Δv * last_dt) / 2.0; // 등가속도 운동에서의 보정.
        let Δs = Δs + Δs_error;
        // Δs = v * Δt

        obj.state.last_velocity = obj.state.velocity;

        obj.state.velocity += Δv;

        obj.state.position + Δs
    };
}
