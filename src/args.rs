pub use clap::Parser as _;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct HyprFocusArgs {
    /// Target: what to focus
    #[command(subcommand)]
    pub target: Target,
}

#[derive(Subcommand, Debug)]
pub enum Target {
    #[command(arg_required_else_help = true)]
    InitialClass(TargetArgs),
    #[command(arg_required_else_help = true)]
    Class(TargetArgs),
    #[command(arg_required_else_help = true)]
    Title(TargetArgs),
}

#[derive(Debug, Args, Clone)]
#[command(flatten_help = true)]
pub struct TargetArgs {
    #[command(flatten)]
    matcher: MatchArgs,

    #[command(subcommand)]
    starter: Option<OpenArgs>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum OpenArgs {
    #[command(arg_required_else_help = true)]
    Start(CommandArgs),
}

#[derive(Debug, Args, Clone)]
pub struct CommandArgs {
    command: String,
}

#[derive(Debug, Args, Clone)]
pub struct MatchArgs {
    #[arg(long, short, value_enum,default_value_t = MatchArea::Exact)]
    area: MatchArea,

    #[arg(long, short, value_enum, default_value_t = MatchMode::Insensitive)]
    mode: MatchMode,

    matcher: String,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum MatchArea {
    Exact,
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum MatchMode {
    CaseSensitive,
    Insensitive,
}

impl std::fmt::Display for MatchArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::fmt::Display for MatchMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}
