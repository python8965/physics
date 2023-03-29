use crate::app::io::get_file;
use egui::mutex::Mutex;
use egui::{Slider, Ui, Widget};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::{BufReader, Cursor};
use std::mem;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

pub struct MusicPlayer {
    stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    current_sink: Option<Sink>,

    data: Arc<Mutex<Vec<u8>>>,

    volume: f32,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let volume = 0.25;

        Self {
            stream: None,
            stream_handle: None,
            current_sink: None,
            data: Arc::new(Mutex::new(vec![])),
            volume,
        }
    }
}

impl MusicPlayer {
    fn init(&mut self) {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let current_sink = Sink::try_new(&stream_handle).unwrap();
        current_sink.set_volume(self.volume);

        self.current_sink.replace(current_sink);
        self.stream.replace(stream);
        self.stream_handle.replace(stream_handle);
    }

    fn run(&self, name: impl Into<String>) {
        get_file(name.into(), self.data.clone());
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        {
            let Self {
                current_sink, data, ..
            } = self;

            let raw_data = &mut *data.lock();

            if let Some(sink) = current_sink {
                if !raw_data.is_empty() {
                    info!("{:?}", sink.is_paused());

                    let file = BufReader::new(Cursor::new(mem::take(raw_data)));

                    // Decode that sound file into a source
                    let source = Decoder::new(file)
                        .unwrap()
                        .fade_in(Duration::from_millis(1000));

                    sink.append(source.convert_samples::<f32>());

                    sink.play();
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
            }
        }

        if ui.button("Play? (Sound Warning)").clicked() {
            if self.current_sink.is_none() {
                self.init();
            }

            self.run("audio/suzume.ogg");
        }
    }
}
