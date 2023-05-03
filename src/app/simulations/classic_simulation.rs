pub mod object;
pub mod sim_state;
pub mod template;

use crate::app::NVec2;

use egui::{Response, Ui};
use nalgebra::{vector, SMatrix};
use tracing::info;

use crate::app::graphics::plot::{InputMessage, PlotData};
use crate::app::manager::SIMULATION_TICK;
use crate::app::simulations::polygon::is_inside;
use crate::app::simulations::state::SimulationState;

use self::object::builder::CSimObjectBuilder;
use self::object::state::{CSObjectState, ForceIndex};
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

    fn step(&mut self, state: &mut SimulationState);

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
        for (i, child) in self.objects.iter_mut().enumerate() {
            ui.push_id(i, |ui| {
                ui.collapsing(format!("Object {}", i), |ui| {
                    child.inspection_ui(ui);
                });
            });
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
        _ctx: &egui::Context,
        state: &mut SimulationState,
    ) {
        let simulation_objects = &mut self.objects;
        match self.operation {
            Operation::Navigate => {
                if let Some(pointer_pos) = msg.pointer_pos {
                    if response.clicked() {
                        for (index, obj) in simulation_objects.iter().enumerate() {
                            if let Some(obj_state) = obj.state_at_timestep(state.current_step) {
                                if is_inside(pointer_pos, obj_state.shape.get_points()) {
                                    plot.selected_index = index;
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Operation::ForceDrag => {
                if let Some(pointer_pos) = msg.pointer_pos {
                    if response.drag_started() {
                        for (index, obj) in simulation_objects.iter().enumerate() {
                            if let Some(obj_state) = obj.state_at_timestep(state.current_step) {
                                if is_inside(pointer_pos, obj_state.shape.get_points()) {
                                    plot.selected_index = index;
                                    plot.dragging_object = true;
                                    break;
                                }
                            }
                        }
                    }

                    if response.dragged() && plot.dragging_object {
                        let pos = simulation_objects[plot.selected_index]
                            .current_state()
                            .position;
                        let selected = &mut simulation_objects[plot.selected_index];

                        selected.current_state_mut().acc_list
                            [ForceIndex::UserInteraction as usize] =
                            vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y];
                    }
                }

                if !response.dragged() && plot.dragging_object {
                    let selected = &mut simulation_objects[plot.selected_index];

                    selected.current_state_mut().acc_list[ForceIndex::UserInteraction as usize] =
                        ZERO_FORCE;

                    plot.dragging_object = false;
                }
            }
            Operation::AddObject => {
                if let Some(pointer_pos) = msg.pointer_pos {
                    if msg.clicked {
                        simulation_objects.push(
                            CSimObjectBuilder::new(CSObjectState {
                                position: vector![pointer_pos.x, pointer_pos.y],
                                ..CSObjectState::default()
                            })
                            .at(state.current_step)
                            .build(),
                        );
                    }
                }
            }
            Operation::RemoveObject => {}
            Operation::EditObject => {}
        }
    }

    fn step(&mut self, state: &mut SimulationState) {
        puffin::profile_scope!("ClassicSimulation::step");

        if let Some(settings) = state.settings.specific.as_c_sim_settings_mut() {
            if let Some(is_grav) = settings.gravity.get() {
                if is_grav {
                    self.global_acc_list[GlobalForceSlot::Gravity as usize] = GRAVITY;
                } else {
                    self.global_acc_list[GlobalForceSlot::Gravity as usize] = ZERO_FORCE;
                }
            }
        }

        let length = self.objects.len();

        for i in 1..length + 1 {
            let (front, end) = self.objects.split_at_mut(i - 1);

            let Some((obj, rest)) = end.split_first_mut() else {panic!("Cannot Reach")};

            let obj_state = &mut obj.current_state();

            obj.update(state);

            if let Some(attached_fn) = &obj.attached() {
                attached_fn(obj.current_state_mut());
            }

            physics_system(obj, self.global_acc_list.iter().sum());

            for obj2 in rest {
                let obj2_state = &mut obj2.current_state();

                if let Some(contact) = obj_state.shape.contact(
                    obj_state.position,
                    &obj2_state.shape,
                    obj2_state.position,
                ) {
                    info!("{:?}", contact);
                }
            }

            obj.save_state();
        }
    }

    fn at_time_step(&mut self, step: usize) {
        for obj in self.objects.iter_mut() {
            obj.at_timestep(step);
        }
    }

    fn get_children(&self) -> &Vec<CSimObject> {
        &self.objects
    }

    fn init(&mut self) {}
}

//noinspection ALL
#[allow(non_snake_case)]
fn physics_system(obj: &mut CSimObject, global_acc: NVec2) {
    let last_state = obj.last_state().unwrap();
    let current_state = obj.current_state();
    let dt = SIMULATION_TICK;

    obj.current_state_mut().position = {
        // ΣF
        // ΣF = ma
        // a = ΣF / m
        // Δv = a * Δt
        // Δp = ΣF * Δt
        // Δs = v * Δt

        let current_acc = current_state.acceleration();

        let Σa = current_acc + global_acc; // Σa
        let Δa = current_acc - last_state.acceleration();

        let Δv = Σa * dt; // 등가속도 운동에서의 보정.
        let Δv_error = (Δa * dt) / 2.0;
        let Δv = Δv + Δv_error;

        let v = current_state.velocity;

        let Δs = v * dt;
        let Δs_error = (Δv * dt) / 2.0; // 등가속도 운동에서의 보정.
        let Δs = Δs + Δs_error;
        // Δs = v * Δt

        obj.current_state_mut().last_velocity = current_state.velocity;

        obj.current_state_mut().velocity += Δv;

        obj.current_state_mut().position + Δs
    };
}
