pub mod print;
pub mod sanitize_input;
pub mod simulator;
pub mod solver;
pub mod validate_range;

#[cfg(test)]
mod impl_chain_tests {
    use crate::args::Origin;

    use super::sanitize_input::SanitizeWorker;

    #[test]
    fn rotate_3x3() {
        let mut indices_bl = [0, 8];
        let mut indices_tr = [8, 0];
        let mut indices_br = [2, 6];
        let expected_tl = [6, 2];

        SanitizeWorker::rotate_light_indices(&mut indices_bl, 3, 3, Origin::BottomLeft);
        SanitizeWorker::rotate_light_indices(&mut indices_tr, 3, 3, Origin::TopRight);
        SanitizeWorker::rotate_light_indices(&mut indices_br, 3, 3, Origin::BottomRight);

        assert_eq!(indices_bl, expected_tl, "Convertion from BL to TL failed");
        assert_eq!(indices_tr, expected_tl, "Convertion from TR to TL failed");
        assert_eq!(indices_br, expected_tl, "Convertion from BR to TL failed");
    }

    #[test]
    fn rotate_5x2() {
        let mut indices_bl = [0, 9];
        let mut indices_tr = [9, 0];
        let mut indices_br = [1, 8];
        let expected_tl = [8, 1];

        SanitizeWorker::rotate_light_indices(&mut indices_bl, 2, 5, Origin::BottomLeft);
        SanitizeWorker::rotate_light_indices(&mut indices_tr, 2, 5, Origin::TopRight);
        SanitizeWorker::rotate_light_indices(&mut indices_br, 2, 5, Origin::BottomRight);

        assert_eq!(indices_bl, expected_tl, "Convertion from BL to TL failed");
        assert_eq!(indices_tr, expected_tl, "Convertion from TR to TL failed");
        assert_eq!(indices_br, expected_tl, "Convertion from BR to TL failed");
    }
}
