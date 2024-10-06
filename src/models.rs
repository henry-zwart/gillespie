use std::fmt::Debug;

pub mod seir;
pub mod sir;

pub use sir::*;

pub trait ModelEvent {
    fn rate(&self) -> f64;
}

pub trait Model {
    type State: Debug + Clone + Copy;
    type Event: ModelEvent;

    fn events(&self, state: &Self::State) -> impl Iterator<Item = Self::Event>;

    fn update(&self, state: &Self::State, event: &Self::Event) -> Self::State;
}
