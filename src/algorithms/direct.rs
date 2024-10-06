use rand::Rng;
use std::fmt::Debug;

use super::util::{is_zero, sample_exponential};
use crate::models::{Model, ModelEvent};

pub fn direct<M: Model + Debug>(
    model: M,
    initial_conditions: M::State,
    max_iters: u64,
    time_unit: &str,
) -> Vec<M::State> {
    println!("Gillespie's direct algorithm");
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
        let rate_total: f64 = events.iter().map(|e| e.rate()).sum();

        // End sim. if all rates are zero
        if is_zero(rate_total) {
            break;
        }

        // Sample time till next event
        let time_till_event = sample_exponential(rate_total, &mut rng).expect("nonzero rate_total");

        // Sample next event type
        let event = events
            .get({
                // Choose some point between 0 and the total rate
                let r2: f64 = rng.gen();
                let p = r2 * rate_total;

                // Figure out which event idx this corresponds to
                events
                    .iter()
                    .scan(0.0, |state, event| {
                        *state += event.rate();
                        Some(*state)
                    })
                    .position(|x| x >= p)
                    .expect("event rates add to rate_total")
            })
            .unwrap();

        // Perform the update
        state = model.update(&state, event);
        time += time_till_event;
        finished_iters += 1;
        states.push(state);
    }

    println!("Time elapsed: {time:.2?} {time_unit} ({finished_iters:?} iters)");
    println!("Final state: {state:?}");
    states
}
