fn calculate(bottom: i32, top: i32) -> i32 {
    (bottom..=top)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn main() {
    println!("Hello, world!");
}
