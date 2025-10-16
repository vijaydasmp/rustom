struct Node{
    value:i32,
    next:Option<Box<Node>>, // A pointer to next node , in rust it can me Some or None 
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    fn add_next(&mut self, value: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(Node::new(value)));

    }
    fn print_list(&self) {
        let mut current = self;
        while let Some(ref next_node) = current.next {
            println!("{}", current.value);
            current = next_node;
        }
        println!("{}", current.value); // Print the last Node
    }
    
}


fn main() {
    let mut head = Node::new(1);
    head.add_next(2);
    head.add_next(3);
    head.add_next(4);
    head.add_next(5);
    
    head.print_list();
}
