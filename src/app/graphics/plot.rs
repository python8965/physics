use eframe::epaint::FontFamily;

use egui::plot::{Line, PlotBounds, PlotPoint, PlotUi, Polygon, Text};
use egui::{Align2, InnerResponse, Pos2, RichText, TextStyle};

use crate::app::graphics::define::PlotColor;
use crate::app::graphics::CSPlotObjects;

use crate::app::simulations::classic_simulation::{CSimObject, Simulation};
use crate::app::simulations::state::SimulationState;

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

#[derive(Default)]
pub struct SimPlot {
    pub plot_objects: CSPlotObjects,

    plot_data: PlotData,
}

pub const PLOT_MAX_DISTANCE: f64 = 225.0;

impl SimPlot {
    // 그래프를 그린다.
    pub fn draw(
        &mut self,
        simulation: &dyn Simulation,
        plot_ui: &mut PlotUi,
        state: &mut SimulationState,
    ) {
        puffin::profile_scope!("draw_plot");

        self.plot_data.nearest_label = String::new();
        self.plot_data.near_value = PLOT_MAX_DISTANCE;

        if !state.sim_started {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([-100.0, -100.0], [100.0, 100.0]));
            state.sim_started = true;
        }

        if let Some(x) = simulation.get_events(state.current_step) {
            x.get_shapes().into_iter().for_each(|shape| {
                shape.draw(plot_ui);
            })
        }

        let simulation_objects = simulation.get_children();

        // 마우스를 이 오브젝트에 포커싱 중이면서 드래그할 때 선을 그려준다.
        if let Some(pointer_pos) = plot_ui.pointer_coordinate() {
            if self.plot_data.dragging_object {
                let pos = simulation_objects[self.plot_data.selected_index]
                    .current_state()
                    .position;

                plot_ui.line(Line::new(vec![
                    [pos.x, pos.y],
                    [pointer_pos.x, pointer_pos.y],
                ]));
            }
        }

        // 시뮬레이션 오브젝트 그리기
        for (index, obj) in simulation_objects.iter().enumerate() {
            if *obj.hide() {
                continue;
            }

            let obj_state = &mut obj.current_state();

            plot_ui.polygon(
                Polygon::new(obj_state.shape.get_plot_points(obj_state.position))
                    .color(PlotColor::Object.get_color()),
            );

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

            plot_ui.text(text);
        }

        for func in self.plot_objects.get_plot_items() {
            func.draw(plot_ui)
        }
    }
}

pub struct InputMessage {
    pub clicked: bool,
    pub hovered: bool,
    pub pointer_pos: Option<PlotPoint>,
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
        inner_response: InnerResponse<InputMessage>,
        ctx: &egui::Context,
        state: &mut SimulationState,
    ) {
        let response = inner_response.response;

        simulation.input(
            &mut self.plot_data,
            inner_response.inner,
            response,
            ctx,
            state,
        );
    }

    // 시뮬레이션 오브젝트를 그린다.
    pub fn draw_object(
        &mut self,
        obj: &CSimObject,
        sim_state: &SimulationState,
        plot_ui: &mut PlotUi,
        timestep: usize,
    ) {
        obj.draw(sim_state, timestep, self.plot_objects.get_stamps())
            .into_iter()
            .for_each(|item| item.draw(plot_ui));
    }
}
