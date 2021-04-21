fn main() {
    for bottle in (1..100).rev() {
        if bottle == 1 {
            println!("1 bottle of beer on the wall.");
            println!("Take one down, pass it around, no more bottles of beer on the wall.");
            println!("\n");
        } else if bottle == 2 {
            println!("2 bottles of beer on the wall.");
            println!("Take one down, pass it around, 1 more bottle of beer on the wall.");
            println!("\n");
        } else {
            println!("{} bottles of beer on the wall.", bottle);
            println!("Take one down, pass it around, {} more bottles of beer on the wall.", (bottle - 1));
            println!("\n");
        }
    }

    println!("No more bottles of beer on the wall, no more bottles of beer");
    println!("Go to the store and buy some more, 99 bottles of beer on the wall.");
    println!("\n\n");
    println!("By the end of this ...");
    println!("You all are quite drunk.");
}