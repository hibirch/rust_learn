fn main() {
    println!("{}", update_light("red"));
}

fn update_light(current: &str) -> String {
    const COLORS: [&str; 3] = ["green", "yellow", "red"];
    COLORS[(COLORS.iter().position(|&s| s == current).unwrap() + 1) % 3].to_owned()
    // (COLORS.iter().position(|&s| s == current).unwrap() + 1) % 3
}
