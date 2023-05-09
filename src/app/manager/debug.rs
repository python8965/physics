use crate::app::graphics::define::BoxedPlotDraw;

pub struct DebugShapeStorage{
    pub debug_shape: Vec<Vec<BoxedPlotDraw>>,
}

impl Default for DebugShapeStorage{
    fn default() -> Self {
        Self{
            debug_shape: vec![Vec::new()],
        }
    }
}

impl DebugShapeStorage{
    pub fn add_debug_shape(&mut self, shape: BoxedPlotDraw){
        self.debug_shape.last_mut().unwrap().push(shape);
    }

    pub fn update(&mut self){
        self.debug_shape.push(Vec::new());
    }

    pub fn get_debug_shape(&mut self, index: usize) -> &Vec<BoxedPlotDraw>{
        &self.debug_shape[index]
    }
}