//! Farts - port of fart.js
//! Example:
//!
//! ```
//! use farts::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // you can discover more assets in farts::fart module
//!     farts::play(TOOT)?;
//!     Ok(())
//! }
//! ```
use std::{fmt::Display, io::Cursor};

use rodio::{Decoder, OutputStream, Sink};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/farts/"]
struct FartAssets;

/// Different types of fartes
pub mod fart {
    /// Fart type which stores path to the asset
    pub type Fart = &'static str;

    pub const TOOT: Fart = "fart1.wav";
    pub const RIPPER: Fart = "fart2.wav";
    pub const PLOP: Fart = "fart3.wav";
    pub const SQUIT: Fart = "fart4.wav";
    pub const RASPBERRY: Fart = "fart5.wav";
    pub const SQUAT: Fart = "fart6.wav";
    pub const TUPPENCE: Fart = "fart7.wav";
    pub const LIFTOFF: Fart = "fart8.wav";
    pub const TRUMPET: Fart = "fart9.wav";
    pub const FIZZLER: Fart = "fart10.wav";
    pub const WINDY: Fart = "fart11.wav";
    pub const EINE: Fart = "fart12.wav";
    pub const FARTCEPTION: Fart = "fart13.wav";
    pub const FARTPOINT1: Fart = "fart14.wav";
}

/// Stores errors which can occure while playing fart sound
#[derive(Debug)]
pub enum FartingError {
    IO(std::io::Error),
    DecodingError(rodio::decoder::DecoderError),
    StreamingError(rodio::StreamError),
    PlayingError(rodio::PlayError),
}

impl Display for FartingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            FartingError::IO(a) => format!("{}", a),
            FartingError::DecodingError(a) => format!("{}", a),
            FartingError::StreamingError(a) => format!("{}", a),
            FartingError::PlayingError(a) => format!("{}", a),
        };

        write!(f, "{}", msg)
    }
}

impl std::error::Error for FartingError {}

/**
Play a fart sound.

###### Arguments:
- `fart` - The fart sound to play.
You can discover the available fart sounds by looking in the `farts::fart` module.

###### Example
``` rust
farts::play(farts::fart::WINDY)
    .unwrap();
```
 */
pub fn play(fart: fart::Fart) -> Result<(), FartingError> {
    let asset = FartAssets::get(fart).unwrap();

    let cursor = Cursor::new(asset.data);
    let source = Decoder::new(cursor)
        .map_err(|e| FartingError::DecodingError(e))?;

    let (_stream, stream_handle) =
        OutputStream::try_default().map_err(|e| FartingError::StreamingError(e))?;

    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| FartingError::PlayingError(e))?;

    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}

pub mod prelude {
    pub use crate as farts;
    pub use crate::fart::*;
}
