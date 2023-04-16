pub mod object;
pub mod state;
pub mod template;

use crate::app::{Float, NVec2};
use egui::plot::PlotPoint;
use egui::{Response, Ui};
use nalgebra::{vector, SMatrix};
use tracing::info;

use crate::app::graphics::plot::{InputMessage, PlotData};
use crate::app::simulations::classic_simulation::object::drawing::get_object_mesh;
use crate::app::simulations::classic_simulation::object::{
    CSObjectState, CSObjectStateHistory, ForceIndex,
};

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
    fn inspection_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("No inspection UI");
    }

    fn operation_ui(&mut self, ui: &mut egui::Ui) {
        ui.label("No operations UI");
    }

    fn input(
        &mut self,
        plot: &mut PlotData,
        input_msg: InputMessage,
        response: egui::Response,
        ctx: &egui::Context,
        state: &mut SimulationState,
    );

    fn step(&mut self, dt: Float, state: &mut SimulationState);

    fn at_time_step(&mut self, step: usize);

    fn get_children(&self) -> &Vec<CSimObject>;

    fn init(&mut self);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operation {
    #[default]
    Navigate,
    ForceDrag,
    AddObject,
    RemoveObject,
    EditObject,
}

const OPERATION_ITER: [Operation; 5] = [
    Operation::Navigate,
    Operation::ForceDrag,
    Operation::AddObject,
    Operation::RemoveObject,
    Operation::EditObject,
];

#[derive()]
pub struct ClassicSimulation {
    pub objects: Vec<CSimObject>,
    pub global_acc_list: Vec<NVec2>,
    pub operation: Operation,
}

impl From<Vec<CSimObject>> for ClassicSimulation {
    fn from(object: Vec<CSimObject>) -> Self {
        ClassicSimulation {
            objects: object,
            global_acc_list: vec![GRAVITY],
            operation: Operation::default(),
        }
    }
}

impl Simulation for ClassicSimulation {
    fn inspection_ui(&mut self, ui: &mut Ui) {
        for child in &mut self.objects {
            child.inspection_ui(ui);
        }
    }

    fn operation_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("Operations");
            ui.separator();
            ui.horizontal(|ui| {
                OPERATION_ITER.iter().for_each(|operation| {
                    ui.selectable_value(
                        &mut self.operation,
                        *operation,
                        format!("{:?}", operation),
                    );
                });
            });
        });
    }

    fn input(
        &mut self,
        plot: &mut PlotData,
        msg: InputMessage,
        response: Response,
        ctx: &egui::Context,
        state: &mut SimulationState,
    ) {
        let simulation_objects = &mut self.objects;

        match self.operation {
            Operation::Navigate => {
                if let Some(pointer_pos) = msg.pointer_pos {
                    if response.clicked() {
                        for (index, obj) in simulation_objects.iter().enumerate() {
                            if is_inside(pointer_pos, get_object_mesh(obj).points()) {
                                plot.selected_index = index;
                                break;
                            }
                        }
                    }
                }
            }
            Operation::ForceDrag => {
                if let Some(pointer_pos) = msg.pointer_pos {
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
                }

                if !response.dragged() && plot.dragging_object {
                    let selected = &mut simulation_objects[plot.selected_index];

                    selected.state.acc_list[ForceIndex::UserInteraction as usize] = ZERO_FORCE;

                    plot.dragging_object = false;
                }
            }
            Operation::AddObject => {
                if let Some(pointer_pos) = msg.pointer_pos {
                    if msg.clicked {
                        simulation_objects.push(CSimObject {
                            state: CSObjectState {
                                position: vector![pointer_pos.x, pointer_pos.y],
                                velocity: vector![0.0, 0.0],
                                ..CSObjectState::default()
                            },
                            init_timestep: state.current_step,
                            ..CSimObject::default()
                        });
                        info!("Clicked at {:?}", pointer_pos);
                    }
                }
            }
            Operation::RemoveObject => {}
            Operation::EditObject => {}
        }
    }

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
            if let Some(state) = obj.state_at_timestep(step) {
                obj.hide = false;
                obj.state = state;
            } else {
                obj.hide = true;
            }
        }
    }

    fn get_children(&self) -> &Vec<CSimObject> {
        &self.objects
    }

    fn init(&mut self) {}
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
