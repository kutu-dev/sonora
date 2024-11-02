use std::{fs, path::Path};
use anyhow::{Context, Result, bail};
use std::fmt::{self, Formatter};
use crate::file_buffer::FileBuffer;

#[derive(Debug)]
pub(crate) struct RiffHeader {
    pub chunk_id: String,
    pub chunk_size: u32,
    pub format: String,
}

impl RiffHeader {
    /// Parse the header of a RIFF file and get a new RiffHeader.
    pub fn new(buffer: &FileBuffer) -> Result<Self> {
        let chunk_id = buffer.get_string(0, 4)
            .context("Unable to parse the ChunkID entry to a string")?;

        if chunk_id != "RIFF" { 
            bail!("The file is not a WAV file, its chunk ID is '{chunk_id}' instead of 'RIFF'");
        }

        let chunk_size = buffer.get_u32(4);
        if chunk_size + 8 != buffer.len().try_into().expect("usize is not big enought to store the size of the file") {
            bail!("The ChunkSize entry does not match the size of the file")
        }

        let format = buffer.get_string(8, 12)
            .context("Unable to parse the format to a string")?;

        if format != "WAVE" { 
            bail!("The file is not a WAV file, its format has been reported as '{format}'");
        }

        Ok(Self {
            chunk_id,
            chunk_size,
            format
        })
    }
}

