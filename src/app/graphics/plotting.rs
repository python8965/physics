use std::f64::consts::TAU;
use std::fmt::Debug;

use crate::app::graphics::define::{DrawShapeType, PlotColor, PlotDrawItem};

use crate::app::simulations::classic_simulation::Simulation;
use crate::app::simulations::object::ClassicSimulationObject;
use crate::app::simulations::state::SimulationState;
use crate::app::simulations::template::PlotObjectFnVec;
use crate::app::NVec2;
use eframe::epaint::FontFamily;
use egui::epaint::util::FloatOrd;
use egui::plot::{Arrows, Line, PlotBounds, PlotPoint, PlotPoints, PlotUi, Polygon, Text};
use egui::{plot, Align2, InnerResponse, Pos2, RichText, TextStyle};
use nalgebra::vector;

pub struct SimulationPlot {
    init: bool,
    pub objects_fn: PlotObjectFnVec,
    pub trace_lines: Vec<ObjectTraceLine>,

    sim_state: SimulationState,

    near_value: f64,
    nearest_label: String,
    nearest_point: PlotPoint,

    dragging_object: bool,
    selected_index: usize,
}

impl Default for SimulationPlot {
    fn default() -> Self {
        Self {
            init: true,
            objects_fn: vec![],
            near_value: 10.0,

            nearest_label: "".to_string(),
            nearest_point: PlotPoint { x: 0.0, y: 0.0 },
            trace_lines: vec![],
            dragging_object: false,
            selected_index: 0,
            sim_state: Default::default(),
        }
    }
}

fn is_inside(pos: PlotPoint, plotpoint: &[PlotPoint]) -> bool {
    let mut contact = 0;
    for (p1, p2) in plotpoint.windows(2).map(|x| (x[0], x[1])) {
        if (pos.y > p1.y) != (pos.y > p2.y) {
            let at_x = (p2.x - p1.x) * (pos.y - p1.y) / (p2.y - p1.y) + p1.x;

            if at_x > pos.x {
                contact += 1;
            }
        }
    }
    contact % 2 > 0
}

impl SimulationPlot {
    pub fn new(objects_count: usize, objects_fn: PlotObjectFnVec) -> Self {
        Self {
            objects_fn,
            trace_lines: (0..objects_count)
                .map(|_| ObjectTraceLine::new())
                .collect::<Vec<_>>(),
            ..Self::default()
        }
    }

    pub fn is_dragging_object(&self) -> bool {
        self.dragging_object
    }

    // 입력을 받아서 상태를 업데이트한다.
    pub fn input(
        &mut self,
        simulation: &mut Box<dyn Simulation>,
        inner_response: InnerResponse<Option<PlotPoint>>,
    ) {
        let simulation_objects = simulation.get_children();
        let response = inner_response.response;

        if let Some(pointer_pos) = inner_response.inner {
            if response.drag_started() {
                for (index, obj) in simulation_objects.iter_mut().enumerate() {
                    if is_inside(pointer_pos, Self::get_object_points(obj).points()) {
                        self.selected_index = index;
                        self.dragging_object = true;
                        break;
                    }
                }
            }

            if response.dragged() && self.dragging_object {
                let pos = simulation_objects[self.selected_index].state.position;
                let selected = &mut simulation_objects[self.selected_index];
                if selected.state.force_list.len() == 2 {
                    selected.state.force_list[1] =
                        vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y];
                } else {
                    selected
                        .state
                        .force_list
                        .push(vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y]);
                }
            }

            if !response.dragged() && self.dragging_object {
                let selected = &mut simulation_objects[self.selected_index];
                if selected.state.force_list.len() == 2 {
                    selected.state.force_list.remove(1);
                }

                self.dragging_object = false;
            }
        }
    }

    // 그래프를 그린다.
    pub fn draw(
        &mut self,
        simulation: &mut Box<dyn Simulation>,
        plot_ui: &mut PlotUi,
        state: SimulationState,
    ) {
        self.nearest_label = String::new();
        self.near_value = 10.0;
        self.sim_state = state;

        // 처음에는 그래프의 범위를 설정한다.
        if self.init {
            self.init = false;

            plot_ui.set_plot_bounds(PlotBounds::from_min_max([-100.0, -100.0], [100.0, 100.0]))
        }

        let simulation_objects = simulation.get_children();

        // 마우스를 이 오브젝트에 포커싱 중이면서 드래그할 때 선을 그려준다.
        if let Some(pointer_pos) = plot_ui.pointer_coordinate() {
            if self.dragging_object {
                let pos = simulation_objects[self.selected_index].state.position;
                PlotDrawItem::Line(Line::new(PlotPoints::new(vec![
                    [pos.x, pos.y],
                    [pointer_pos.x, pointer_pos.y],
                ])))
                .draw(plot_ui);
            }
        }

        // 시뮬레이션 오브젝트마다 정보 모양을 제공한다.
        for (index, obj) in simulation_objects.iter_mut().enumerate() {
            plot_ui.polygon(
                Polygon::new(Self::get_object_points(obj)).color(PlotColor::Object.get_color()),
            );

            self.draw_object(obj, plot_ui, index);
        }

        // 가장 가까운 점의 정보를 표시한다.
        if !self.nearest_label.is_empty() {
            let text = Text::new(
                {
                    |pos: Pos2| {
                        let a = plot_ui.plot_from_screen(Pos2::new(pos.x + 2.0, pos.y + 3.0));
                        PlotPoint::new(a.x, a.y)
                    }
                }(plot_ui.screen_from_plot(self.nearest_point)),
                RichText::new(self.nearest_label.clone())
                    .family(FontFamily::Proportional)
                    .text_style(TextStyle::Body),
            )
            .anchor(Align2::LEFT_TOP);

            let text = PlotDrawItem::Text(text);

            text.draw(plot_ui);
        }

        for func in &mut self.objects_fn {
            for info in func(state) {
                info.draw(plot_ui)
            }
        }
    }

    // 오브젝트의 정보 모양을 반환한다.
    pub fn draw_object(
        &mut self,
        obj: &mut ClassicSimulationObject,
        plot_ui: &mut PlotUi,
        index: usize,
    ) {
        if self.sim_state.settings.text {
            let text = format!(
                "Position : {:.3?}\nVelocity : {:.3?}\nForce(s) : {:.3?}\nMomentum : {:.3?}",
                obj.state.position,
                obj.state.velocity().norm(),
                obj.state.force_list,
                obj.state.momentum
            );

            plot_ui.text(Text::new(
                PlotPoint::new(obj.state.position.x, obj.state.position.y),
                SimulationPlot::get_sized_text(&self.sim_state, text, 1.0),
            ));
        }

        if self.sim_state.settings.sigma_force {
            let sigma_force = obj.state.sigma_force(); // Sum of force

            let vector = (obj.state.position, obj.state.position + sigma_force);

            let data = ("Sigma_Force", (vector.1 - vector.0));

            let (text, arrows) =
                self.get_info_vector((vector.0, vector.1), PlotColor::SigmaForceVector, data);

            plot_ui.arrows(arrows);
            plot_ui.text(text);
        }

        if self.sim_state.settings.velocity {
            let vector = (
                obj.state.position,
                obj.state.position + obj.state.velocity(),
            );

            let data = ("Velocity", obj.state.velocity().norm());

            let (text, arrows) =
                self.get_info_vector((vector.0, vector.1), PlotColor::VelocityVector, data);
            plot_ui.arrows(arrows);
            plot_ui.text(text);
        }

        if self.sim_state.settings.force {
            for force in &mut obj.state.force_list {
                let vector = (obj.state.position, obj.state.position + *force);

                let data = ("force", force);

                let (text, arrows) =
                    self.get_info_vector((vector.0, vector.1), PlotColor::ForceVector, data);

                plot_ui.arrows(arrows);
                plot_ui.text(text);
            }
        }

        if self.sim_state.settings.equation {
            let sim_time = self.sim_state.time;
            let current_pos = obj.state.position;
            let init_pos = obj.init_state().position;
            let init_velocity = obj.init_state().velocity();
            let acceleration = obj.state.acceleration();

            let text = Self::get_sized_text(
                &self.sim_state,
                format!(
                    "pos_{{final}} = pos_{{start}} +v_{{start}}*t + 1/2 * a_{{start}}*t^2\n\
                         pos_{{final}} = {:.3?} + {:.3?}*{:.3?} + 1/2 * {:.3?}*{:.3?}^2\n\
                         calculated pos_{{final}} = {:.3?} and in-simulation-pos = {:.3?}\n\
                         ",
                    init_pos,
                    init_velocity,
                    sim_time,
                    acceleration,
                    sim_time,
                    init_pos + (init_velocity * sim_time) + (0.5 * acceleration * sim_time.powi(2)),
                    current_pos,
                ),
                10.0,
            )
            .color(PlotColor::Equation.get_color());

            let text = Text::new(
                PlotPoint::new(current_pos.x, current_pos.y + (obj.state.scale() * 2.0)),
                text,
            )
            .anchor(Align2::LEFT_BOTTOM)
            .name("equation 2.16");

            plot_ui.text(text);
        }

        {
            let trace_line = &mut self.trace_lines[index];

            let res = trace_line.update(obj.state.position, self.sim_state);

            if !res.1.is_empty() && res.0 < self.near_value {
                self.near_value = res.0;
                self.nearest_label = res.1;
                self.nearest_point = res.2;
            }

            if self.sim_state.settings.trace {
                plot_ui.line(trace_line.line());
            }
        }
    }

    // 오브젝트 모양 점을 반환한다.
    fn get_object_points(obj: &ClassicSimulationObject) -> PlotPoints {
        let scale = obj.state.scale();

        match obj.shape {
            DrawShapeType::Circle => PlotPoints::from_parametric_callback(
                move |t| {
                    (
                        t.sin() * scale + obj.state.position.x,
                        t.cos() * scale + obj.state.position.y,
                    )
                },
                0.0..TAU,
                512,
            ),

            DrawShapeType::Box => vec![
                [obj.state.position.x - scale, obj.state.position.y - scale],
                [obj.state.position.x - scale, obj.state.position.y + scale],
                [obj.state.position.x + scale, obj.state.position.y + scale],
                [obj.state.position.x + scale, obj.state.position.y - scale],
            ]
            .into_iter()
            .map(|e| [e[0], e[1]])
            .collect::<Vec<_>>()
            .into(),
        }
    }

    // 벡터의 화살표 모양을 반환한다.
    fn get_info_vector(
        &self,
        vector: (NVec2, NVec2),
        color: PlotColor,
        data: (impl ToString, impl Debug),
    ) -> (plot::Text, plot::Arrows) {
        let string = data.0.to_string();
        let value = data.1;

        let start = vector.0;
        let end = vector.1;

        let arrows = Arrows::new([start.x, start.y], [(end.x), (end.y)]);

        let text = Text::new(
            PlotPoint::from(((start + end) / 2.0).data.0[0]),
            SimulationPlot::get_sized_text(&self.sim_state, format!("{string} : {value:?}"), 1.0),
        )
        .color(color.get_color())
        .name(string.clone());

        let arrows = arrows.color(color.get_color()).name(string);

        (text, arrows)
    }

    // 크기가 조정된 텍스트를 반환한다.
    fn get_sized_text(state: &SimulationState, text: String, scale: f64) -> RichText {
        let font_size_raw = (((1.0 / state.zoom) * 100.0) * scale) + 10.0;

        let default_max = 64.0;

        let default_min = 12.0;

        match font_size_raw {
            _x if font_size_raw > default_max => RichText::new(""),
            _x if font_size_raw < default_min => RichText::new(""),
            x => RichText::new(text).size(x as f32),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ObjectTraceLine {
    data: Vec<[f64; 2]>,
    last_pos: NVec2,
    last_time: f64,
}

impl ObjectTraceLine {
    const MIN_TIME: f64 = 0.25;

    pub(crate) fn new() -> Self {
        Self {
            data: vec![],
            last_pos: NVec2::new(0.0, 0.0),
            last_time: -Self::MIN_TIME,
        }
    }

    fn update(&mut self, pos: NVec2, state: SimulationState) -> (f64, String, PlotPoint) {
        let time = state.time;

        if (time - self.last_time) > Self::MIN_TIME {
            self.data.push([pos.x, pos.y]);
            self.last_pos = pos;
            self.last_time = time;
        }

        if let Some(pointer) = state.pointer {
            let closest = self
                .data
                .iter()
                .enumerate()
                .map(|(index, pos)| {
                    (
                        index,
                        pointer
                            .to_pos2()
                            .distance_sq(Pos2::new(pos[0] as f32, pos[1] as f32)),
                    )
                })
                .min_by_key(|e| e.1.ord());

            if let Some(closest) = closest {
                return (
                    closest.1 as f64,
                    format!("At time : {}s", (closest.0 as f64) * Self::MIN_TIME),
                    PlotPoint::from(self.data[closest.0]),
                );
            }
        }

        (0.0, String::new(), PlotPoint::new(0.0, 0.0))
    }

    fn line(&self) -> Line {
        Line::new(self.data.clone())
            .color(PlotColor::TraceLine.get_color())
            .name("trace line")
    }
}
