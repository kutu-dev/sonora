// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::Path;
use sonora::WavFile;
use anyhow::Result;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = Path::new(&args[1]);
    let wav_file = WavFile::new(file_path)?;
    let duration = wav_file.duration;

    println!("{wav_file:#?}");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(wav_file.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(duration));

    Ok(())
}
