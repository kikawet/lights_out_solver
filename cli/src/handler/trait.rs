pub trait Handler<I, O> {
    /// # Errors
    ///
    /// Will exit the program with the return error, for example on validation
    fn handle(&self, state: I) -> Result<O, clap::Error>;
}
