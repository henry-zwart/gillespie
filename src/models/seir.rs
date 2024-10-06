use super::{Model, ModelEvent};

#[derive(Clone, Copy, Debug)]
pub struct SeirPopulation {
    pub susceptible: u64,
    pub exposed: u64,
    pub infected: u64,
    pub recovered: u64,
}

impl From<[u64; 4]> for SeirPopulation {
    fn from(value: [u64; 4]) -> Self {
        Self {
            susceptible: value[0],
            exposed: value[1],
            infected: value[2],
            recovered: value[3],
        }
    }
}

pub enum SeirCompartment {
    Susceptible,
    Exposed,
    Infected,
    Recovered,
}

pub enum SeirEvent {
    Transmission(f64),
    Infectious(f64),
    Recovery(f64),
    Birth(f64),
    NaturalDeath(SeirCompartment, f64),
}

impl ModelEvent for SeirEvent {
    fn rate(&self) -> f64 {
        match self {
            Self::Transmission(beta) => *beta,
            Self::Infectious(sigma) => *sigma,
            Self::Recovery(gamma) => *gamma,
            Self::Birth(mu) => *mu,
            Self::NaturalDeath(_, mu) => *mu,
        }
    }
}

#[derive(Debug)]
pub struct Seir {
    beta: f64,
    sigma: f64,
    gamma: f64,
    mu: f64,
}

impl Seir {
    pub fn new(beta: f64, sigma: f64, gamma: f64, mu: f64) -> Self {
        Self {
            beta,
            sigma,
            gamma,
            mu,
        }
    }
}

impl Model for Seir {
    type State = SeirPopulation;
    type Event = SeirEvent;

    fn events(&self, state: &SeirPopulation) -> impl Iterator<Item = Self::Event> {
        let s = state.susceptible as f64;
        let e = state.exposed as f64;
        let i = state.infected as f64;
        let r = state.recovered as f64;
        let n = s + e + i + r;
        vec![
            SeirEvent::Transmission(self.beta * s * i / n),
            SeirEvent::Infectious(self.sigma * e),
            SeirEvent::Recovery(self.gamma * i),
            SeirEvent::Birth(self.mu * n),
            SeirEvent::NaturalDeath(SeirCompartment::Susceptible, self.mu * s),
            SeirEvent::NaturalDeath(SeirCompartment::Exposed, self.mu * e),
            SeirEvent::NaturalDeath(SeirCompartment::Infected, self.mu * i),
            SeirEvent::NaturalDeath(SeirCompartment::Recovered, self.mu * r),
        ]
        .into_iter()
    }

    fn update(&self, state: &Self::State, event: &Self::Event) -> Self::State {
        let mut new_state = *state;
        match event {
            SeirEvent::Transmission(_) => {
                assert!(new_state.susceptible > 0);
                new_state.susceptible -= 1;
                new_state.exposed += 1;
            }
            SeirEvent::Infectious(_) => {
                assert!(new_state.exposed > 0);
                new_state.exposed -= 1;
                new_state.infected += 1;
            }
            SeirEvent::Recovery(_) => {
                assert!(new_state.infected > 0);
                new_state.infected -= 1;
                new_state.recovered += 1;
            }
            SeirEvent::Birth(_) => {
                new_state.susceptible += 1;
            }
            SeirEvent::NaturalDeath(SeirCompartment::Susceptible, _) => {
                assert!(new_state.susceptible > 0);
                new_state.susceptible -= 1;
            }
            SeirEvent::NaturalDeath(SeirCompartment::Exposed, _) => {
                assert!(new_state.exposed > 0);
                new_state.exposed -= 1;
            }
            SeirEvent::NaturalDeath(SeirCompartment::Infected, _) => {
                assert!(new_state.infected > 0);
                new_state.infected -= 1;
            }
            SeirEvent::NaturalDeath(SeirCompartment::Recovered, _) => {
                assert!(new_state.recovered > 0);
                new_state.recovered -= 1;
            }
        }
        new_state
    }
}
