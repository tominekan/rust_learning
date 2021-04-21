fn main() {
    let hi = String::from("你好");
    let length = operate_on(&hi);
    println!("Value of hi is {}", hi);
    println!("It's length is {}", length);
}

fn operate_on(something: &String) -> usize {
    something.len()
}