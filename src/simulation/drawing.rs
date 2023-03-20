use crate::simulation::object::SimulationObject;
use crate::simulation::{DrawShapeType, OVec2, PlotDrawItem, PlotVectorType};
use egui::plot::{Arrows, Line, PlotPoint, PlotPoints, Polygon, Text};
use egui::{plot, Color32, RichText};
use std::f64::consts::TAU;
use tracing_subscriber::fmt::format;
use vector2math::{FloatingVector2, Vector2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlotInfoFilter {
    pub(crate) force: bool,
    pub(crate) sigma_force: bool,
    pub(crate) velocity: bool,
    pub(crate) trace: bool,
    pub(crate) text: bool,
}

pub struct PlotDrawing {}

impl PlotDrawing {
    pub(crate) fn get_draw_items(
        obj: &mut SimulationObject,
        filter: PlotInfoFilter,
        time: f64,
        zoom: f64,
    ) -> Vec<PlotDrawItem> {
        let mut items = vec![PlotDrawItem::Polygon(Self::get_draw_shape(obj))];
        items.extend(Self::get_info_shape(obj, filter, time, zoom));
        items
    }

    fn get_draw_shape(obj: &mut SimulationObject) -> Polygon {
        let scale = obj.get_scale();

        Polygon::new(match obj.shape {
            DrawShapeType::Circle => PlotPoints::from_parametric_callback(
                move |t| {
                    (
                        t.sin() + obj.position.x() as f64,
                        t.cos() + obj.position.y() as f64,
                    )
                },
                0.0..TAU,
                512,
            ),

            DrawShapeType::Box => vec![
                [obj.position.x() - scale, obj.position.y() - scale],
                [obj.position.x() - scale, obj.position.y() + scale],
                [obj.position.x() + scale, obj.position.y() + scale],
                [obj.position.x() + scale, obj.position.y() - scale],
            ]
            .into_iter()
            .map(|e| [e[0] as f64, e[1] as f64])
            .collect::<Vec<_>>()
            .into(),
        })
    }

    fn get_info_vector(start: OVec2, end: OVec2, text: RichText) -> [PlotDrawItem; 2] {
        let arrows = Arrows::new([start.x(), start.y()], [(end.x()), (end.y())]);

        let text = Text::new(PlotPoint::from(((start + end) / 2.0).map_vec2()), text);

        let arrows = PlotDrawItem::Arrows(arrows.color(PlotVectorType::Velocity.to_color()));
        let text = PlotDrawItem::Text(text);

        [text, arrows]
    }

    pub fn get_info_shape(
        obj: &mut SimulationObject,
        filter: PlotInfoFilter,
        time: f64,
        zoom: f64,
    ) -> Vec<PlotDrawItem> {
        let mut draw_vec = vec![];

        let scale = obj.get_scale();
        let font_size_raw = ((scale / zoom) * 400.0) as f32;

        let font_size = match font_size_raw {
            _x if font_size_raw > 64.0 => 64.0,
            _x if font_size_raw < 8.0 => 8.0,
            x => x,
        };

        let velocity_string = format!("Velocity : {:.3?}", obj.velocity().length());

        if filter.text {
            let text = match font_size_raw {
                // TODO: DO NOT USE ..= PATTERN WITH FLOAT
                ..=64.0 => {
                    let mut text = RichText::new(format!(
                        "Position : {:?}\nVelocity : {:?}\nForce(s) : {:?}\nMomentum : {:?}",
                        obj.position, velocity_string, obj.force_list, obj.momentum
                    ));

                    match font_size {
                        ..=12.0 => {
                            text = text.size(12.0);
                        }
                        12.0..=64.0 => {
                            text = text.size(font_size);
                        }
                        _ => {
                            assert!(true);
                        }
                    }

                    Some(text)
                }

                64.0.. => None,
                _ => {
                    assert!(true);
                    None
                }
            };

            if let Some(text) = text {
                draw_vec.push(PlotDrawItem::Text(plot::Text::new(
                    PlotPoint::new(obj.position.x(), obj.position.y()),
                    text,
                )));
            }
        }

        if filter.sigma_force {
            let vector = obj.force_list.iter().fold(
                (OVec2::new(0.0, 0.0), OVec2::new(0.0, 0.0)),
                |mut acc, force| {
                    acc.0 += OVec2::new(obj.position.x(), obj.position.y());
                    acc.1 += OVec2::new(force.x() + obj.position.x(), force.y() + obj.position.y());
                    acc
                },
            ); // Sum of force

            let text =
                RichText::new(format!("Sigma_Force {:.3?}", (vector.1 - vector.0))).size(font_size);

            let [text, arrows] = PlotDrawing::get_info_vector(vector.0, vector.1, text);

            draw_vec.push(text);
            draw_vec.push(arrows);
        }

        if filter.velocity {
            let vector = (obj.position, obj.position + obj.velocity());

            let text = RichText::new(velocity_string.clone()).size(font_size);

            let [text, arrows] = PlotDrawing::get_info_vector(vector.0, vector.1, text);
            draw_vec.push(text);
            draw_vec.push(arrows);
        }

        if filter.force {
            for force in &mut obj.force_list {
                let vector = (obj.position, obj.position + *force);

                dbg!(vector);

                let text = RichText::new(format!("force : {:?}", force)).size(font_size);

                let [text, arrows] = PlotDrawing::get_info_vector(vector.0, vector.1, text);
                draw_vec.push(text);
                draw_vec.push(arrows);
            }
        }

        if filter.trace {
            draw_vec.push(PlotDrawItem::Line(obj.trace.line()));
        }

        obj.trace.update(obj.position, time);

        draw_vec
    }
}

pub struct ObjectTraceLine {
    data: Vec<[f64; 2]>,
    last_pos: OVec2,
    last_time: f64,
}

impl ObjectTraceLine {
    const MIN_TIME: f64 = 0.5;

    pub(crate) fn new() -> Self {
        Self {
            data: vec![],
            last_pos: OVec2::new(0.0, 0.0),
            last_time: 0.0,
        }
    }

    fn update(&mut self, pos: OVec2, time: f64) {
        if (time - self.last_time) > Self::MIN_TIME {
            self.data.push([pos.x() as f64, pos.y() as f64]);
            self.last_pos = pos;
            self.last_time = time;
        }
    }

    fn line(&self) -> Line {
        Line::new(self.data.clone()).color(Color32::from_rgba_unmultiplied(245, 2, 216, 0))
    }
}
