use crate::app::NVec2;
use egui::plot::PlotPoints;
use std::f64::consts::TAU;

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
    pub fn circle(radius: f64) -> Self {
        Self::Circle(Circle { radius })
    }

    pub fn rect(width: f64, height: f64) -> Self {
        Self::Rect(Rect { width, height })
    }

    pub fn get_points(&self) -> Vec<[f64; 2]> {
        match self {
            Self::Circle(circle) => circle.get_points(),
            Self::Rect(rect) => rect.get_points(),
        }
    }

    pub fn get_plot_points(&self, position: NVec2) -> PlotPoints {
        match self {
            Self::Circle(circle) => circle.get_plot_points(position),
            Self::Rect(rect) => rect.get_plot_points(position),
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
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: Self::DEFAULT_RADIUS,
        }
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

    pub fn get_points(&self) -> Vec<[f64; 2]> {
        self._get_points(Self::DEFAULT_RESOLUTION)
    }

    pub fn get_plot_points(&self, position: NVec2) -> PlotPoints {
        PlotPoints::from_parametric_callback(
            move |t| {
                (
                    t.sin() * self.radius + position.x,
                    t.cos() * self.radius + position.y,
                )
            },
            0.0..TAU,
            Self::DEFAULT_RESOLUTION as usize,
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn get_points(&self) -> Vec<[f64; 2]> {
        let width = self.width;
        let height = self.height;
        vec![
            [-width / 2.0, -height / 2.0],
            [width / 2.0, -height / 2.0],
            [width / 2.0, height / 2.0],
            [-width / 2.0, height / 2.0],
        ]
    }

    pub fn get_plot_points(&self, position: NVec2) -> PlotPoints {
        let width = self.width;
        let height = self.height;
        vec![
            (-width / 2.0, -height / 2.0),
            (width / 2.0, -height / 2.0),
            (width / 2.0, height / 2.0),
            (-width / 2.0, height / 2.0),
        ]
        .into_iter()
        .map(|e| [e.0 + position.x, e.1 + position.y])
        .collect::<Vec<_>>()
        .into()
    }
}
