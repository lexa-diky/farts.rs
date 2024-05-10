use farts::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // you can discover more assets in farts::fart module
    farts::play(TOOT)?;

    Ok(())
}