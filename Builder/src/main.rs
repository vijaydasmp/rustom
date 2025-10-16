
struct Pizza {
    size: String,
    cheese: bool,
    mushrooms: bool,
    bell_peppers: bool,
    paneer: bool,
}

struct PizzaBuilder {
    size: Option<String>,
    cheese: bool,
    mushrooms: bool,
    bell_peppers: bool,
    paneer: bool,
}

impl PizzaBuilder {
    fn new() -> Self {
        PizzaBuilder {
            size: None,
            cheese: false,
            mushrooms: false,
            bell_peppers: false,
            paneer: false,
        }
    }

    fn size(mut self, size: &str) -> Self {
        self.size = Some(size.to_string());
        self
    }

    fn cheese(mut self, cheese: bool) -> Self {
        self.cheese = cheese;
        self
    }

    fn mushrooms(mut self, mushrooms: bool) -> Self {
        self.mushrooms = mushrooms;
        self
    }

    fn bell_peppers(mut self, bell_peppers: bool) -> Self {
        self.bell_peppers = bell_peppers;
        self
    }

    fn paneer(mut self, paneer: bool) -> Self {
        self.paneer = paneer;
        self
    }

    fn build(self) -> Pizza {
        Pizza {
            size: self.size.unwrap_or("Medium".to_string()),
            cheese: self.cheese,
            mushrooms: self.mushrooms,
            bell_peppers: self.bell_peppers,
            paneer: self.paneer,
        }
    }
}

fn main() {
   let pizza = PizzaBuilder::new()
       .size("Large")
       .cheese(true)
       .mushrooms(true)
       .bell_peppers(false)
       .paneer(true)
       .build();

   println!("Pizza size: {}", pizza.size);
   println!("Extra cheese: {}", pizza.cheese);
   println!("Mushrooms: {}", pizza.mushrooms);
   println!("Bell peppers: {}", pizza.bell_peppers);
   println!("Paneer: {}", pizza.paneer);
}
