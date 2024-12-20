// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::{Context, Result, bail};
use crate::file_buffer::FileBuffer;

#[derive(Debug)]
pub struct WaveData {
    pub subchunk2_id: String,
    pub subchunk2_size: u32,
}

impl WaveData {
    pub fn new(buffer: &FileBuffer) -> Result<Self> {
        let subchunk2_id = buffer.get_string(36, 40)
            .context("Unable to parse the Subchunk2ID entry to a string")?;

        if subchunk2_id != "data" {
            bail!("The second WAV subchunk is not a data chunk ('data'), reported as '{subchunk2_id}'");
        }

        let subchunk2_size = buffer.get_u32(40);

        let calculated_subchunk2_size = buffer.len() - 44;
        if subchunk2_size != calculated_subchunk2_size.try_into().expect("usize is not big enought to store the size of the sound data") {
            bail!("The Subchunk2Size entry doesnt not match its theorical value (reported: {subchunk2_size} calculated: {calculated_subchunk2_size}");
        }

        Ok(Self{
            subchunk2_id,
            subchunk2_size,
        })
    }
}
