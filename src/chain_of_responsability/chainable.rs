use super::worker::Worker;

pub trait Chainable {
    fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker;
    fn next(&mut self) -> Option<&mut dyn Worker>;
}

#[macro_export]
macro_rules! define_chainable {
    ($chainable:ident $(, $field:ident : $field_type:ty)*) => {
        #[derive(Default)]
        pub struct $chainable {
            next: Option<Box<dyn Worker>>,
            $($field : $field_type,)* // This will add the specified fields to the struct
        }

        impl Chainable for $chainable {
            fn set_next(&mut self, next: Box<dyn Worker>) -> &mut dyn Worker {
                &mut **self.next.insert(next)
            }

            fn next(&mut self) -> Option<&mut (dyn Worker + '_)> {
                self.next.as_deref_mut().map(|r| r as _)
            }
        }
    };
}
