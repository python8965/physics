use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleFormat, SizedSample, Stream, StreamConfig};
use std::fmt::Debug;
use tracing::{debug, error, info};

pub struct MusicPlayer {
    config: StreamConfig,
    device: Device,
    stream: Option<Stream>,
    format: SampleFormat,
}

impl Default for MusicPlayer {
    fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("failed to find a default output device");

        info!(dev = device.name().unwrap());
        let config = device.default_output_config().unwrap();
        let format = config.sample_format();

        Self {
            device,
            config: config.into(),
            format,
            stream: None,
        }
    }
}

impl MusicPlayer {
    fn run<T>(&mut self) -> Result<(), anyhow::Error>
    where
        T: SizedSample + FromSample<f32> + Debug,
    {
        let sample_rate = self.config.sample_rate.0 as f32;
        let channels = self.config.channels as usize;
        info!("as sample rate {} as channel {}", sample_rate, channels);
        // Produce a sinusoid of maximum amplitude.
        let mut sample_clock = 0f32;
        let mut next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| error!("an error occurred on stream: {}", err);

        let stream = self.device.build_output_stream(
            &self.config,
            move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                Self::write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )?;

        stream.play()?;

        self.stream.replace(stream);
        //std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }

    fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
    where
        T: Sample + FromSample<f32> + Debug,
    {
        for frame in output.chunks_mut(channels) {
            let value: T = T::from_sample(next_sample());
            for sample in frame.iter_mut() {
                *sample = value;
            }
        }
    }

    pub fn play_audio(&mut self) {
        match self.format {
            cpal::SampleFormat::F32 => self.run::<f32>(),
            cpal::SampleFormat::I16 => self.run::<i16>(),
            cpal::SampleFormat::U16 => self.run::<u16>(),
            _ => panic!("unsupported sample format"),
        }
        .expect("Run<T> ERROR");
    }
}
