//visitor design pattern

trait Element {
    fn accept(&self, visitor: &dyn Visitor);
    fn get_type(&self) -> &str;

}

trait Visitor {
    fn visit(&self, element: &dyn Element);
}



struct ConcreteElementA;
struct ConcreteElementB;
struct ConcreteVisitor;


impl Element for ConcreteElementA {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit(self);
    }

    fn get_type(&self) -> &str {
        "ConcreteElementA"
    }
}

impl Element for ConcreteElementB {
    fn accept(&self, visitor: &dyn Visitor) {
        visitor.visit(self);
    }
    fn get_type(&self) -> &str {
        "ConcreteElementB"
    }
}


impl Visitor for ConcreteVisitor {
    fn visit(&self, element: &dyn Element) {
        println!("Visiting element of type: {}", element.get_type());
    }
}

fn main() {
    let element_a = ConcreteElementA;
    let element_b = ConcreteElementB;

    // Here you would create a concrete visitor and pass it to the elements
   let visitor = ConcreteVisitor;
   element_a.accept(&visitor);
   element_b.accept(&visitor);
}
