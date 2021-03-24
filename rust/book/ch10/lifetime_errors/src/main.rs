fn main() {
    println!("The string is {}", get_str());
}

fn get_str<'a>() -> &'a String {
    let x: String = String::from("test");
    &x
}
