use crate::{args::Input, solvers::board::Board};

pub struct State<'a> {
    pub input: Input,
    pub board: &'a dyn Board,
}

pub trait Worker<'a> {
    fn execute(&'a mut self, state: &mut State) {
        self.handle(state);

        if let Some(next) = self.next() {
            next.execute(state);
        }
    }

    fn handle(&mut self, state: &mut State);
    fn set_next(&'a mut self, next: &'a mut dyn Worker<'a>) -> &'a mut dyn Worker<'a>;
    fn next(&'a mut self) -> &mut Option<&mut dyn Worker<'a>>;
}
