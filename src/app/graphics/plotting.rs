use std::f64::consts::TAU;
use std::fmt::Debug;

use eframe::epaint::FontFamily;
use egui::epaint::util::FloatOrd;
use egui::plot::{Arrows, Line, PlotBounds, PlotPoint, PlotPoints, PlotUi, Polygon, Text};
use egui::{plot, Align2, Color32, InnerResponse, Pos2, RichText, TextStyle};
use nalgebra::vector;

use crate::app::graphics::{DrawShapeType, PlotColor, PlotDrawItem};
use crate::app::simulations::object::ClassicSimulationObject;
use crate::app::simulations::simengine::Simulation;
use crate::app::simulations::state::SimulationState;
use crate::app::simulations::template::PlotObjectFnVec;
use crate::app::NVec2;

pub struct SimulationPlot {
    init: bool,
    pub objects_fn: PlotObjectFnVec,
    pub trace_lines: Vec<ObjectTraceLine>,

    state: SimulationState,

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
            state: Default::default(),
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
                if !self.dragging_object {}
            }

            if response.dragged() {
                if self.dragging_object {
                    let pos = simulation_objects[self.selected_index].position;
                    let mut selected = &mut simulation_objects[self.selected_index];
                    if selected.force_list.len() == 2 {
                        selected.force_list[1] =
                            vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y];
                    } else {
                        selected
                            .force_list
                            .push(vector![pointer_pos.x - pos.x, pointer_pos.y - pos.y]);
                    }
                }
            }

            if response.drag_released() {
                if self.dragging_object {
                    let mut selected = &mut simulation_objects[self.selected_index];
                    selected.force_list.pop();
                    self.dragging_object = false;
                }
            }
        }
    }

    pub fn draw(
        &mut self,
        simulation: &mut Box<dyn Simulation>,
        plot_ui: &mut PlotUi,
        state: SimulationState,
    ) {
        self.nearest_label = String::new();
        self.near_value = 10.0;
        self.state = state;

        if self.init {
            self.init = false;

            plot_ui.set_plot_bounds(PlotBounds::from_min_max([-100.0, -100.0], [100.0, 100.0]))
        }

        let simulation_objects = simulation.get_children();

        if let Some(pointer_pos) = plot_ui.pointer_coordinate() {
            if self.dragging_object {
                let pos = simulation_objects[self.selected_index].position;
                PlotDrawItem::Line(Line::new(PlotPoints::new(vec![
                    [pos.x, pos.y],
                    [pointer_pos.x, pointer_pos.y],
                ])))
                .draw(plot_ui);
            }
        }

        for (index, obj) in simulation_objects.iter_mut().enumerate() {
            let mut items = vec![PlotDrawItem::Polygon(
                Polygon::new(Self::get_object_points(obj)).color(PlotColor::Object.get_color()),
            )];

            items.extend(self.get_info_shape(obj, index));

            for info in items {
                info.draw(plot_ui);
            }
        }

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

    fn get_object_points(obj: &ClassicSimulationObject) -> PlotPoints {
        let scale = obj.get_scale();

        match obj.shape {
            DrawShapeType::Circle => PlotPoints::from_parametric_callback(
                move |t| (t.sin() + obj.position.x, t.cos() + obj.position.y),
                0.0..TAU,
                512,
            ),

            DrawShapeType::Box => vec![
                [obj.position.x - scale, obj.position.y - scale],
                [obj.position.x - scale, obj.position.y + scale],
                [obj.position.x + scale, obj.position.y + scale],
                [obj.position.x + scale, obj.position.y - scale],
            ]
            .into_iter()
            .map(|e| [e[0] as f64, e[1] as f64])
            .collect::<Vec<_>>()
            .into(),
        }
    }

    fn get_info_vector(
        &self,
        vector: (NVec2, NVec2),
        color: PlotColor,
        data: (impl ToString, impl Debug),
    ) -> [PlotDrawItem; 2] {
        let string = data.0.to_string();
        let value = data.1;

        let start = vector.0;
        let end = vector.1;

        let arrows = Arrows::new([start.x, start.y], [(end.x), (end.y)]);

        let text = Text::new(
            PlotPoint::from(((start + end) / 2.0).data.0[0]),
            SimulationPlot::get_sized_text(&self.state, format!("{string} : {value:?}")),
        )
        .color(color.get_color())
        .name(string.clone());

        let arrows = PlotDrawItem::Arrows(arrows.color(color.get_color()).name(string.clone()));

        let text = PlotDrawItem::Text(text);

        [text, arrows]
    }

    fn get_sized_text(state: &SimulationState, text: String) -> RichText {
        let font_size_raw = (((1.0 / state.zoom) * 100.0) + 5.0) as f32;

        match font_size_raw {
            _x if font_size_raw > 64.0 => RichText::new(""),
            _x if font_size_raw < 8.0 => RichText::new(""),
            x => RichText::new(text).size(x),
        }
    }

    pub fn get_info_shape(
        &mut self,
        obj: &mut ClassicSimulationObject,
        index: usize,
    ) -> Vec<PlotDrawItem> {
        let mut draw_vec = vec![];

        if self.state.filter.text {
            let text = format!(
                "Position : {:.3?}\nVelocity : {:.3?}\nForce(s) : {:.3?}\nMomentum : {:.3?}",
                obj.position,
                obj.velocity().norm(),
                obj.force_list,
                obj.momentum
            );

            draw_vec.push(PlotDrawItem::Text(Text::new(
                PlotPoint::new(obj.position.x, obj.position.y),
                SimulationPlot::get_sized_text(&self.state, text),
            )));
        }

        if self.state.filter.sigma_force {
            let sigma_force = obj
                .force_list
                .iter()
                .fold(NVec2::new(0.0, 0.0), |mut acc, force| {
                    acc += NVec2::new(force.x, force.y);
                    acc
                }); // Sum of force

            let vector = (obj.position, obj.position + sigma_force);

            let data = ("Sigma_Force", (vector.1 - vector.0));

            let [text, arrows] =
                self.get_info_vector((vector.0, vector.1), PlotColor::SigmaForceVector, data);

            draw_vec.push(arrows);
            draw_vec.push(text);
        }

        if self.state.filter.velocity {
            let vector = (obj.position, obj.position + obj.velocity());

            let data = ("Velocity", obj.velocity().norm());

            let [text, arrows] =
                self.get_info_vector((vector.0, vector.1), PlotColor::VelocityVector, data);
            draw_vec.push(arrows);
            draw_vec.push(text);
        }

        if self.state.filter.force {
            for force in &mut obj.force_list {
                let vector = (obj.position, obj.position + *force);

                let data = ("force", force);

                let [text, arrows] =
                    self.get_info_vector((vector.0, vector.1), PlotColor::ForceVector, data);
                draw_vec.push(arrows);
                draw_vec.push(text);
            }
        }

        {
            let trace_line = &mut self.trace_lines[index];

            if self.state.filter.trace {
                draw_vec.push(PlotDrawItem::Line(trace_line.line()));
            }

            let res = trace_line.update(obj.position, self.state);

            if !res.1.is_empty() && res.0 < self.near_value {
                self.near_value = res.0;
                self.nearest_label = res.1;
                self.nearest_point = res.2;
            }
        }

        draw_vec
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
            last_time: 0.0,
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
