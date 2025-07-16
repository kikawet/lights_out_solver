pub mod chainable;
pub mod handler;
pub mod implementations;
pub mod state;
pub mod worker;

#[macro_export]
macro_rules! define_chainable {
    ($chainable:ident) => {
        #[derive(Default)]
        pub struct $chainable {
            next: Option<Box<dyn Worker>>
        }

        impl Chainable for $chainable {
            fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
                &mut **self.next.insert(next)
            }

            fn next(&mut self) -> Option<&mut (dyn Worker)> {
                self.next.as_deref_mut().map(|r| r as _)
            }
        }
    };
}
