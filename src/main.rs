use gillespie::{
    algorithms::direct,
    models::{Sir, SirPopulation},
};

fn main() {
    let initial_population = SirPopulation([1_000_000, 5, 0]);
    let sir = Sir::new(1.0, 0.1, 0.02 / 365.0);
    let _simulation = direct(sir, initial_population, 1_000_000);
}
