use crate::app::graphics::define::{BoxedPlotDraw, DrawShapeType, PlotColor, PlotTextSize};

use crate::app::simulations::classic_simulation::object::CSObjectState;

use crate::app::simulations::classic_simulation::template::stamp::CSObjectStamp;
use crate::app::simulations::classic_simulation::CSimObject;
use crate::app::simulations::state::{SimulationState};
use crate::app::NVec2;
use egui::plot::{Arrows, PlotPoint, PlotPoints, Points, Text};
use egui::{Align2, RichText};
use std::f64::consts::TAU;
use std::fmt::Debug;

fn get_sized_text(zoom: f64, text: String, scale: f64) -> RichText {
    let font_size_raw = ((((1.0 / zoom) * 1000.0) * scale) + 10.0) / 5.0;

    let default_max = 64.0;

    let default_min = 5.0;

    match font_size_raw {
        _x if font_size_raw > default_max => RichText::new(""),
        _x if font_size_raw < default_min => RichText::new(""),
        x => RichText::new(text).size(x as f32),
    }
}

// 오브젝트 모양 점을 반환한다.
pub fn get_object_mesh(obj: &CSimObject) -> PlotPoints {
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
    zoom: f64,
    vector: (NVec2, NVec2),
    color: PlotColor,
    data: (impl ToString, impl Debug),
) -> (Text, Arrows) {
    let string = data.0.to_string();
    let value = data.1;

    let start = vector.0;
    let end = vector.1;

    let arrows = Arrows::new([start.x, start.y], [(end.x), (end.y)]);

    let text = Text::new(
        PlotPoint::from(((start + end) / 2.0).data.0[0]),
        get_sized_text(
            zoom,
            format!("{string} : {value:?}"),
            PlotTextSize::Medium.get_size(),
        ),
    )
    .color(color.get_color())
    .name(string.clone());

    let arrows = arrows.color(color.get_color()).name(string);

    (text, arrows)
}

impl CSimObject {
    pub fn update(&mut self, _dt: f64, sim_state: &SimulationState) {
        self.trace_line.update(self.state.position, sim_state.time);
    }

    pub fn draw(
        &self,
        sim_state: &SimulationState,
        index: usize,
        stamps: &mut Vec<CSObjectStamp>,
    ) -> Vec<BoxedPlotDraw> {
        let mut items: Vec<BoxedPlotDraw> = vec![];
        let settings = sim_state.settings.as_c_sim_settings().unwrap();
        let filter = &settings.plot_filter;

        let get_state_text_raw = |state: &CSObjectState| {
            format!(
                "Position : {:.3?}\nVelocity : {:.3?}\nForce(s) : {:.3?}\nMomentum : {:.3?}",
                state.position,
                state.momentum().norm(),
                state.acc_list,
                state.velocity
            )
        };

        let get_self_state_text = |state: &CSObjectState| {
            let text = get_state_text_raw(state);

            Text::new(
                PlotPoint::new(state.position.x, state.position.y),
                get_sized_text(sim_state.zoom, text, PlotTextSize::Small.get_size()),
            )
        };

        // Draw stamp
        if filter.stamp && sim_state.is_sim_started() {
            for stamp in stamps {
                if let Some(stamp_result) = stamp.get_data(&self.state, index, sim_state.time) {
                    let text = format!(
                        "<Stamp>\nLabel:{:?}\n{:}\nOn State Time {:.3?}",
                        stamp_result.label,
                        get_state_text_raw(&stamp_result.state),
                        stamp_result.time
                    );

                    let text = Text::new(
                        PlotPoint::new(
                            stamp_result.state.position.x,
                            stamp_result.state.position.y,
                        ),
                        get_sized_text(sim_state.zoom, text, PlotTextSize::Medium.get_size()),
                    )
                    .anchor(Align2::LEFT_TOP)
                    .name(stamp_result.name.clone())
                    .color(PlotColor::StampText.get_color());

                    let point =
                        Points::new([stamp_result.state.position.x, stamp_result.state.position.y])
                            .radius(2.0)
                            .color(PlotColor::Stamp.get_color());

                    items.push(Box::new(text));
                    items.push(Box::new(point));
                }
            }
        }

        if filter.text {
            items.push(Box::new(get_self_state_text(&self.state)));
        }

        if filter.sigma_force {
            let sigma_force = self.state.sigma_force(); // Sum of force

            let vector = (self.state.position, self.state.position + sigma_force);

            let data = ("Sigma_Force", (vector.1 - vector.0));

            let (text, arrows) = get_info_vector(
                sim_state.zoom,
                (vector.0, vector.1),
                PlotColor::SigmaForceVector,
                data,
            );

            items.push(Box::new(arrows));
            items.push(Box::new(text));
        }

        if filter.velocity {
            let vector = (
                self.state.position,
                self.state.position + self.state.velocity,
            );

            let data = ("Velocity", self.state.velocity);

            let (text, arrows) = get_info_vector(
                sim_state.zoom,
                (vector.0, vector.1),
                PlotColor::VelocityVector,
                data,
            );

            items.push(Box::new(arrows));
            items.push(Box::new(text));
        }

        if filter.acceleration {
            for acceleration in self
                .state
                .acc_list
                .iter()
                .filter(|x| !x.iter().all(|x| *x == 0.0))
            {
                let vector = (self.state.position, self.state.position + acceleration);

                let data = ("acceleration", acceleration);

                let (text, arrows) = get_info_vector(
                    sim_state.zoom,
                    (vector.0, vector.1),
                    PlotColor::ForceVector,
                    data,
                );

                items.push(Box::new(arrows));
                items.push(Box::new(text));
            }
        }

        if filter.equation {
            let sim_time = sim_state.time;
            let current_pos = self.state.position;
            let init_pos = self.init_state().position;
            let init_velocity = self.init_state().velocity;
            let acceleration = self.state.acceleration();
            let calc_pos =
                init_pos + (init_velocity * sim_time) + (0.5 * acceleration * sim_time.powi(2));
            let text = get_sized_text(
                sim_state.zoom,
                format!(
                    "pos_{{final}} = pos_{{start}} +v_{{start}}*t + 1/2 * a_{{start}}*t^2\n\
                         pos_{{final}} = {:.3?} + {:.3?}*{:.3?} + 1/2 * {:.3?}*{:.3?}^2\n\
                         calculated pos_{{final}} = {:.3?} and in-simulation-pos = {:.3?}\n\
                         error = {:.3?}\n\
                         ",
                    init_pos,
                    init_velocity,
                    sim_time,
                    acceleration,
                    sim_time,
                    calc_pos,
                    current_pos,
                    (calc_pos - current_pos).norm()
                ),
                10.0,
            )
            .color(PlotColor::Equation.get_color());

            let text = Text::new(
                PlotPoint::new(current_pos.x, current_pos.y + (self.state.scale() * 2.0)),
                text,
            )
            .anchor(Align2::LEFT_BOTTOM)
            .name("equation 2.16");

            items.push(Box::new(text));
        }

        if sim_state.sim_started {
            let trace_line = &self.trace_line;

            if filter.trace {
                items.push(Box::new(trace_line.line()));
            }
        }

        items
    }
}
//change sim_state to self
