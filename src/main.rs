use hyprland::{
    data::Clients,
    shared::{HyprData as _, HyprDataVec as _},
};

mod args;
use args::*;

mod target;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let args = HyprFocusArgs::parse();

    args.target.execute();

    let clients = Clients::get()?.to_vec();
    tracing::info!(?clients, "got clients");

    Ok(())
}
