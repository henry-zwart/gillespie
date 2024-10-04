use std::fmt::Debug;

pub trait Model: Debug {
    type State: Copy + Clone + Debug;
    type Event;

    fn get_event(idx: &usize) -> Self::Event;

    fn rates(&self, state: &Self::State) -> impl Iterator<Item = f64>;

    fn update(&self, state: &Self::State, event: &Self::Event) -> Self::State;
}

#[derive(Clone, Copy, Debug)]
pub struct SirPopulation(pub [u64; 3]);

pub enum SirEvent {
    Transmission,
    Recovery,
    Birth,
    NaturalDeathSusceptible,
    NaturalDeathInfected,
    NaturalDeathRecovered,
}

#[derive(Debug)]
pub struct Sir {
    pub beta: f64,
    pub gamma: f64,
    pub mu: f64,
}

impl Sir {
    pub fn new(beta: f64, gamma: f64, mu: f64) -> Self {
        Self { beta, gamma, mu }
    }
}

impl Model for Sir {
    type State = SirPopulation;
    type Event = SirEvent;

    fn rates(&self, state: &Self::State) -> impl Iterator<Item = f64> {
        let rate_transmission = {
            self.beta * (state.0[0] as f64) * (state.0[1] as f64)
                / state.0.iter().sum::<u64>() as f64
        };
        let rate_recovery = self.gamma * state.0[1] as f64;
        let rate_birth = self.mu * state.0.iter().sum::<u64>() as f64;
        let rate_death_susceptible = self.mu * state.0[0] as f64;
        let rate_death_infected = self.mu * state.0[1] as f64;
        let rate_death_recovered = self.mu * state.0[2] as f64;
        [
            rate_transmission,
            rate_recovery,
            rate_birth,
            rate_death_susceptible,
            rate_death_infected,
            rate_death_recovered,
        ]
        .into_iter()
    }

    fn get_event(idx: &usize) -> Self::Event {
        match idx {
            0 => SirEvent::Transmission,
            1 => SirEvent::Recovery,
            2 => SirEvent::Birth,
            3 => SirEvent::NaturalDeathSusceptible,
            4 => SirEvent::NaturalDeathInfected,
            5 => SirEvent::NaturalDeathRecovered,
            _ => panic!(),
        }
    }

    fn update(&self, state: &Self::State, event: &Self::Event) -> Self::State {
        let mut new_state = *state;
        match event {
            SirEvent::Transmission => {
                assert_ne!(new_state.0[0], 0);
                new_state.0[0] -= 1;
                new_state.0[1] += 1;
            }
            SirEvent::Recovery => {
                assert_ne!(new_state.0[1], 0);
                new_state.0[1] -= 1;
                new_state.0[2] += 1;
            }
            SirEvent::Birth => {
                new_state.0[0] += 1;
            }
            SirEvent::NaturalDeathSusceptible => {
                assert_ne!(new_state.0[0], 0);
                new_state.0[0] -= 1;
            }
            SirEvent::NaturalDeathInfected => {
                assert_ne!(new_state.0[1], 0);
                new_state.0[1] -= 1;
            }
            SirEvent::NaturalDeathRecovered => {
                assert_ne!(new_state.0[2], 0);
                new_state.0[2] -= 1;
            }
        };
        new_state
    }
}
