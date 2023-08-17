use super::worker::Worker;

pub trait Chainable {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker;
    fn next(&mut self) -> Option<&mut dyn Worker>;
}
