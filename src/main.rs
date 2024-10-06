use clap::Parser;
use cli::Algorithm;

mod cli;

use gillespie::{
    algorithms::{direct, first_reaction},
    models::{seir, sir},
};

fn main() {
    let cli_args = cli::Cli::parse();

    match cli_args.command {
        cli::Command::Run {
            algorithm,
            model,
            max_iters,
            time_unit,
        } => match model {
            cli::Model::Sir {
                susceptible,
                infected,
                recovered,
                beta,
                gamma,
                mu,
            } => {
                let alg = match algorithm {
                    Algorithm::Direct => direct,
                    Algorithm::FirstReaction => first_reaction,
                };
                let population = sir::SirPopulation::from([susceptible, infected, recovered]);
                let model = sir::Sir::new(beta, gamma, mu);
                alg(model, population, max_iters, &time_unit);
            }
            cli::Model::Seir {
                susceptible,
                exposed,
                infected,
                recovered,
                beta,
                sigma,
                gamma,
                mu,
            } => {
                let alg = match algorithm {
                    Algorithm::Direct => direct,
                    Algorithm::FirstReaction => first_reaction,
                };
                let population =
                    seir::SeirPopulation::from([susceptible, exposed, infected, recovered]);
                let model = seir::Seir::new(beta, sigma, gamma, mu);
                alg(model, population, max_iters, &time_unit);
            }
        },
        cli::Command::Benchmark => {}
    };
}

// #[derive(Parser)]
// #[command(version, about)]
// struct Cli {
//     #[arg(value_enum)]
//     algorithm: Algorithm,

//     #[arg(short, long, default_value_t = 1000)]
//     max_iters: u64,

//     #[arg(short, long, default_value_t = String::from("days"))]
//     time_unit: String,

//     #[command(subcommand)]
//     model: Model,
// }

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
// enum Algorithm {
//     Direct,
//     FirstReaction,
// }

// #[derive(Clone, Debug, PartialEq, PartialOrd, Subcommand)]
// enum Model {
//     Sir {
//         susceptible: u64,

//         #[arg(default_value_t = 1)]
//         infected: u64,

//         #[arg(default_value_t = 0)]
//         recovered: u64,

//         #[arg(short, long)]
//         beta: f64,

//         #[arg(short, long)]
//         gamma: f64,

//         #[arg(short, long)]
//         mu: f64,
//     },
// }

// fn main() {
//     let cli = Cli::parse();

//     let (model, population) = match cli.model {
//         Model::Sir {
//             susceptible,
//             infected,
//             recovered,
//             beta,
//             gamma,
//             mu,
//         } => (
//             models::Sir::new(beta, gamma, mu),
//             [susceptible, infected, recovered].into(),
//         ),
//     };

//     let _result = match cli.algorithm {
//         Algorithm::Direct => algorithms::direct(model, population, cli.max_iters, &cli.time_unit),
//         Algorithm::FirstReaction => {
//             algorithms::first_reaction(model, population, cli.max_iters, &cli.time_unit)
//         }
//     };
// }
