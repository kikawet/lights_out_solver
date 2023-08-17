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
    /// Transformation are symectric so calling this twice with the same state is going to undo the changes
    /// It rotates the indices to the origin Top Left
    pub fn rotate_light_indices(indices: &mut [usize], cols: usize, rows: usize, location: Origin) {
        match location {
            Origin::TopRight => Self::reorder_cols(indices, rows, cols),
            Origin::BottomLeft => Self::reorder_rows(indices, rows, cols),
            Origin::BottomRight => Self::reorder_rows_cols(indices, rows, cols),
            Origin::TopLeft => { /*Do nothing 👀*/ }
        };
    }

    fn reorder_cols(indices: &mut [usize], _rows: usize, cols: usize) {
        for index in indices.iter_mut() {
            let col = *index % cols;
            let offset = cols - 1;

            *index += offset;
            *index -= 2 * col
        }
    }

    fn reorder_rows(indices: &mut [usize], rows: usize, cols: usize) {
        for index in indices.iter_mut() {
            let row = *index / cols;
            let offset = cols * rows - cols;

            *index += offset;
            *index -= 2 * row * cols;
        }
    }

    fn reorder_rows_cols(indices: &mut [usize], rows: usize, cols: usize) {
        for index in indices.iter_mut() {
            let row = *index / cols;
            let col = *index % cols;
            let offset = rows * cols - 1;

            *index += offset;
            *index -= 2 * (row * cols + col);
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
        lights.iter_mut().for_each(|val| *val -= 1);
        Self::rotate_light_indices(lights, cols, rows, origin);

        let simulation_steps = &mut state.input.simulation_steps;
        simulation_steps.iter_mut().for_each(|val| *val -= 1);
        Self::rotate_light_indices(simulation_steps, cols, rows, origin);

        state.board = Some(Box::new(Binary::new_from_positions(lights, cols, rows)));

        Ok(state)
    }
}
