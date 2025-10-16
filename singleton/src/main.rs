
mod config;

fn main() 
{
    let c1 = config::INSTANCE.get_or_init(|| config::Config { value: 42 });
    let c2 = config::INSTANCE.get_or_init(|| config::Config { value: 100 });

    println!("c1 value: {}", c1.value); // 42
    println!("c2 value: {}", c2.value); // 42, same instance

    let mutable_instance = config::SingletonMutableConfig::new().get_instance();

    {
    // Lock to mutate safely
    let mut config = mutable_instance.lock().unwrap();
    config.value += 1;
    println!("Updated value inside lock: {}", config.value);
    }


        // Access again
    {
        let cfg = mutable_instance.lock().unwrap();
        println!("Read value again: {}", cfg.value);
    }
}
