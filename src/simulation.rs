use egui::epaint::{CircleShape, RectShape};
use egui::plot::{Line, PlotPoint, PlotPoints, PlotUi, Polygon};
use egui::{plot, Color32, Pos2, Rect, RichText, Shape, Vec2, WidgetText};
use std::f64::consts::TAU;
use std::iter::Sum;
use std::ops::Add;
use tracing_subscriber::fmt::time;

pub const SIM: &[SimulationType] = &[SimulationType::BaseSim];

type Float = f32;

pub enum SimulationType {
    BaseSim,
}

impl SimulationType {
    pub fn as_str(&self) -> &str {
        match self {
            SimulationType::BaseSim => "BaseSim",
        }
    }

    pub fn as_func(&self) -> Simulation {
        match self {
            SimulationType::BaseSim => basic_sim(),
        }
    }
}

fn basic_sim() -> Simulation {
    let a = PhysicsObject {
        mass: 5.0,
        shape: DrawShapeType::Box,
        scale: None,
        force: vec![Vec2::new(0.0, -9.8)],
        ..PhysicsObject::default()
    };

    Simulation::from(vec![a])
}

#[derive(Debug)]
pub enum DrawShapeType {
    Circle,
    Box,
}

#[derive()]
pub struct PhysicsObject {
    pub position: Pos2,
    pub momentum: Vec2,

    pub force: Vec<Vec2>,

    pub mass: Float,

    pub shape: DrawShapeType,
    pub scale: Option<Float>,
}

impl Default for PhysicsObject {
    fn default() -> Self {
        Self {
            position: Pos2::new(0.0, 0.0),
            momentum: Default::default(),
            force: vec![],
            mass: 0.0,
            shape: DrawShapeType::Circle,
            scale: None,
        }
    }
}

impl PhysicsObject {
    fn velocity(&self) -> Vec2 {
        // p = mv -> v = p/m
        self.momentum / self.mass
    }

    fn get_scale(&self) -> Float {
        if self.scale.is_some() {
            self.scale.unwrap()
        } else {
            1.0 + (self.mass / 1.5)
        }
    }

    pub fn get_draw_shape(&self) -> PlotPoints {
        let scale = self.get_scale();

        match self.shape {
            DrawShapeType::Circle => PlotPoints::from_parametric_callback(
                move |t| {
                    (
                        t.sin() + self.position.x as f64,
                        t.cos() + self.position.y as f64,
                    )
                },
                0.0..TAU,
                512,
            ),

            DrawShapeType::Box => vec![
                [self.position.x - scale, self.position.y - scale],
                [self.position.x - scale, self.position.y + scale],
                [self.position.x + scale, self.position.y + scale],
                [self.position.x + scale, self.position.y - scale],
            ]
            .into_iter()
            .map(|e| [e[0] as f64, e[1] as f64])
            .collect::<Vec<_>>()
            .into(),
        }
    }

    pub fn get_draw_texts(&self, zoom: f64) -> Option<plot::Text> {
        let scale = self.get_scale();

        let font_size = (scale / zoom as f32) * 200.0;
        let mut text = RichText::new(format!(
            "Pos : {:?}\nVelocity : {:?}\nMass : {:?}\nForce(s) : {:?}",
            self.position,
            self.velocity(),
            self.mass,
            self.force
        ));

        if font_size < 12.0 {
            text = text.size(12.0);
        } else if font_size > 64.0 {
            return None;
        } else {
            text = text.size(font_size);
        }

        Some(plot::Text::new(
            PlotPoint::new(self.position.x, self.position.y),
            text,
        ))
    }
}

#[derive()]
pub struct Simulation {
    children: Vec<PhysicsObject>,
    active: bool,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            children: vec![],
            active: true,
        }
    }

    pub fn from(children: Vec<PhysicsObject>) -> Simulation {
        Simulation {
            children,
            active: true,
        }
    }

    pub fn finish(&mut self) {
        self.active = false;
    }

    pub fn running(&self) -> bool {
        self.active
    }

    pub fn step(&mut self, dt: Float) {
        for child in &mut self.children {
            physics_system(dt, child);
        }
    }

    pub fn draw(&mut self, plot_ui: &mut PlotUi) {
        let zoom = plot_ui.plot_bounds().width();
        for child in &mut self.children {
            let points = child.get_draw_shape();
            plot_ui.polygon(Polygon::new(points));
            if let Some(text) = child.get_draw_texts(zoom) {
                plot_ui.text(text);
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

fn physics_system(dt: Float, obj: &mut PhysicsObject) {
    obj.position = {
        let sigma_force: Vec2 = obj.force.sum_only(); // ΣF

        // ΣF = dp / dt
        // 우리는 운동량 p를 원한다
        // dp = ΣF * dt

        let delta_momentum = sigma_force * dt;
        obj.momentum += delta_momentum;

        // ds = v * dt

        let delta_position = obj.velocity() * dt;

        obj.position + delta_position
    };
}
