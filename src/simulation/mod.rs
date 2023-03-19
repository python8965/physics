use egui::plot::{Arrows, Line, PlotPoints, PlotUi, Polygon, Text};
use egui::Vec2;
use std::f64::consts::TAU;
use std::fmt::{Debug, Formatter};

pub mod drawing;
pub mod engine;
pub mod manager;
pub mod object;
pub mod template;

type Float = f32;

fn to_f64(x: f32, y: f32) -> [f64; 2] {
    [x as f64, y as f64]
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DrawShapeType {
    Circle,
    Box,
}

pub enum PlotDrawItem {
    Polygon(Polygon),
    Arrow(Arrows),
    Text(Text),
    Line(Line),
}

impl Debug for PlotDrawItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PlotDrawItem::Polygon(_) => "DrawShape",
            PlotDrawItem::Arrow(_) => "Arrow",
            PlotDrawItem::Text(_) => "Text",
            PlotDrawItem::Line(_) => "Line",
        })
    }
}

impl PlotDrawItem {
    fn draw(self, plot_ui: &mut PlotUi) {
        match self {
            PlotDrawItem::Polygon(polygon) => {
                plot_ui.polygon(polygon);
            }
            PlotDrawItem::Arrow(arrows) => {
                plot_ui.arrows(arrows);
            }
            PlotDrawItem::Text(text) => {
                plot_ui.text(text);
            }
            PlotDrawItem::Line(line) => {
                plot_ui.line(line);
            }
        }
    }
}

pub trait SumOnly {
    fn sum_only(&self) -> Vec2;
}

impl SumOnly for Vec<Vec2> {
    fn sum_only(&self) -> Vec2 {
        let mut vec = Vec2::new(0.0, 0.0);
        for i in self {
            vec += *i;
        }
        vec
    }
}
