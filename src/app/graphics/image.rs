use egui::epaint::ImageDelta;
use egui::plot::{PlotImage, PlotPoint};
use egui::{vec2, ColorImage, Context, TextureId, TextureOptions};

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
        let delta = ImageDelta::full(color_image, TextureOptions::default());

        ctx.tex_manager().write().set(TextureId::User(0), delta);

        let texture =
            ctx.load_texture("plot_demo", egui::ColorImage::example(), Default::default());
        Self {
            texture: vec![texture],
        }
    }

    pub fn get_plot_image(&mut self, index: usize, pos: PlotPoint, size: f64) -> PlotImage {
        let tex = &mut self.texture[index];

        let image = PlotImage::new(tex.id(), pos, (size as f32) * vec2(tex.aspect_ratio(), 1.0));

        image
    }
}
