use crate::app::graphics::define::PlotItem;
use crate::app::simulations::classic_simulation::object::state::{CSObjectState, ListAdd};
use crate::app::NVec2;
use egui::plot::Arrows;

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
        let obj1_pos = self.obj1_state.position.data.0[0];
        let obj2_pos = self.obj2_state.position.data.0[0];

        let obj2_velocity = self.obj2_state.velocity.data.0[0];
        let obj1_velocity = self.obj1_state.velocity.data.0[0];

        let contact_normal = self.contact_normal.data.0[0];

        let obj1_momentum = self.obj1_state.momentum().data.0[0];
        let obj2_momentum = self.obj2_state.momentum().data.0[0];

        vec![
            Arrows::new(vec![obj2_pos], vec![contact_normal]).name("contact_normal"),
            Arrows::new(vec![obj1_pos], vec![obj1_velocity.add(obj1_pos)])
                .name("obj1_velocity_diff"),
            Arrows::new(vec![obj2_pos], vec![obj2_velocity.add(obj2_pos)])
                .name("obj2_velocity_diff"),
            Arrows::new(vec![obj1_pos, obj2_pos], vec![obj1_momentum, obj2_momentum])
                .name("momentum"),
        ]
    }
}
