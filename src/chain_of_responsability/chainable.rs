use super::worker::Worker;

pub trait Chainable {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker;
    fn next(&mut self) -> Option<&mut dyn Worker>;
}

// #[derive(Default)]
// pub struct MyChainable {
//     next: Option<Box<dyn Worker>>,
// }
// impl Chainable for MyChainable {
//     fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
//         &mut **self.next.insert(next)
//     }

//     fn next(&mut self) -> Option<&mut dyn Worker> {
//         self.next.as_deref_mut()
//     }
// }
