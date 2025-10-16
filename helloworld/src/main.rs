
use std::io;

fn add(a:i32,b:i32)->i32
{
    a+b
}

fn mult(a:i32,b:i32)->i32
{
    a*b
}

fn square(a:i32)->i32
{
    a*a
}

// taking string parametr

fn greet(name:&str)
{
    println!("Hello {}",name);
}

fn your_name()->String
{
    let name="Vijay";
    name.to_string()
}

fn main() {
    //data types in rust
    let i:i32 = -100;
    let f:f32 = 20.15;
    let age:u32=200;
    let greeting: &str = "Namaste Rust"; 

    
    println!("i = {} f ={} age = {}, greeting = {}",i,f,age,greeting);
    let a=2;
    let b=3;
    let sum = add(a,b);
    let mul = mult(a,b);
    let squ = square(a);
    println!("Sum = {} , Mul = {}, suqare = {}",sum,mul,squ);
    greet("vijay");
    println!("Hello I am Ajaya and you are {}",your_name()); 

    let mut input = String::new(); // mutable buffer for user input
    println!("Enter your name");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let name = input.trim();
    greet(name);

    //reset input
    input.clear();
    
    println!("Enter your age");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let age:usize = input.trim().parse().expect("Please type a number!");

    if age < 18 {
        println!("You are a minor.");
    } else if age < 65 {
        println!("You are an adult.");
    } else {
        println!("You are a senior citizen.");        
    }


    //loops
    //infinite loop
    let mut count = 0;
    loop {
        println!("This is an infinite loop count = {}", count);
        count += 1;
        if count == 5 {
            println!("Breaking the loop at count = {}", count);
            break; // exit the loop
        }
    }

    //while loop
    let mut count = 0;
    while count < 5 {
        println!("This is a while loop count = {}", count);
        count += 1;
    }

    //for loop
    for i in 0..5 {
        println!("This is a for loop count = {}", i);
    }

    //Multiplication Table

    let mut number: i32;
loop  {   
loop{
    println!("Enter a number to generate its multiplication table:");
    input.clear();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //number = input.trim().parse().expect("Please type a number!");
    match input.trim().parse::<usize>() {
        Ok(num) => {
                number = num as i32;
                break; // exit the loop if parsing is successful
            }
        Err(_) => {
            println!("Please enter  number.");
            continue; // go back to the beginning of the loop
        }
    }
}
    for i in 1..=10 {
        println!("{} x {} = {}", number, i, number * i);
    }
   
    println!("Do you want to generate another multiplication table? (yes/no)");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let response = input.trim();
    if response != "yes" && response != "y" {
        break; // exit the outer loop if the user doesn't want to continue
    }
}//outer loop
}
