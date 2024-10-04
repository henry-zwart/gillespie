use rand::Rng;

use crate::model::{InfectionModel, Population, Update};

pub fn direct<M: Population + Update + InfectionModel>(mut model: M) {
    let mut rng = rand::thread_rng();
    let n_iters = 1_000_000;

    let mut time = 0.0;

    let mut completed_iters = 0;
    for _ in 1..=n_iters {
        let rates = model.rates();
        let mut total_rate = None;
        for r in &rates {
            if let Some(rate) = r {
                total_rate = Some(total_rate.unwrap_or(0.0) + rate)
            }
        }
        if total_rate == None {
            break;
        }
        let total_rate = total_rate.unwrap();
        let rand1: f64 = rng.gen();
        let time_till_next = (-1.0 * rand1.ln()) / total_rate;

        // Find next event
        let p: f64 = total_rate * rng.gen::<f64>();
        let event = model.event_by_index(
            &rates
                .iter()
                .scan(0.0, |state, &x| {
                    *state += x.unwrap_or(0.0);
                    Some(*state)
                })
                .position(|x| x >= p)
                .expect("some event happens"),
        );

        model = model.update(event);
        time += time_till_next;
        completed_iters += 1;
    }

    println!("Time elapsed: {time:?} days ({completed_iters:?} iters)");
    println!("Model state: {model:?}");
}

#[cfg(test)]
mod test {
    pub use super::*;
    pub use crate::model::*;

    #[test]
    fn sir() {
        let sir = Sir::new(1000000, 1, 0, 1.0, 0.1);
        direct(sir);
        assert!(false);
    }
}
