use std::io;

fn to_celsius(farenheit: f64) -> f64 {
    let num = (farenheit - 32.0) / 1.8;
    num
}

fn to_farenheit(celsius: f64) -> f64 {
    let num = (celsius * 1.8) + 32.0;
    num
}

fn collect_num() -> f64 {

    println!("Enter a number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: isize = input
        .trim()
        .parse()
        .expect("This is not a number");
    
    let input = input as f64;
    input
    
}

fn main() {
    println!("{}", to_celsius(10000.0));
    println!("{}", to_farenheit(0.0));

    println!("Welcome to the temperature check engine, written in Rust!!!!");
    println!("NOTE: At any time you wanto to quit, press Ctrl+C");
    println!("Would you like to convert to celsius or farenheit.");
    println!("To celsius: c");
    println!("To farenheit: f");
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");
    let choice = choice.trim();
    if choice == "c" {
        // To Celsius
        let farenheit = collect_num();
        let farenheit =  farenheit as f64;
        let celsius = to_celsius(farenheit);
        println!("{} degress Farenheit is {} degress Celsius", farenheit, celsius);

    } else if choice == "f" {
        // To Farenheit
        let celsius = collect_num();
        let celsius = celsius as f64;
        let farenheit = to_farenheit(celsius);
        println!("{} degress Celsius is {} degress Farenheit", celsius, farenheit);

    } else {
        println!("Expected c or f, got {}", choice);
    }
}

