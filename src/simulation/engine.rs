use egui::plot::PlotUi;
use egui::Vec2;

use crate::simulation::drawing::{PlotDrawing, PlotInfoFilter};
use crate::simulation::object::SimulationObject;
use crate::simulation::{Float, SumOnly};

pub trait Simulation: Send + Sync {
    fn finish(&mut self);

    fn finished(&self) -> bool;

    fn step(&mut self, dt: Float);

    fn draw(&mut self, plot_ui: &mut PlotUi);
}

#[derive()]
pub struct BasicSim {
    children: Vec<SimulationObject>,
    active: bool,
}

impl BasicSim {
    pub fn new() -> BasicSim {
        BasicSim {
            children: vec![],
            active: true,
        }
    }

    pub fn from(children: Vec<SimulationObject>) -> BasicSim {
        BasicSim {
            children,
            active: true,
        }
    }
}

impl Simulation for BasicSim {
    fn finish(&mut self) {
        self.active = false;
    }

    fn finished(&self) -> bool {
        !self.active
    }

    fn step(&mut self, dt: Float) {
        for child in &mut self.children {
            physics_system(dt, child);
        }
    }

    fn draw(&mut self, plot_ui: &mut PlotUi) {
        let zoom = plot_ui.plot_bounds().width();
        for child in &mut self.children {
            for info in PlotDrawing::get_draw_items(
                child,
                PlotInfoFilter {
                    force: true,
                    velocity: true,
                    trace: true,
                    text: false,
                },
                zoom,
            ) {
                info.draw(plot_ui)
            }
        }
    }
}

fn physics_system(dt: Float, obj: &mut SimulationObject) {
    obj.position = {
        let sigma_force: Vec2 = obj.force_list.sum_only(); // ΣF

        // ΣF = δp / δt
        // 우리는 운동량 p를 원한다
        // δp = ΣF * δt

        let delta_momentum = sigma_force * dt;
        obj.momentum += delta_momentum;

        // δs = v * δt

        let delta_position = obj.velocity() * dt;

        obj.position + delta_position
    };
}
