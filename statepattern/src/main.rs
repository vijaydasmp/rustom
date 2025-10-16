
use std::io::{self, Write};
mod traffic; 
use std::thread::sleep;

use traffic::{State, red::Red};


fn main() {
    println!("State Pattern!");
    let mut state: Box<dyn State> = Box::new(Red);
    for _ in 0..6 {
        println!("Current Light: {}", state.name());
        for _ in 0..10 {
            sleep(std::time::Duration::from_secs(1));
            print!("Tick...");
            io::stdout().flush().unwrap(); 
        }

        state = state.next();
    }

}
