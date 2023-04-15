pub mod object;
pub mod state;
pub mod template;

use crate::app::{Float, NVec2};
use egui::plot::PlotPoint;
use egui::{Response, Ui};
use nalgebra::{vector, SMatrix};

use crate::app::graphics::plot::PlotData;
use crate::app::simulations::classic_simulation::object::drawing::get_object_mesh;
use crate::app::simulations::classic_simulation::object::{CSObjectStateHistory, ForceIndex};

use crate::app::simulations::polygon::is_inside;
use crate::app::simulations::state::SimulationState;
pub use object::CSimObject;

pub const GRAVITY: SMatrix<f64, 2, 1> = vector![0.0, -9.8];
pub const ZERO_FORCE: SMatrix<f64, 2, 1> = vector![0.0, 0.0];

#[repr(usize)]
pub enum GlobalForceSlot {
    Gravity = 0,
    MAX = 1,
}

pub trait Simulation: Send + Sync {
    fn step(&mut self, dt: Float, state: &mut SimulationState);

    fn at_time_step(&mut self, step: usize);

    fn get_children(&self) -> &Vec<CSimObject>;

    fn input(&mut self, plot: &mut PlotData, pointer_pos: PlotPoint, response: egui::Response);

    fn init(&mut self);

    fn inspection_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("No inspection UI");
    }
}

#[derive()]
pub struct ClassicSimulation {
    pub objects: Vec<CSimObject>,
    pub global_acc_list: Vec<NVec2>,
}

impl From<Vec<CSimObject>> for ClassicSimulation {
    fn from(object: Vec<CSimObject>) -> Self {
        ClassicSimulation {
            objects: object,
            global_acc_list: vec![GRAVITY],
        }
    }
}

impl Simulation for ClassicSimulation {
    fn step(&mut self, dt: f64, state: &mut SimulationState) {
        if let Some(settings) = state.settings.as_c_sim_settings_mut() {
            if let Some(is_grav) = settings.gravity.get() {
                if is_grav {
                    self.global_acc_list[GlobalForceSlot::Gravity as usize] = GRAVITY;
                } else {
                    self.global_acc_list[GlobalForceSlot::Gravity as usize] = ZERO_FORCE;
                }
            }
        }

        for child in &mut self.objects {
            child.update(dt, state);

            child
                .state_history
                .push(CSObjectStateHistory::new(child.state.clone(), dt));

            if let Some(attached_fn) = &child.attached {
                attached_fn(&mut child.state, dt);
            }

            physics_system(dt, child, self.global_acc_list.iter().sum());
        }
    }

    fn at_time_step(&mut self, step: usize) {
        for obj in self.objects.iter_mut() {
            obj.state = obj.state_at_step(step);
        }
    }

    fn get_children(&self) -> &Vec<CSimObject> {
        &self.objects
    }

    fn input(&mut self, plot: &mut PlotData, pointer_pos: PlotPoint, response: Response) {
        let simulation_objects = &mut self.objects;
        if response.clicked() {
            for (index, obj) in simulation_objects.iter().enumerate() {
                if is_inside(pointer_pos, get_object_mesh(obj).points()) {
                    plot.selected_index = index;
                    break;
                }
            }
        }

        if response.drag_started() {
            for (index, obj) in simulation_objects.iter().enumerate() {
                if is_inside(pointer_pos, get_object_mesh(obj).points()) {
                    plot.selected_index = index;
                    plot.dragging_object = true;
                    break;
                }
            }
        }

        if response.dragged() && plot.dragging_object {
            let pos = simulation_objects[plot.selected_index].state.position;
            let selected = &mut simulation_objects[plot.selected_index];

            selected.state.acc_list[ForceIndex::UserInteraction as usize] =
                vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y];
        }

        if !response.dragged() && plot.dragging_object {
            let selected = &mut simulation_objects[plot.selected_index];

            selected.state.acc_list[ForceIndex::UserInteraction as usize] = ZERO_FORCE;

            plot.dragging_object = false;
        }
    }

    fn init(&mut self) {
        self.objects.iter_mut().for_each(|obj| {
            obj.init();
        });
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
fn physics_system(Δt: Float, obj: &mut CSimObject, global_acc: NVec2) {
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
