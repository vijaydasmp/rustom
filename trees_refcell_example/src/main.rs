use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct NAryTreeNode {
    value: i32,
    parent: RefCell<Weak<NAryTreeNode>>,
    children: RefCell<Vec<Rc<NAryTreeNode>>>,    // strong refs to children
}

fn add_child(parent: &Rc<NAryTreeNode>, child: Rc<NAryTreeNode>) {
    parent.children.borrow_mut().push(child);
}

fn create_node(value: i32) -> Rc<NAryTreeNode> {
    Rc::new(NAryTreeNode {
        value,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    })
}

fn print_list(node: &Rc<NAryTreeNode>, level: usize) {
    let indent = "  ".repeat(level);
    println!("{}Node: {}", indent, node.value);
    for child in node.children.borrow().iter() {
        print_list(child, level + 1);
    }
}

fn print_counts(name: &str, node: &Rc<NAryTreeNode>) {
    println!(
        "{} => strong={}, weak={}",
        name,
        Rc::strong_count(node),
        Rc::weak_count(node)
    );
}

fn main() {
// Create a leaf node (initially no parent, no children)

let leaf1 = create_node(1);
let leaf2 = create_node(2);
let leaf3 = create_node(3);
let leaf4 = create_node(4);


let branch = create_node(100);

    println!("--- After creation ---");
    print_counts("branch", &branch);
    print_counts("leaf1", &leaf1);
    print_counts("leaf2", &leaf2);
    print_counts("leaf3", &leaf3);
    print_counts("leaf4", &leaf4);
    println!();
    add_child(&branch, Rc::clone(&leaf1));
    add_child(&branch, Rc::clone(&leaf2));
    add_child(&branch, Rc::clone(&leaf3));
    add_child(&branch, Rc::clone(&leaf4));

*leaf1.parent.borrow_mut() = Rc::downgrade(&branch); // Set parent of leaf1 to branch
*leaf2.parent.borrow_mut() = Rc::downgrade(&branch); // Set parent of leaf2 to branch
*leaf3.parent.borrow_mut() = Rc::downgrade(&branch); // Set parent of leaf3 to branch
*leaf4.parent.borrow_mut() = Rc::downgrade(&branch); // Set parent of leaf4 to branch
   
    println!("\n--- After setting parents ---");
    print_counts("branch", &branch);
    print_counts("leaf1", &leaf1);
    print_counts("leaf2", &leaf2);
    print_counts("leaf3", &leaf3);
    print_counts("leaf4", &leaf4);

    println!("\n--- Printing tree structure ---");
    print_list(&branch, 0);
}