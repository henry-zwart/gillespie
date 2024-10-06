use std::fmt::Debug;

use super::util::sample_exponential;
use crate::models::{Model, ModelEvent};

pub fn first_reaction<M: Model + Debug>(
    model: M,
    initial_conditions: M::State,
    max_iters: u64,
    time_unit: &str,
) -> Vec<M::State> {
    println!("Gillespie's first-reaction algorithm");
    println!("Max iters: {max_iters:?}");
    println!("Model: {model:?}");
    println!("Initial population: {initial_conditions:?}");

    let mut rng = rand::thread_rng();
    let mut time = 0.0;

    let mut finished_iters = 0;
    let mut state = initial_conditions;
    let mut states = vec![state];
    for _ in 1..=max_iters {
        let events: Vec<M::Event> = model.events(&state).collect();
        let next_event_times: Vec<Option<f64>> = events
            .iter()
            .map(|e| sample_exponential(e.rate(), &mut rng))
            .collect();

        if next_event_times.iter().all(|t| t.is_none()) {
            println!("(Iter {finished_iters:?}): All rates are zero, finishing early.");
            break;
        }

        let (event_idx, delta_t) = next_event_times
            .iter()
            .enumerate()
            .filter_map(|(i, x)| x.map(|val| (i, val)))
            .min_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap())
            .expect("at least one not None");

        state = model.update(&state, events.get(event_idx).unwrap());
        time += delta_t;
        finished_iters += 1;
        states.push(state);
    }

    println!("Time elapsed: {time:.2?} {time_unit} ({finished_iters:?} iters)");
    println!("Final state: {state:?}");
    states
}
