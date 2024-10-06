use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Run {
        #[arg(value_enum)]
        algorithm: Algorithm,

        #[command(subcommand)]
        model: Model,

        #[arg(short, long, default_value_t = 1000)]
        max_iters: u64,

        #[arg(short, long, default_value_t = String::from("days"))]
        time_unit: String,
    },
    Benchmark,
}

#[derive(Clone, ValueEnum)]
pub enum Algorithm {
    Direct,
    FirstReaction,
}

#[derive(Subcommand)]
pub enum Model {
    Sir {
        susceptible: u64,
        infected: u64,
        recovered: u64,

        #[arg(short, long)]
        beta: f64,

        #[arg(short, long)]
        gamma: f64,

        #[arg(short, long)]
        mu: f64,
    },
    Seir {
        susceptible: u64,
        exposed: u64,
        infected: u64,
        recovered: u64,

        #[arg(short, long)]
        beta: f64,

        #[arg(short, long)]
        sigma: f64,

        #[arg(short, long)]
        gamma: f64,

        #[arg(short, long)]
        mu: f64,
    },
}
