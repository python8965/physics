use crate::app::NVec2;
use egui::plot::PlotPoints;
use std::f64::consts::TAU;

pub trait Shape {
    fn get_points(&self) -> Vec<[f64; 2]>;
    fn get_plot_points(&self) -> PlotPoints;
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectShape {
    Circle(Circle),
    Rect(Rect),
}

impl Default for ObjectShape {
    fn default() -> Self {
        Self::Circle(Circle::default())
    }
}

#[allow(dead_code)]
impl ObjectShape {
    pub fn circle(pos: NVec2, radius: f64) -> Self {
        Self::Circle(Circle {
            radius,
            position: pos,
        })
    }

    pub fn rect(pos: NVec2, width: f64, height: f64) -> Self {
        Self::Rect(Rect {
            position: pos,
            width,
            height,
        })
    }

    pub fn get_points(&self) -> Vec<[f64; 2]> {
        match self {
            Self::Circle(circle) => circle.get_points(),
            Self::Rect(rect) => rect.get_points(),
        }
    }

    pub fn get_plot_points(&self) -> PlotPoints {
        match self {
            Self::Circle(circle) => circle.get_plot_points(),
            Self::Rect(rect) => rect.get_plot_points(),
        }
    }

    pub fn contact(&self, ops: ObjectShape) -> Option<ContactInfo> {
        match self {
            Self::Circle(circle) => circle.contact(ops),
            Self::Rect(rect) => rect.contact(ops),
        }
    }
}

struct ContactInfo {
    pub contact_point: NVec2,
    pub contact_normal: NVec2,
    pub penetration: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub radius: f64,
    pub position: NVec2,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: Self::DEFAULT_RADIUS,
            position: Default::default(),
        }
    }
}

impl Shape for Circle {
    fn get_points(&self) -> Vec<[f64; 2]> {
        self._get_points(Self::DEFAULT_RESOLUTION)
    }

    fn get_plot_points(&self) -> PlotPoints {
        PlotPoints::from_parametric_callback(
            move |t| {
                (
                    t.sin() * self.radius + self.position.x,
                    t.cos() * self.radius + self.position.y,
                )
            },
            0.0..TAU,
            Self::DEFAULT_RESOLUTION as usize,
        )
    }
}

impl Circle {
    const DEFAULT_RESOLUTION: u64 = 50;
    const DEFAULT_RADIUS: f64 = 10.0;

    fn _get_points(&self, resolution: u64) -> Vec<[f64; 2]> {
        let mut points = vec![];
        let radius = self.radius;
        let mut angle = 0.0;
        while angle < TAU {
            points.push([radius * angle.cos(), radius * angle.sin()]);
            angle += TAU / (resolution as f64);
        }
        points
    }

    #[allow(dead_code)]
    pub fn get_points_with_resolution(&self, resolution: u64) -> Vec<[f64; 2]> {
        if resolution < 2 {
            return vec![];
        }

        self._get_points(resolution)
    }

    pub fn contact(&self, ops: ObjectShape) -> Option<ContactInfo> {
        match ops {
            ObjectShape::Circle(circle) => {
                //TODO: copilot maked code, may be wrong
                let distance = (self.position - circle.position).norm();
                let penetration = self.radius + circle.radius - distance;
                let contact_normal = (circle.position - self.position).normalize();
                let contact_point = self.position + contact_normal * self.radius;
                Some(ContactInfo {
                    contact_point,
                    contact_normal,
                    penetration,
                })
            }
            ObjectShape::Rect(rect) => {
                todo!("Circle-Rect contact");
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub width: f64,
    pub height: f64,

    pub position: NVec2,
}

impl Shape for Rect {
    fn get_points(&self) -> Vec<[f64; 2]> {
        let width = self.width;
        let height = self.height;
        vec![
            [-width / 2.0, -height / 2.0],
            [width / 2.0, -height / 2.0],
            [width / 2.0, height / 2.0],
            [-width / 2.0, height / 2.0],
        ]
    }

    fn get_plot_points(&self) -> PlotPoints {
        let width = self.width;
        let height = self.height;
        vec![
            (-width / 2.0, -height / 2.0),
            (width / 2.0, -height / 2.0),
            (width / 2.0, height / 2.0),
            (-width / 2.0, height / 2.0),
        ]
        .into_iter()
        .map(|e| [e.0 + self.position.x, e.1 + self.position.y])
        .collect::<Vec<_>>()
        .into()
    }
}

impl Rect {
    pub fn contact(&self, ops: ObjectShape) -> Option<ContactInfo> {
        todo!()
    }
}
