use eframe::epaint::FontFamily;

use egui::plot::{Line, PlotBounds, PlotPoint, PlotUi, Polygon, Text};
use egui::{Align2, InnerResponse, Pos2, RichText, TextStyle};

use std::fmt::Debug;

use crate::app::graphics::define::{PlotColor, PlotDrawHelper};
use crate::app::graphics::CSPlotObjects;
use crate::app::simulations::classic_simulation::object::drawing::get_object_mesh;

use crate::app::simulations::classic_simulation::{CSimObject, Simulation};
use crate::app::simulations::state::SimulationState;
use crate::app::NVec2;

pub mod object;

pub struct PlotData {
    pub near_value: f64,
    pub nearest_label: String,
    pub nearest_point: PlotPoint,
    pub dragging_object: bool,
    pub selected_index: usize,
}

impl Default for PlotData {
    fn default() -> Self {
        Self {
            near_value: 0.0,
            nearest_label: String::new(),
            nearest_point: PlotPoint::new(0.0, 0.0),
            dragging_object: false,
            selected_index: 0,
        }
    }
}

pub struct SimPlot {
    pub plot_objects: CSPlotObjects,

    plot_data: PlotData,
}

impl Default for SimPlot {
    fn default() -> Self {
        Self {
            plot_objects: CSPlotObjects::default(),
            plot_data: PlotData::default(),
        }
    }
}

impl SimPlot {
    // 그래프를 그린다.
    pub fn draw(
        &mut self,
        simulation: &Box<dyn Simulation>,
        plot_ui: &mut PlotUi,
        state: &mut SimulationState,
    ) {
        self.plot_data.nearest_label = String::new();
        self.plot_data.near_value = ObjectTraceLine::MAX_DISTANCE;

        if !state.sim_started {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([-100.0, -100.0], [100.0, 100.0]));
            state.sim_started = true;
        }

        let simulation_objects = simulation.get_children();

        // 마우스를 이 오브젝트에 포커싱 중이면서 드래그할 때 선을 그려준다.
        if let Some(pointer_pos) = plot_ui.pointer_coordinate() {
            if self.plot_data.dragging_object {
                let pos = simulation_objects[self.plot_data.selected_index]
                    .state
                    .position;
                Line::new(vec![[pos.x, pos.y], [pointer_pos.x, pointer_pos.y]]).draw(plot_ui);
            }
        }

        // 시뮬레이션 오브젝트 그리기
        for (index, obj) in simulation_objects.iter().enumerate() {
            plot_ui
                .polygon(Polygon::new(get_object_mesh(obj)).color(PlotColor::Object.get_color()));

            self.draw_object(obj, state, plot_ui, index);
        }

        // 가장 가까운 점의 좌표를 플롯에 표시한다.
        if !self.plot_data.nearest_label.is_empty() {
            let text = Text::new(
                {
                    |pos: Pos2| {
                        let a = plot_ui.plot_from_screen(Pos2::new(pos.x + 2.0, pos.y + 3.0));
                        PlotPoint::new(a.x, a.y)
                    }
                }(plot_ui.screen_from_plot(self.plot_data.nearest_point)),
                RichText::new(self.plot_data.nearest_label.clone())
                    .family(FontFamily::Proportional)
                    .text_style(TextStyle::Body),
            )
            .anchor(Align2::LEFT_TOP);

            text.draw(plot_ui);
        }

        for func in self.plot_objects.get_plot_items() {
            func.draw(plot_ui)
        }
    }
}

impl SimPlot {
    pub fn new(plot_objects: CSPlotObjects) -> Self {
        Self {
            plot_objects,
            ..Self::default()
        }
    }

    pub fn is_dragging_object(&self) -> bool {
        self.plot_data.dragging_object
    }

    // 입력을 받아서 상태를 업데이트한다.
    pub fn input(
        &mut self,
        simulation: &mut Box<dyn Simulation>,
        inner_response: InnerResponse<Option<PlotPoint>>,
    ) {
        let response = inner_response.response;

        if let Some(pointer_pos) = inner_response.inner {
            simulation.input(&mut self.plot_data, pointer_pos, response);
        }
    }

    // 시뮬레이션 오브젝트를 그린다.
    pub fn draw_object(
        &mut self,
        obj: &CSimObject,
        sim_state: &SimulationState,
        plot_ui: &mut PlotUi,
        index: usize,
    ) {
        obj.draw(sim_state, index, self.plot_objects.get_stamps())
            .into_iter()
            .for_each(|item| item.draw(plot_ui));
    }
}

#[derive(Clone, Debug)]
pub struct ObjectTraceLine {
    data: Vec<[f64; 2]>,
    last_pos: NVec2,
    last_time: f64,
}

impl ObjectTraceLine {
    const MIN_TIME: f64 = 0.0125;
    const MAX_DISTANCE: f64 = 225.0;
    const MAX_TRACE_LENGTH: usize = 1000;

    pub(crate) fn new() -> Self {
        Self {
            data: vec![],
            last_pos: NVec2::new(0.0, 0.0),
            last_time: -Self::MIN_TIME * 2.0,
        }
    }

    pub(crate) fn update(&mut self, pos: NVec2, current_time: f64) {
        let time = current_time;

        if (time - self.last_time) > Self::MIN_TIME {
            if self.data.len() > Self::MAX_TRACE_LENGTH {
                self.data.remove(0);
            }
            self.data.push([pos.x, pos.y]);
            self.last_pos = pos;
            self.last_time = time;
        }
    }

    pub(crate) fn line(&self) -> Line {
        Line::new(self.data.clone())
            .color(PlotColor::TraceLine.get_color())
            .name("trace line")
    }
}
