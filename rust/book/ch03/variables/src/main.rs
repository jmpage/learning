fn main() {
    let mut x = 5;
    print_value(x);
    x = 6;
    print_value(x);
}

fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}
