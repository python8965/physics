use crate::app::NVec2;

use egui::plot::{PlotImage, PlotPoint};
use egui::{vec2, ColorImage, Context, ImageData, TextureOptions};

#[derive(Clone, Default)]
pub struct ImageManager {
    texture: Vec<egui::TextureHandle>,
}

impl ImageManager {
    pub fn new(ctx: &Context) -> Self {
        let img_data = include_bytes!("../../../assets/equation01.png");
        let img = image::io::Reader::new(std::io::Cursor::new(img_data)) // Read the bytes of the image
            .with_guessed_format() // Guess the format of the image
            // Decode the image
            .unwrap()
            .decode()
            .unwrap(); // Unwrap the DecodingResult

        let size = [img.width() as _, img.height() as _];

        let img = img.to_rgba8();
        let pixels = img.as_flat_samples();

        let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        let image_data = ImageData::from(color_image);

        ctx.tex_manager().write().alloc(
            "plot_eq1".parse().unwrap(),
            image_data,
            TextureOptions::default(),
        );

        let texture = ctx.load_texture("plot_eq1", egui::ColorImage::example(), Default::default());

        Self {
            texture: vec![texture],
        }
    }

    pub fn get_plot_image(&mut self, index: usize, pos: NVec2, size: f64) -> PlotImage {
        let tex = &mut self.texture[index];

        let image = PlotImage::new(
            tex.id(),
            PlotPoint::from([pos.x, pos.y]),
            (size as f32) * vec2(tex.aspect_ratio(), 1.0),
        );

        image
    }
}
