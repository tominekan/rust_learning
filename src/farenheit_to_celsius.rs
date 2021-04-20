// src/farenheit_to_celsius.rs
use std::io;

// This is a function that converts a given 
// degree farenheit to celsius
fn to_celsius(farenheit: f64) -> f64 {
    let num = (farenheit - 32.0) / 1.8;
    num
}

// This is a function that converts a given 
// degree celsius to farenheit.
fn to_farenheit(celsius: f64) -> f64 {
    let num = (celsius * 1.8) + 32.0;
    num
}

// This is a function that prompts the user 
// to enter a number and returns it as a floating
// point integer
fn collect_num() -> f64 {

    println!("Enter a number:"); // Prompting the user
    let mut input = String::new();

    io::stdin() // Collect the input
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: isize = input // Format the input to an integer
        .trim()
        .parse()
        .expect("This is not a number");
    
    let input = input as f64; // Change that integer to a floating point
    input
    
}

fn main() {
    // Intro
    println!("Welcome to the temperature check engine, written in Rust!!!!");
    println!("NOTE: At any time you wanto to quit, press Ctrl+C");

    // Prompt the user for what they want to convert
    println!("Would you like to convert to celsius or farenheit.");
    println!("To celsius: c");
    println!("To farenheit: f");

    let mut choice = String::new(); // I also learned String::from() :)

    io::stdin() // Read that input
        .read_line(&mut choice)
        .expect("Failed to read input");
    
    let choice = choice.trim(); // Srtip the newline from it

    if choice == "c" { 
        // The user wants to convert farenheit to celsius
        let farenheit = collect_num();
        let celsius = to_celsius(farenheit);
        println!("{} degress Farenheit is {} degress Celsius", farenheit, celsius);

    } else if choice == "f" {
        // The user wants to convert celsius to farenheit
        let celsius = collect_num();
        let farenheit = to_farenheit(celsius);
        println!("{} degress Celsius is {} degress Farenheit", celsius, farenheit);

    } else {
        // If they did follow the instructions, then tell them so and quit.
        println!("Expected c or f, got {}", choice);
    }
}

