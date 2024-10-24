use clap::{Args, Parser, Subcommand};

/// Adder Interact CLI
#[derive(Default, PartialEq, Eq, Debug, Parser)]
#[command(version, about)]
#[command(propagate_version = true)]
pub struct InteractCli {
    #[command(subcommand)]
    pub command: Option<InteractCliCommand>,
}

/// Adder Interact CLI Commands
#[derive(Clone, PartialEq, Eq, Debug, Subcommand)]
pub enum InteractCliCommand {
    #[command(name = "deploy", about = "Deploy contract")]
    Deploy,
    #[command(name = "sizes", about = "Print pixel block sizes")]
    Sizes,
    #[command(name = "paint", about = "Paint!")]
    Paint(PaintArgs),
    #[command(name = "paint-all", about = "Paint every pixel")]
    PaintAll,
    #[command(name = "paint-rect", about = "Paint rectangle by rectangle")]
    PaintRectangles,
    #[command(name = "paint-rand", about = "Paint random points")]
    PaintRand,
    
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct PaintArgs {
    /// Repeat this number of times
    #[arg(short = 'c', long = "count", default_value = "1")]
    pub count: usize,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct UpgradeArgs {
    /// The value to add
    #[arg(short = 'v', long = "value")]
    pub value: u32,
}

#[derive(Default, Clone, PartialEq, Eq, Debug, Args)]
pub struct MultiDeployArgs {
    /// The number of contracts to deploy
    #[arg(short = 'c', long = "count")]
    pub count: usize,
}
