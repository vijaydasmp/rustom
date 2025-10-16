use super::{State,yellow::Yellow};

pub struct Pedestrian;

impl State for Pedestrian {
    fn next(&self) -> Box<dyn State> {
        println!("Transitioning from Pedestrian to Yellow");
        Box::new(Yellow)
    }

    fn name(&self) -> &'static str {
        "Pedestrian"
    }
}
