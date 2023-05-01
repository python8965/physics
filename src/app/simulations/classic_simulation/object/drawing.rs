use crate::app::graphics::define::{BoxedPlotDraw, PlotColor, PlotTextSize};

use crate::app::simulations::classic_simulation::object::state::CSObjectState;

use crate::app::simulations::classic_simulation::template::stamp::CSObjectStamp;
use crate::app::simulations::classic_simulation::CSimObject;
use crate::app::simulations::state::SimulationState;
use crate::app::NVec2;
use egui::plot::{Arrows, Line, PlotPoint, Points, Text};
use egui::{Align2, RichText};

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
    pub fn draw(
        &self,
        sim_state: &SimulationState,
        index: usize,
        stamps: &mut Vec<CSObjectStamp>,
    ) -> Vec<BoxedPlotDraw> {
        puffin::profile_function!();

        let mut items: Vec<BoxedPlotDraw> = vec![];
        let settings = sim_state.settings.specific.as_c_sim_settings().unwrap();
        let filter = &settings.plot_filter;
        let current_state = self.current_state();

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
                if let Some(stamp_result) = stamp.get_data(&current_state, index, sim_state.time) {
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
            items.push(Box::new(get_self_state_text(&current_state)));
        }

        if filter.sigma_force {
            let sigma_force = current_state.sigma_force(); // Sum of force

            let vector = (current_state.position, current_state.position + sigma_force);

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
                current_state.position,
                current_state.position + current_state.velocity,
            );

            let data = ("Velocity", current_state.velocity);

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
            for acceleration in current_state
                .acc_list
                .iter()
                .filter(|x| !x.iter().all(|x| *x == 0.0))
            {
                let vector = (
                    current_state.position,
                    current_state.position + acceleration,
                );

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

        puffin::profile_scope!("trace");

        if sim_state.sim_started & filter.trace {
            const MAX_TRACE_LENGTH: usize = 5000;

            let line = {
                let current_timestep = sim_state.current_step;
                let init_timestep = self.init_timestep;

                let line_len = current_timestep
                    .saturating_sub(init_timestep)
                    .clamp(0, MAX_TRACE_LENGTH);

                let data_len = self.state_timeline.len();

                let index_end = current_timestep
                    .saturating_sub(init_timestep)
                    .clamp(0, data_len);

                let index_start = index_end.saturating_sub(line_len);

                Line::new(
                    self.state_timeline[index_start..index_end]
                        .iter()
                        .map(|x| {
                            let pos = x.position;
                            [pos.x, pos.y]
                        })
                        .collect::<Vec<_>>()
                        .to_vec(),
                )
                .color(PlotColor::TraceLine.get_color())
                .name("trace line")
            };

            items.push(Box::new(line));
        }

        items
    }
}
//change sim_state to self
