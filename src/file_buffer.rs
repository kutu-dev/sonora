use std::{fs, path::Path};
use anyhow::{Context, Result, bail};
use std::fmt::{self, Formatter};

pub(crate) struct FileBuffer {
    pub data: Vec<u8>
}

impl FileBuffer {
    pub fn new(file_path: &Path) -> Result<Self> {
        let data = fs::read(file_path)
            .context("Failed to open the WAV file")?;

        if data.len() < 44 {
            bail!("The file is not big enought to have WAV data inside of it");
        }

        Ok(Self {
            data
        })
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn get_u32(&self, index: usize) -> u32 {
        u32::from_le_bytes(self.data[index..index+4].try_into().unwrap())
    }

    pub fn get_u16(&self, index: usize) -> u16 {
        u16::from_le_bytes(self.data[index..index+2].try_into().unwrap())
    }

    pub fn get_string(&self, start: usize, end: usize) -> Result<String> {
        Ok(String::from_utf8(self.data[start..end].to_vec())?)
    }
}

