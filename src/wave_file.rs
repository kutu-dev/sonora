use std::{fs, path::Path};
use anyhow::{Context, Result, bail};
use std::fmt::{self, Formatter};
use crate::{
    file_buffer::FileBuffer,
    riff_header::RiffHeader,
    wave_format::WaveFormat,
    wave_data::WaveData,
};

pub struct WaveFile {
    pub riff_header: RiffHeader,
    pub wave_format: WaveFormat,
    pub wave_data: WaveData,
    pub sound_data: Vec<u8>,
}

impl WaveFile {
    pub fn new(file_path: &Path) -> Result<Self> {
        let buffer = FileBuffer::new(&file_path)?;

        let riff_header = RiffHeader::new(&buffer)?;
        let wave_format = WaveFormat::new(&buffer)?;
        let wave_data = WaveData::new(&buffer, &wave_format)?;
        let sound_data = buffer.data[44..].to_vec();

        let calculated_chunk_size = 4 + (8 + wave_format.subchunk1_size) + (8 + wave_data.subchunk2_size);
        if riff_header.chunk_size != calculated_chunk_size {
            bail!("The ChunkSize entry of the file do not match the value calculated by the 'fmt ' and 'data' subchunks (reported: {0} calculated: {calculated_chunk_size})", riff_header.chunk_size);
        }

        Ok(Self{
            riff_header,
            wave_format,
            wave_data,
            sound_data,
        })
    }
}

impl fmt::Debug for WaveFile {
    /// Custom formatter used to avoid printing the `sound_data` entry of the struct.
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WaveFile")
            .field("RiffHeader", &self.riff_header)
            .field("WaveFormat", &self.wave_format)
            .field("WaveData", &self.wave_data)
            .finish()
    }
}
