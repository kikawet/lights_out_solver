use super::state::State;

pub trait Handler {
    fn handle(&mut self, state: State) -> Result<State, clap::Error>;
}
