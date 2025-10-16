//Observer Design Pattern

struct Subject {
    state: String,
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    fn new() -> Self {
        Subject { state: String::new(), observers: Vec::new() }
    }

    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn notify(&self, message: &str) {
        for observer in &self.observers {
            observer.update(message);
        }
    }

    fn change_state(&mut self, new_state: String) {
        self.state = new_state;
        self.notify(&self.state);
    }
}

trait Observer {
    fn update(&self, message: &str);
}

struct ConcreteObserver {
    id: u32,
}

impl Observer for ConcreteObserver {
    fn update(&self, message: &str) {
        println!("Observer {}: {}", self.id, message);
    }
}

fn main() {
    let mut subject = Subject::new();

    let observer1 = Box::new(ConcreteObserver { id: 1 });
    let observer2 = Box::new(ConcreteObserver { id: 2 });

    subject.attach(observer1);
    subject.attach(observer2);

    subject.change_state("Initial State".to_string());
    subject.change_state("New State".to_string());
    subject.change_state("Final State".to_string());
}
