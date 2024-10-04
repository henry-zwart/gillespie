pub trait Population: std::fmt::Debug {
    fn n(&self) -> u64;
}

pub trait InfectionModel: Population {
    fn params(&self) -> Vec<f64>;

    fn r0(&self) -> f64;
}

pub trait Update: InfectionModel {
    type Event;

    fn events(&self) -> Vec<Self::Event>;

    fn event_by_index(&self, idx: &usize) -> Self::Event;

    fn rate(&self, event: &Self::Event) -> Option<f64>;

    fn rates(&self) -> Vec<Option<f64>> {
        self.events().iter().map(|e| self.rate(e)).collect()
    }

    fn update(self, event: Self::Event) -> Self;
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Sir {
    counts: [u64; 3],
    pub beta: f64,
    pub gamma: f64,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub enum SirEvent {
    Infection,
    Recovery,
}

impl Sir {
    pub fn new(susceptible: u64, infected: u64, recovered: u64, beta: f64, gamma: f64) -> Self {
        Self {
            counts: [susceptible, infected, recovered],
            beta,
            gamma,
        }
    }

    pub fn susceptible(&self) -> u64 {
        self.counts[0]
    }

    pub fn infected(&self) -> u64 {
        self.counts[1]
    }

    pub fn recovered(&self) -> u64 {
        self.counts[2]
    }
}

impl Population for Sir {
    fn n(&self) -> u64 {
        self.counts.iter().sum()
    }
}

impl InfectionModel for Sir {
    fn params(&self) -> Vec<f64> {
        vec![self.beta, self.gamma]
    }

    fn r0(&self) -> f64 {
        self.beta / self.gamma
    }
}

impl Update for Sir {
    type Event = SirEvent;

    fn events(&self) -> Vec<SirEvent> {
        vec![SirEvent::Infection, SirEvent::Recovery]
    }

    fn event_by_index(&self, idx: &usize) -> Self::Event {
        match idx {
            0 => SirEvent::Infection,
            1 => SirEvent::Recovery,
            _ => panic!(),
        }
    }

    fn rate(&self, event: &SirEvent) -> Option<f64> {
        match event {
            SirEvent::Infection => {
                if self.infected() == 0 || self.susceptible() == 0 {
                    None
                } else {
                    Some(
                        self.beta * (self.susceptible() as f64) * (self.infected() as f64)
                            / (self.n() as f64),
                    )
                }
            }
            SirEvent::Recovery => {
                if self.infected() == 0 {
                    None
                } else {
                    Some(self.gamma * (self.infected() as f64))
                }
            }
        }
    }

    fn update(self, event: SirEvent) -> Self {
        let new_counts = match event {
            SirEvent::Infection => {
                if self.susceptible() == 0 {
                    println!("{:?}", self);
                    println!("{:?}", self.rates());
                }
                assert_ne!(self.susceptible(), 0);
                let mut counts = self.counts;
                counts[0] -= 1;
                counts[1] += 1;
                counts
            }
            SirEvent::Recovery => {
                assert_ne!(self.infected(), 0);
                let mut counts = self.counts;
                counts[1] -= 1;
                counts[2] += 1;
                counts
            }
        };
        Sir {
            counts: new_counts,
            ..self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_sir() {
        let sir = Sir::new(1000, 1, 0, 1.0, 0.1);
        assert!(sir.n() == 1001);
        assert_eq!(sir.params().len(), 2);
        assert!((sir.r0() - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sir_rates() {
        let sir = Sir::new(10, 1, 0, 1.0, 0.1);

        let expected_inf_rate = 10.0 / 11.0;
        let calculated_inf_rate = sir.rate(&SirEvent::Infection).unwrap();
        assert!((calculated_inf_rate - expected_inf_rate).abs() < f64::EPSILON);

        let expected_rec_rate = 0.1;
        let calculated_rec_rate = sir.rate(&SirEvent::Recovery).unwrap();
        assert!((calculated_rec_rate - expected_rec_rate).abs() < f64::EPSILON);
    }
}
