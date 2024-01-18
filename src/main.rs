extern crate hound;
extern crate rustfft;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FftPlanner;
use std::f32::consts::PI;
use std::i16;

fn main() {
    let amplitude = i16::MAX as f32;
    let frequency = 440.0;
    let duration = 2.0;

    let sample_rate = 44100;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples: Vec<_> = (0..num_samples).map(|i| {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * frequency * 2.0 * PI).sin();
        (sample * amplitude) as i16
    }).collect();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: sample_rate as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for sample in samples.iter() {
        writer.write_sample(*sample).unwrap();
    }
    writer.finalize().unwrap();
}

