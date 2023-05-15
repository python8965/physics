use crate::app::graphics::define::PlotItem;
use crate::app::simulations::classic_simulation::object::state::{CSObjectState};
use crate::app::NVec2;
use egui::plot::Arrows;
use egui::CollapsingHeader;

pub struct SimulationEvents(Vec<SimulationEvent>);

impl Default for SimulationEvents {
    fn default() -> Self {
        Self(vec![])
    }
}

impl SimulationEvents {
    pub fn add_events(&mut self, events: Vec<impl Into<SimulationEvent>>) {
        self.0.extend(events.into_iter().map(|x| x.into()));
    }

    pub fn add_event(&mut self, event: impl Into<SimulationEvent>) {
        self.0.push(event.into());
    }

    pub fn get_shapes(&self) -> Vec<PlotItem> {
        self.0.iter().fold(vec![], |mut acc, x| {
            acc.extend(x.get_shapes());
            acc
        })
    }

    pub fn inspection_ui(&self, ui: &mut egui::Ui) {
        self.0.iter().enumerate().for_each(|(i, x)| {
            CollapsingHeader::new(format!("CollisionEvent, {:?}", i))
                .default_open(false)
                .show(ui, |ui| {
                    ui.push_id(i, |ui| x.inspection_ui(ui));
                });
        });
    }
}

impl Into<SimulationEvent> for CollisionEvent {
    fn into(self) -> SimulationEvent {
        SimulationEvent::Collision(self)
    }
}

pub enum SimulationEvent {
    Collision(CollisionEvent),
}

impl SimulationEvent {
    pub fn get_shapes(&self) -> Vec<PlotItem> {
        match self {
            Self::Collision(event) => event.get_shapes().into_iter().map(|x| x.into()).collect(),
        }
    }

    pub fn inspection_ui(&self, ui: &mut egui::Ui) {
        match self {
            Self::Collision(event) => {
                ui.label(format!("penetration: {:?}", event.penetration));
                ui.label(format!("contact_point: {:?}", event.contact_point));
                ui.label(format!("contact_normal: {:?}", event.contact_normal));
                ui.label(format!("obj1_velocity: {:?}", event.obj1_velocity));
                ui.label(format!("obj2_velocity: {:?}", event.obj2_velocity));
                ui.label(format!("obj1_state: {:?}", event.obj1_state));
                ui.label(format!("obj2_state: {:?}", event.obj2_state));
            }
        }
    }
}

pub struct CollisionEvent {
    pub contact_point: NVec2,
    pub contact_normal: NVec2,
    pub obj1_velocity: NVec2,
    pub obj2_velocity: NVec2,
    pub obj1_state: CSObjectState,
    pub obj2_state: CSObjectState,
    pub penetration: f64,
}

impl CollisionEvent {
    pub fn get_shapes(&self) -> Vec<impl Into<PlotItem>> {
        let obj1_pos = self.obj1_state.position;
        let obj2_pos = self.obj2_state.position;

        let obj2_velocity = self.obj2_velocity;
        let obj1_velocity = self.obj1_velocity;

        let contact_normal = self.contact_normal;

        let c = |a: NVec2| a.data.0[0];

        vec![
            Arrows::new(
                vec![c(obj2_pos)],
                vec![c(obj2_pos + (-contact_normal * 10.0))],
            )
            .name("contact_normal"),
            Arrows::new(vec![c(obj1_pos)], vec![c(obj1_velocity + obj1_pos)])
                .name("obj1_velocity_diff"),
            Arrows::new(vec![c(obj2_pos)], vec![c(obj2_velocity + obj2_pos)])
                .name("obj2_velocity_diff"),
        ]
    }
}
