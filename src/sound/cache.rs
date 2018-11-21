use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use rodio::{self, Source};
use rodio::buffer::SamplesBuffer;

struct Sound {
    channels: u16,
    samples_rate: u32,
    samples: Vec<f32>,
}

impl Sound {
    pub fn to_buffer(&self) -> SamplesBuffer<f32> {
        SamplesBuffer::new(self.channels, self.samples_rate, self.samples.clone())
    }
}

pub struct SoundCache {
    cache: HashMap<String, Sound>,
    endpoint: rodio::Device,
}

impl SoundCache {
    pub fn new() -> Self {
        SoundCache {
            cache: HashMap::new(),
            endpoint: rodio::default_output_device().unwrap(),
        }
    }

    fn cache_sound(&mut self, path: &str) {
        let file = File::open(&path).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

        let sample = Sound {
            channels: source.channels(),
            samples_rate: source.sample_rate(),
            samples: source.convert_samples().collect::<Vec<f32>>(),
        };

        self.cache.insert(path.to_string(), sample);
    }

    pub fn play(&mut self, path: &str) {
        if !self.cache.contains_key(path) {
            self.cache_sound(path);
        }

        let sound = self.cache.get(path).unwrap();
        rodio::play_raw(&self.endpoint, sound.to_buffer().amplify(0.2));
    }
}
