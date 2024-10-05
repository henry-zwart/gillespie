use clap::{Parser, Subcommand, ValueEnum};

use gillespie::{algorithms, models};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(value_enum)]
    algorithm: Algorithm,

    #[arg(short, long, default_value_t = 1000)]
    max_iters: u64,

    #[arg(short, long, default_value_t = String::from("days"))]
    time_unit: String,

    #[command(subcommand)]
    model: Model,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    Direct,
    FirstReaction,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Subcommand)]
enum Model {
    Sir {
        susceptible: u64,

        #[arg(default_value_t = 1)]
        infected: u64,

        #[arg(default_value_t = 0)]
        recovered: u64,

        #[arg(short, long)]
        beta: f64,

        #[arg(short, long)]
        gamma: f64,

        #[arg(short, long)]
        mu: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    let (model, population) = match cli.model {
        Model::Sir {
            susceptible,
            infected,
            recovered,
            beta,
            gamma,
            mu,
        } => (
            models::Sir::new(beta, gamma, mu),
            models::SirPopulation([susceptible, infected, recovered]),
        ),
    };

    let _result = match cli.algorithm {
        Algorithm::Direct => algorithms::direct(model, population, cli.max_iters, &cli.time_unit),
        Algorithm::FirstReaction => {
            algorithms::first_reaction(model, population, cli.max_iters, &cli.time_unit)
        }
    };
}
