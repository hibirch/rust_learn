fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello.");

    println!("{}", s1);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}
