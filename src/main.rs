use clap::Parser;
use audio::audio;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = audio::Config::default();
    let mut component = audio::new(config);
    component.initialize()?;
    println!("SigmaOS-audio ready");
    Ok(())
}
