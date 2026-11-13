mod args;
use args::*;

mod clients;
mod target;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = HyprFocusArgs::parse();

    args.target.execute()?;

    Ok(())
}
