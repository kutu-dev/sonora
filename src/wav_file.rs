// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;
use anyhow::{Result, bail};
use std::time::Duration;
use fmt::Debug;
use rodio::source::Source;
use std::fmt::{self, Formatter};
use crate::{
    file_buffer::FileBuffer,
    riff_header::RiffHeader,
    wave_format::WaveFormat,
    wave_data::WaveData,
};

const MIN_I24: f64 = (-(2_i32.pow(24)/2)) as f64;
const MAX_I24: f64 = (2_i32.pow(24)/2 - 1) as f64;

pub struct WavFile {
    pub riff_header: RiffHeader,
    pub wave_format: WaveFormat,
    pub wave_data: WaveData,
    pub sound_data: Vec<u8>,
    pub duration: u64,
    index: usize,
}

impl WavFile {
    pub fn new(file_path: &Path) -> Result<Self> {
        let buffer = FileBuffer::new(&file_path)?;

        let riff_header = RiffHeader::new(&buffer)?;
        let wave_format = WaveFormat::new(&buffer)?;
        let wave_data = WaveData::new(&buffer)?;
        let sound_data = buffer.data[44..].to_vec();

        let calculated_chunk_size = 4 + (8 + wave_format.subchunk1_size) + (8 + wave_data.subchunk2_size);
        if riff_header.chunk_size != calculated_chunk_size {
            bail!("The ChunkSize entry of the file do not match the value calculated by the 'fmt ' and 'data' subchunks (reported: {0} calculated: {calculated_chunk_size})", riff_header.chunk_size);
        }

        let duration = sound_data.len() as u64 / wave_format.num_of_channels as u64 / (wave_format.bits_per_sample as u64 / 8) / wave_format.sample_rate as u64;

        Ok(Self{
            riff_header,
            wave_format,
            wave_data,
            sound_data,
            duration,
            index: 0,
        })
    }
}

impl Debug for WavFile {
    /// Custom formatter used to avoid printing the `sound_data` entry of the struct.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WaveFile")
            .field("riff_header", &self.riff_header)
            .field("wave_format", &self.wave_format)
            .field("wave_data", &self.wave_data)
            .field("duration", &self.duration)
            .finish()
    }
}

fn to_i16_range(value: f64, min: f64, max: f64) -> i16 {
    // `i16` max and min numbers
    const new_max: f64 = 32767.0;
    const new_min: f64 = -32768.0;

    ((((value - min) * (new_max - new_min)) / (max- min)) + new_min) as i16
}

impl Iterator for WavFile {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.sound_data.len() {
            self.duration = 0;
            return None;
        }

        match self.wave_format.bits_per_sample {
            8 => {
                let value = self.sound_data[self.index] as f64;
                self.index += 1;

                Some(to_i16_range(value, u8::MIN.into(), u8::MAX.into()))
            }

            16 => {
                let segment_1 = self.sound_data[self.index] as i16;
                let segment_2 = (self.sound_data[self.index + 1] as i16) << 8;
                self.index += 2;

                Some(segment_1 | segment_2)
            }

            24 => {
                let segment_1 = self.sound_data[self.index] as i32;
                let segment_2 = (self.sound_data[self.index + 1] as i32) << 8;
                let segment_3 = (self.sound_data[self.index + 2] as i32) << 16;
                self.index += 3;

                Some(to_i16_range((segment_1 | segment_2 | segment_3) as f64, MIN_I24, MAX_I24))
            }

            32 => {
                let segment_1 = self.sound_data[self.index] as i32;
                let segment_2 = (self.sound_data[self.index + 1] as i32) << 8;
                let segment_3 = (self.sound_data[self.index + 2] as i32) << 16;
                let segment_4 = (self.sound_data[self.index + 3] as i32) << 24;
                self.index += 4;

                Some(to_i16_range((segment_1 | segment_2 | segment_3 | segment_4) as f64, u32::MIN.into(), u32::MAX.into()))
            }

            num_of_bits => {
                println!("{num_of_bits}bit length PCM encoding is not compatible");
                None
            }
        }
    }
}

impl Source for WavFile {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        self.wave_format.num_of_channels
    }

    fn sample_rate(&self) -> u32 {
        self.wave_format.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::new(self.duration, 0))
    }
}
