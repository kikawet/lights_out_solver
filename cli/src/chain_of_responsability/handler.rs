use super::state::State;

pub trait Handler {
    /// # Errors
    ///
    /// Will exit with this return error Error, for example on validation
    fn handle(&self, state: State) -> Result<State, clap::Error>;
}
