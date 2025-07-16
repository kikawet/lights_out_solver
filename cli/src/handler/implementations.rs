pub mod print;
pub mod sanitize_input;
pub mod simulator;
pub mod solver;
pub mod validate;

#[cfg(test)]
mod impl_chain_tests {
    use crate::args::Origin;

    use super::sanitize_input::SanitizeHandler;

    #[test]
    fn rotate_3x3() {
        let mut indices_bottom_left = [0, 8];
        let mut indices_top_right = [8, 0];
        let mut indices_bottom_right = [2, 6];
        let expected_top_left = [6, 2];

        SanitizeHandler::rotate_light_indices(&mut indices_bottom_left, 3, 3, Origin::BottomLeft);
        SanitizeHandler::rotate_light_indices(&mut indices_top_right, 3, 3, Origin::TopRight);
        SanitizeHandler::rotate_light_indices(&mut indices_bottom_right, 3, 3, Origin::BottomRight);

        assert_eq!(
            indices_bottom_left, expected_top_left,
            "Conversion from BL to TL failed"
        );
        assert_eq!(
            indices_top_right, expected_top_left,
            "Conversion from TR to TL failed"
        );
        assert_eq!(
            indices_bottom_right, expected_top_left,
            "Conversion from BR to TL failed"
        );
    }

    #[test]
    fn rotate_5x2() {
        let mut indices_bottom_left = [0, 9];
        let mut indices_top_right = [9, 0];
        let mut indices_bottom_right = [1, 8];
        let expected_top_left = [8, 1];

        SanitizeHandler::rotate_light_indices(&mut indices_bottom_left, 2, 5, Origin::BottomLeft);
        SanitizeHandler::rotate_light_indices(&mut indices_top_right, 2, 5, Origin::TopRight);
        SanitizeHandler::rotate_light_indices(&mut indices_bottom_right, 2, 5, Origin::BottomRight);

        assert_eq!(
            indices_bottom_left, expected_top_left,
            "Conversion from BL to TL failed"
        );
        assert_eq!(
            indices_top_right, expected_top_left,
            "Conversion from TR to TL failed"
        );
        assert_eq!(
            indices_bottom_right, expected_top_left,
            "Conversion from BR to TL failed"
        );
    }
}
