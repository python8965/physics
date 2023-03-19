use crate::simulation::object::SimulationObject;
use crate::simulation::{to_f64, DrawShapeType, PlotDrawItem};
use egui::plot::{Arrows, Line, PlotPoint, PlotPoints, Polygon};
use egui::{plot, Color32, Pos2, RichText};
use std::f64::consts::TAU;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlotInfoFilter {
    pub(crate) force: bool,
    pub(crate) velocity: bool,
    pub(crate) trace: bool,
    pub(crate) text: bool,
}

pub struct PlotDrawing {}

impl PlotDrawing {
    pub(crate) fn get_draw_items(
        obj: &mut SimulationObject,
        filter: PlotInfoFilter,
        zoom: f64,
    ) -> Vec<PlotDrawItem> {
        let mut items = vec![PlotDrawItem::Polygon(Self::get_draw_shape(obj))];
        items.extend(Self::get_info_shape(obj, filter, zoom));
        items
    }

    fn get_draw_shape(obj: &mut SimulationObject) -> Polygon {
        let scale = obj.get_scale();

        Polygon::new(match obj.shape {
            DrawShapeType::Circle => PlotPoints::from_parametric_callback(
                move |t| {
                    (
                        t.sin() + obj.position.x as f64,
                        t.cos() + obj.position.y as f64,
                    )
                },
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
        })
    }

    pub fn get_info_shape(
        obj: &mut SimulationObject,
        filter: PlotInfoFilter,
        zoom: f64,
    ) -> Vec<PlotDrawItem> {
        let mut draw_vec = vec![];

        let scale = obj.get_scale();
        let font_size = (scale / zoom as f32) * 200.0;

        if filter.text {
            let text = match font_size {
                // TODO: DO NOT USE .. PATTERN WITH FLOAT
                ..=64.0 => {
                    let mut text = RichText::new(format!(
                        "Pos : {:?}\nVelocity : {:?}\nMass : {:?}\nForce(s) : {:?}",
                        obj.position,
                        obj.velocity(),
                        obj.mass,
                        obj.force_list
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
                    PlotPoint::new(obj.position.x, obj.position.y),
                    text,
                )));
            }
        }

        if filter.force {
            let points = obj
                .force_list
                .iter()
                .fold((vec![], vec![]), |mut acc, force| {
                    acc.0.push(to_f64(obj.position.x, obj.position.y));
                    acc.1
                        .push(to_f64(force.x + obj.position.x, force.y + obj.position.y));
                    acc
                });

            let arrows = PlotDrawItem::Arrow(Arrows::new(points.0, points.1));
            draw_vec.push(arrows);
        }

        if filter.velocity {
            let points = obj
                .force_list
                .iter()
                .fold((vec![], vec![]), |mut acc, force| {
                    acc.0.push(to_f64(obj.position.x, obj.position.y));
                    acc.1.push(to_f64(
                        obj.velocity().x + obj.position.x,
                        obj.velocity().y + obj.position.y,
                    ));
                    acc
                });

            let arrows = PlotDrawItem::Arrow(Arrows::new(points.0, points.1));
            draw_vec.push(arrows);
        }

        if filter.trace {
            draw_vec.push(PlotDrawItem::Line(obj.trace.line()));
            obj.trace.update(obj.position);
        }

        draw_vec
    }
}

pub struct ObjectTraceLine {
    data: Vec<[f64; 2]>,
    last_pos: Pos2,
}

impl ObjectTraceLine {
    const MIN_DISTANCE: f32 = 3.0;

    pub(crate) fn new() -> Self {
        Self {
            data: vec![],
            last_pos: Pos2::new(0.0, 0.0),
        }
    }

    fn update(&mut self, pos: Pos2) {
        if self.last_pos.distance(pos) > Self::MIN_DISTANCE {
            self.data.push([pos.x as f64, pos.y as f64]);
            self.last_pos = pos;
        }
    }

    fn line(&self) -> Line {
        Line::new(self.data.clone()).color(Color32::from_rgba_unmultiplied(245, 2, 216, 0))
    }
}
