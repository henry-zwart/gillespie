use rand::{rngs::ThreadRng, Rng};

use crate::models::Model;

fn is_zero(x: f64) -> bool {
    (x - 0.0).abs() < f64::EPSILON
}

fn sample_exponential(mean: f64, rng: &mut ThreadRng) -> Option<f64> {
    match is_zero(mean) {
        true => None,
        false => Some(-1.0 * rng.gen::<f64>().ln() / mean),
    }
}

pub fn direct<M: Model>(
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
        let rate_total: f64 = model.rates(&state).sum();
        let rates = model.rates(&state);

        // End sim. if all rates are zero
        if is_zero(rate_total) {
            break;
        }

        // Sample time till next event
        let r1: f64 = rng.gen();
        let time_till_event = -1.0 * r1.ln() / rate_total;

        // Sample next event type
        let event = M::get_event({
            // Choose some point between 0 and the total rate
            let r2: f64 = rng.gen();
            let p = r2 * rate_total;

            // Figure out which event idx this corresponds to
            &rates
                .scan(0.0, |state, x| {
                    *state += x;
                    Some(*state)
                })
                .position(|x| x >= p)
                .expect("event rates add to rate_total")
        });

        // Perform the update
        state = model.update(&state, &event);
        time += time_till_event;
        finished_iters += 1;
        states.push(state);
    }

    println!("Time elapsed: {time:.2?} {time_unit} ({finished_iters:?} iters)");
    println!("Final state: {state:?}");
    states
}

pub fn first_reaction<M: Model>(
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
        let next_event_times: Vec<Option<f64>> = model
            .rates(&state)
            .into_iter()
            .map(|r| sample_exponential(r, &mut rng))
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
        state = model.update(&state, &M::get_event(&event_idx));
        time += delta_t;
        finished_iters += 1;
        states.push(state);
    }

    println!("Time elapsed: {time:.2?} {time_unit} ({finished_iters:?} iters)");
    println!("Final state: {state:?}");
    states
}
