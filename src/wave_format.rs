use std::{fs, path::Path};
use anyhow::{Context, Result, bail};
use std::fmt::{self, Formatter};
use crate::file_buffer::FileBuffer;

#[derive(Debug)]
pub(crate) struct WaveFormat {
    pub subchunk1_id: String,
    pub subchunk1_size: u32,
    pub audio_format: u16,
    pub num_of_channels: u16,
    pub sample_rate: u32,
    pub byte_rate: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

impl WaveFormat {
    /// Parse a RIFF WAV file `fmt` chunk and get a new WaveFormat. Will fail it the file is not PCM
    /// encoded.
    ///
    /// * `data`: The file buffer in bytes.
    pub fn new(buffer: &FileBuffer) -> Result<Self> {
        let subchunk1_id = buffer.get_string(12, 16)
            .context("Unable to parse the Subchunk1ID entry to a string")?;

        if subchunk1_id != "fmt " {
            bail!("The first WAV subchunk is not a format chunk ('fmt '), reported as '{subchunk1_id}'");
        }
        
        let subchunk1_size = buffer.get_u32(16);
        if subchunk1_size != 16 {
            bail!("The Subchunk1Size entry does not match the size expected for a PCM encoded WAV file");
        }

        let audio_format = buffer.get_u16(20);
        if audio_format != 1 {
            bail!("The file is not PCM encoded");
        }

        let num_of_channels = buffer.get_u16(22);
        let sample_rate = buffer.get_u32(24);
        let byte_rate = buffer.get_u32(28);
        let block_align = buffer.get_u16(32);
        let bits_per_sample = buffer.get_u16(34);

        let calculated_byte_rate = num_of_channels as u32 * sample_rate * bits_per_sample as u32/8;
        if byte_rate != calculated_byte_rate {
            bail!("The ByteRate entry doesnt not match its theorical value (reported: {byte_rate} calculated: {calculated_byte_rate}");
        }

        let calculated_block_align = num_of_channels * bits_per_sample/8;
        if block_align != calculated_block_align {
            bail!("The BlockAlign entry doesnt not match its theorical value (reported: {block_align} calculated: {calculated_block_align}");
        }

        
        Ok(Self{
            subchunk1_id,
            subchunk1_size,
            audio_format,
            num_of_channels,
            sample_rate,
            byte_rate,
            block_align,
            bits_per_sample,
        })
    }
}
