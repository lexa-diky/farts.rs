#[cfg(test)]
mod tests {
    use farts::*;

    #[test]
    fn it_works() -> Result<(), Box<dyn std::error::Error>> {
        play(fart::TOOT)?;

        Ok(())
    }
}
