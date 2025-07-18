
pub trait Handler<I, O> {
    /// # Errors
    ///
    /// Will exit the program with the return error, for example on validation
    fn handle(&self, state: I) -> Result<O, clap::Error>;
}

pub trait HandlerMut<I, O> {
    /// # Errors
    ///
    /// Will exit the program with the return error, for example on validation
    fn handle(&mut self, state: I) -> Result<O, clap::Error>;
}

/// Chain an input though multiple handlers.
///
/// # Examples
///
/// ```
/// use crate::args::Args;
/// use clap::{Error, Parser};
/// use crate::handler::state::{SolvedState, State};
///
/// let input = Args::parse();
/// let state = State::new(input);
///
/// let solvedState = chain!(state, [ValidateHandler, SanitizeHandler, SolverHandler]);
///
/// if let Some(err) = solvedState.err() {
///     err.exit();
/// }
///
/// ```
#[macro_export]
macro_rules! chain {
    ($state:ident, [$first:expr, $( $handler:expr ),*]) => {
        {
            $first.handle($state)
            $(
                .and_then(|s| $handler.handle(s))
            )*
        }
    };
}
