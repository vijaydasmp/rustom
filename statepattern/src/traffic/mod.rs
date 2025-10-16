// src/traffic/mod.rs
pub mod red;
pub mod yellow;
pub mod green;
pub mod pedestrian;
pub trait State {
    fn next(&self) -> Box<dyn State>;
    fn name(&self) -> &'static str;
}
