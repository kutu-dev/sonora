// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;
use sonora::WaveFile;
use anyhow::{Context, Result, bail};

fn main() -> Result<()> {
    let file_path = Path::new("square.wav");
    let wave_file = WaveFile::new(file_path)?;

    println!("{wave_file:#?}");

    Ok(())
}
