use gillespie::{
    algorithms::direct,
    models::{Sir, SirPopulation},
};

fn main() {
    let initial_population = SirPopulation([1_000_000, 5, 0]);
    let sir = Sir::new(1.0, 0.1);
    let _simulation = direct(sir, initial_population, 1_000_000_000);
}
