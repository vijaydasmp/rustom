use super::{State, green::Green};


pub struct Yellow;

impl State for Yellow {
    fn next(&self) -> Box<dyn State> {
        println!("Transitioning from Yellow to Green");
        Box::new(Green)
    }

    fn name(&self) -> &'static str {
        "Yellow"
    }
}
