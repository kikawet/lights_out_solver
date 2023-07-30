use log::debug;

use crate::{solvers::gf2, workers::worker::Worker};

struct SolverWorker<'a> {
    next: Option<&'a mut dyn Worker<'a>>,
}

impl<'a> Worker<'a> for SolverWorker<'a> {
    fn handle(&mut self, state: &mut crate::workers::worker::State) {
        debug!("Searching for solution ...");

        let solution = gf2::solve(state.board);
        debug!("Final solution: {:?}", &solution);
    }

    fn set_next(&'a mut self, next: &'a mut dyn Worker<'a>) -> &'a mut dyn Worker<'a> {
        self.next = Some(next);
        self
    }

    fn next(&'a mut self) -> &mut Option<&mut dyn Worker<'a>> {
        &mut self.next
    }
}
