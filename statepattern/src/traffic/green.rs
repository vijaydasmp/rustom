
use super::{State,  red::Red};
pub struct Green;

impl State for Green {
    fn next(&self) -> Box<dyn State> {
        println!("Transitioning from Green to Red");
        Box::new(Red)
    }

    fn name(&self) -> &'static str {
        "Green"
    }
}
