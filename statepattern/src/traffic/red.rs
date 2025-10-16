use super::{State,  pedestrian::Pedestrian};

pub struct Red;

impl State for Red {
    fn next(&self) -> Box<dyn State> {
        println!("Transitioning from Red to Pedestrian mode, Allowing People to Cross Road");
        Box::new(Pedestrian)

    }

    fn name(&self) -> &'static str {
        "Red"
    }
}
