use crate::app::io::get_file;
use egui::mutex::Mutex;
use egui::{Slider, Ui, Widget};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::{BufReader, Cursor};
use std::mem;
use std::sync::Arc;
use std::time::Duration;

pub struct MusicPlayer {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    current_sink: Sink,

    data: Arc<Mutex<Vec<u8>>>,

    volume: f32,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let current_sink = Sink::try_new(&stream_handle).unwrap();
        let volume = 0.25;
        current_sink.set_volume(volume);
        Self {
            stream,
            stream_handle,
            current_sink,
            data: Arc::new(Mutex::new(vec![])),
            volume,
        }
    }
}

impl MusicPlayer {
    fn run(data: Arc<Mutex<Vec<u8>>>, name: impl Into<String>) {
        get_file(name.into(), data);
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let Self {
            current_sink, data, ..
        } = self;

        let sink = current_sink;
        let raw_data = &mut *data.lock();

        if !raw_data.is_empty() {
            let file = BufReader::new(Cursor::new(mem::take(raw_data)));

            // Decode that sound file into a source
            let source = Decoder::new(file)
                .unwrap()
                .fade_in(Duration::from_millis(1000));

            sink.append(source.convert_samples::<f32>());
        }

        if !sink.empty() {
            ui.horizontal(|ui| {
                if ui
                    .button(if sink.is_paused() { "▶" } else { "⏸" })
                    .clicked()
                {
                    if sink.is_paused() {
                        sink.play()
                    } else {
                        sink.pause()
                    }
                }

                if Slider::new(&mut self.volume, 0.0..=1.0)
                    .text("Volume")
                    .ui(ui)
                    .dragged()
                {
                    sink.set_volume(self.volume);
                }
            });
        }

        if ui.button("Play? (Sound Warning)").clicked() {
            Self::run(data.clone(), "/suzume.ogg");
        }
    }
}
