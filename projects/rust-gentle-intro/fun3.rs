fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    let n = 10;
    println!("factorial({}): {}", n, factorial(n))
}
