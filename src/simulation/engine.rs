use egui::plot::PlotUi;

use crate::simulation::drawing::{PlotDrawing, PlotInfoFilter};
use crate::simulation::object::DefaultSim;
use crate::simulation::{Float, PlotDrawItem, Vec2};

#[derive(Clone, Copy, Debug, Default)]
pub struct SimState {
    pub(crate) time: f64,
    pub(crate) filter: PlotInfoFilter,
}

pub trait Simulation: Send + Sync {
    fn step(&mut self, dt: Float);

    fn draw(&mut self, plot_ui: &mut PlotUi, state: SimState);
}

pub type ItemGetFn = Box<dyn FnOnce(SimState) -> Vec<PlotDrawItem> + Sync + Send>;

#[derive()]
pub struct BasicSim {
    children: Vec<DefaultSim>,
    items_fn: Box<dyn FnOnce(SimState) -> Vec<PlotDrawItem> + Sync + Send>,
}

impl From<Vec<DefaultSim>> for BasicSim {
    fn from(children: Vec<DefaultSim>) -> Self {
        BasicSim {
            children,
            items_fn: Box::new(|_| vec![]),
        }
    }
}

impl From<(Vec<DefaultSim>, ItemGetFn)> for BasicSim {
    fn from((children, items): (Vec<DefaultSim>, ItemGetFn)) -> Self {
        BasicSim {
            children,
            items_fn: items,
        }
    }
}

impl BasicSim {
    pub fn new() -> BasicSim {
        BasicSim {
            children: vec![],
            items_fn: Box::new(|_| vec![]),
        }
    }
}

impl Simulation for BasicSim {
    fn step(&mut self, dt: f64) {
        for child in &mut self.children {
            physics_system(dt, child);
        }
    }

    fn draw(&mut self, plot_ui: &mut PlotUi, state: SimState) {
        let zoom = plot_ui.plot_bounds().width();
        for child in &mut self.children {
            for info in PlotDrawing::get_draw_items(child, state, zoom) {
                info.draw(plot_ui)
            }
        }
        //TODO: Draw items_fn
    }
}

fn physics_system(dt: Float, obj: &mut DefaultSim) {
    obj.position = {
        let sigma_force: Vec2 = obj.force_list.iter().fold(Vec2::zeros(), |acc, x| acc + *x); // ΣF

        // ΣF = Δp / Δt
        // 우리는 운동량 p를 원한다
        // Δp = ΣF * Δt

        let delta_momentum = sigma_force * dt;
        obj.momentum += delta_momentum;

        // Δs = v * Δt

        let delta_position = obj.velocity() * dt;

        obj.position + delta_position
    };
}
