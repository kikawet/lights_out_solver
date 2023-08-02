use crate::{
    args::Origin,
    chain_of_responsability::{
        chainable::Chainable, handler::Handler, state::State, worker::Worker,
    },
    define_chainable,
    solvers::board::Binary,
};

define_chainable!(SanitizeWorker);

impl SanitizeWorker {
    /**
     * Transformation are symectric so calling this twice with the same state is going to undo the changes
     */
    pub fn rotate_light_indices(indices: &mut [usize], cols: usize, rows: usize, location: Origin) {
        match location {
            Origin::TopRight => Self::reorder_cols(indices, cols),
            Origin::BottomLeft => Self::reorder_rows(indices, rows),
            Origin::BottomRight => {
                Self::reorder_cols(indices, cols);
                Self::reorder_rows(indices, rows);
            }
            Origin::TopLeft => { /*Do nothing 👀*/ }
        };
    }

    fn reorder_rows(indices: &mut [usize], rows: usize) {
        let rows = rows as isize;

        for undex in indices.iter_mut() {
            let index = *undex as isize;

            let row = (index - 1) / rows;
            let offset = rows * (rows - 1 - 2 * row);
            *undex = (index + offset) as usize;
        }
    }

    fn reorder_cols(indices: &mut [usize], cols: usize) {
        let cols = cols as isize;

        for undex in indices.iter_mut() {
            let index = *undex as isize;

            let col = (index - 1) % cols;
            let offset = cols - 1 - 2 * col;
            *undex = (index + offset) as usize;
        }
    }
}

impl Handler for SanitizeWorker {
    fn handle(&mut self, mut state: State) -> Result<State, clap::error::Error> {
        let rows = state.input.rows;
        let cols = state.input.cols;
        let origin = state.input.origin_location;

        let lights = &mut state.input.lights;
        lights.sort_unstable();
        lights.dedup();
        Self::rotate_light_indices(lights, cols, rows, origin);
        lights.iter_mut().for_each(|val| *val -= 1);

        let simulation_steps = &mut state.input.simulation_steps;
        Self::rotate_light_indices(simulation_steps, cols, rows, origin);
        simulation_steps.iter_mut().for_each(|val| *val -= 1);

        state.board = Some(Box::new(Binary::new_from_positions(lights, cols, rows)));

        Ok(state)
    }
}
