use super::{Model, ModelEvent};

#[derive(Clone, Copy, Debug)]
pub struct SirPopulation {
    pub susceptible: u64,
    pub infected: u64,
    pub recovered: u64,
}

impl From<[u64; 3]> for SirPopulation {
    fn from(value: [u64; 3]) -> Self {
        Self {
            susceptible: value[0],
            infected: value[1],
            recovered: value[2],
        }
    }
}

pub enum SirCompartment {
    Susceptible,
    Infected,
    Recovered,
}

pub enum SirEvent {
    Transmission(f64),
    Recovery(f64),
    Birth(f64),
    NaturalDeath(SirCompartment, f64),
}

impl ModelEvent for SirEvent {
    fn rate(&self) -> f64 {
        match self {
            Self::Transmission(beta) => *beta,
            Self::Recovery(gamma) => *gamma,
            Self::Birth(mu) => *mu,
            Self::NaturalDeath(_, mu) => *mu,
        }
    }
}

#[derive(Debug)]
pub struct Sir {
    beta: f64,
    gamma: f64,
    mu: f64,
}

impl Sir {
    pub fn new(beta: f64, gamma: f64, mu: f64) -> Self {
        Self { beta, gamma, mu }
    }
}

impl Model for Sir {
    type State = SirPopulation;
    type Event = SirEvent;

    fn events(&self, state: &SirPopulation) -> impl Iterator<Item = Self::Event> {
        let s = state.susceptible as f64;
        let i = state.infected as f64;
        let r = state.recovered as f64;
        let n = s + i + r;
        vec![
            SirEvent::Transmission(self.beta * s * i / n),
            SirEvent::Recovery(self.gamma * i),
            SirEvent::Birth(self.mu * n),
            SirEvent::NaturalDeath(SirCompartment::Susceptible, self.mu * s),
            SirEvent::NaturalDeath(SirCompartment::Infected, self.mu * i),
            SirEvent::NaturalDeath(SirCompartment::Recovered, self.mu * r),
        ]
        .into_iter()
    }

    fn update(&self, state: &Self::State, event: &Self::Event) -> Self::State {
        let mut new_state = *state;
        match event {
            SirEvent::Transmission(_) => {
                assert!(new_state.susceptible > 0);
                new_state.susceptible -= 1;
                new_state.infected += 1;
            }
            SirEvent::Recovery(_) => {
                assert!(new_state.infected > 0);
                new_state.infected -= 1;
                new_state.recovered += 1;
            }
            SirEvent::Birth(_) => {
                new_state.susceptible += 1;
            }
            SirEvent::NaturalDeath(SirCompartment::Susceptible, _) => {
                assert!(new_state.susceptible > 0);
                new_state.susceptible -= 1;
            }
            SirEvent::NaturalDeath(SirCompartment::Infected, _) => {
                assert!(new_state.infected > 0);
                new_state.infected -= 1;
            }
            SirEvent::NaturalDeath(SirCompartment::Recovered, _) => {
                assert!(new_state.recovered > 0);
                new_state.recovered -= 1;
            }
        }
        new_state
    }
}
