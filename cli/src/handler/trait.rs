pub trait Handler<I, O> {
    /// # Errors
    ///
    /// Will exit with this return error Error, for example on validation
    fn handle(&self, state: I) -> Result<O, clap::Error>;
}
