
use crate::app::simulations::classic_simulation::CSimObject;
use crate::app::simulations::classic_simulation::object::AttachedFn;
use crate::app::simulations::classic_simulation::object::shape::ObjectShape;
use crate::app::simulations::classic_simulation::object::state::CSObjectState;

pub struct CSimObjectBuilder {
    init_state: Option<CSObjectState>,
    init_timestep: Option<usize>,
    shape: Option<ObjectShape>,
    attached: Option<AttachedFn>,
}

impl CSimObjectBuilder {
    pub fn new(state: CSObjectState) -> Self {
        Self {
            init_state: Some(state),
            init_timestep: None,
            shape: None,
            attached: None,
        }
    }

    pub fn init_timestep(mut self, init_timestep: usize) -> Self {
        self.init_timestep = Some(init_timestep);
        self
    }

    pub fn shape(mut self, shape: ObjectShape) -> Self {
        self.shape = Some(shape);
        self
    }

    pub fn attached(mut self, attached: AttachedFn) -> Self {
        self.attached = Some(attached);
        self
    }

    pub fn build(self) -> CSimObject {
        let init_timestep = self.init_timestep.unwrap_or(0);
        CSimObject {
            state_timeline: vec![self.init_state.unwrap_or_default()],
            init_timestep,
            timestep: init_timestep,
            hide: false,
            attached: self.attached,
        }
    }
}
