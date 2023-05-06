pub mod object;
pub mod sim_state;
pub mod template;

use crate::app::NVec2;
use std::iter::Sum;
use std::ops::Add;

use egui::plot::PlotPoint;
use egui::{Response, Ui};
use nalgebra::{vector, SMatrix};
use tracing::info;

use crate::app::graphics::plot::{InputMessage, PlotData};
use crate::app::manager::SIMULATION_TICK;
use crate::app::simulations::polygon::is_inside;
use crate::app::simulations::state::SimulationState;

use self::object::builder::CSimObjectBuilder;
use self::object::state::{CSObjectState, ForceIndex};
use crate::app::simulations::classic_simulation::object::state::Collision;
pub use object::CSimObject;

pub const GRAVITY: SMatrix<f64, 2, 1> = vector![0.0, -9.8];
pub const ZERO_FORCE: SMatrix<f64, 2, 1> = vector![0.0, 0.0];

#[repr(usize)]
pub enum GlobalForceSlot {
    Gravity = 0,
    MAX = 1,
}

pub trait Simulation: Send + Sync {
    fn inspection_ui(&mut self, ui: &mut Ui) {
        ui.label("No inspection UI");
    }

    fn operation_ui(&mut self, ui: &mut Ui) {
        ui.label("No operations UI");
    }

    fn input(
        &mut self,
        plot: &mut PlotData,
        input_msg: InputMessage,
        response: Response,
        ctx: &egui::Context,
        state: &mut SimulationState,
    );

    fn step(&mut self, state: &mut SimulationState);

    fn at_time_step(&mut self, step: usize);

    fn get_children(&self) -> &Vec<CSimObject>;
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
        //TODO: 모바일 환경에서의 터치도 감지하기.
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
                    if response.dragged() {
                        if plot.dragging_object {
                            // 드래그 중일 때
                            let pos = simulation_objects[plot.selected_index]
                                .current_state()
                                .position;

                            let selected = &mut simulation_objects[plot.selected_index];

                            let user_vec = vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y];
                            selected.current_state_mut().acc_list
                                [ForceIndex::UserInteraction as usize] = user_vec;

                            dbg!(
                                selected.current_state().acc_list
                                    [ForceIndex::UserInteraction as usize]
                            );

                            dbg!(
                                selected.current_state().acc_list.len(),
                                selected.local_timestep(0)
                            );
                        } else {
                            // 드래그 시작할 때
                            for (index, obj) in simulation_objects.iter().enumerate() {
                                let obj_state = obj.current_state();

                                {
                                    if is_inside(
                                        pointer_pos,
                                        obj_state
                                            .shape
                                            .get_points()
                                            .into_iter()
                                            .map(|a| {
                                                let b = obj_state.position;
                                                PlotPoint::new(a[0] + b.x, a[1] + b.y)
                                            })
                                            .collect::<Vec<_>>(),
                                    ) {
                                        plot.selected_index = index;
                                        plot.dragging_object = true;
                                        break;
                                    }
                                }
                            }
                        }
                    } else {
                        // 드래그 중이 아닐 때
                    }
                }

                if !response.dragged() && plot.dragging_object {
                    // 드래그가 끝났을 때
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

        //TODO: 이거 더 좋은 방법 없나?
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

        //충돌 처리 부분
        for i in 1..length + 1 {
            let (_front, end) = self.objects.split_at_mut(i - 1);

            let Some((obj, rest)) = end.split_first_mut() else {panic!("Cannot Reach")};

            for obj2 in rest {
                collision(obj, obj2);
            }
        }

        //물리 처리 부분
        for obj in self.objects.iter_mut() {
            if let Some(attached_fn) = &obj.attached() {
                attached_fn(obj.current_state_mut());
            }

            physics(obj, &self.global_acc_list);
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
}

fn collision(obj: &mut CSimObject, obj2: &mut CSimObject) {
    let obj_state = obj.current_state_mut();
    let obj2_state = obj2.current_state_mut();

    if let Some(contact) = obj_state.contact(obj2_state) {
        let obj_vel = contact.contact_momentum / obj_state.mass;
        let obj2_vel = contact.contact_momentum / obj2_state.mass;

        obj_state.velocity += obj_vel * contact.contact_normal;
        obj2_state.velocity += obj2_vel * -contact.contact_normal;
        obj_state.position += contact.penetration * contact.contact_normal;
        obj2_state.position += contact.penetration * -contact.contact_normal;

        dbg!(obj_vel, contact.contact_normal);

        info!("{:?}", obj_vel * contact.contact_normal);
    }
}

//
fn physics(obj: &mut CSimObject, global_acc_list: &[NVec2]) {
    // Physics
    let global_acc: NVec2 = global_acc_list.iter().sum();
    let state = obj.current_state_mut();
    let dt = SIMULATION_TICK;

    // ΣF
    // ΣF = ma
    // a = ΣF / m
    // Δv = a * Δt
    // Δp = ΣF * Δt
    // Δs = v * Δt

    {
        let current_acc = state.acceleration();

        let sum_acc = current_acc + global_acc; // Σa
                                                // let Δa = current_acc - last_state.acceleration();

        let delta_v = sum_acc * dt; // 등가속도 운동에서의 보정.
                                    // let Δv_error = (Δa * dt) / 2.0;
                                    // let Δv = Δv + Δv_error;

        let v = state.velocity;

        let delta_pos = v * dt;
        let dpos_error = (delta_v * dt) / 2.0; // 등가속도 운동에서의 보정.
        let delta_pos = delta_pos + dpos_error;
        // Δs = v * Δt

        state.last_velocity = state.velocity;
        state.velocity += delta_v;
        state.position += delta_pos
    }
}
