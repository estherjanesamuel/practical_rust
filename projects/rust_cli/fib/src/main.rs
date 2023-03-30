fn main() {
    println!("Fibonacci 1st = {}", fib(1));
    println!("Fibonacci 2nd = {}", fib(2));
    println!("Fibonacci 3rd = {}", fib(3));
    println!("Fibonacci 4th = {}", fib(4));
    println!("Fibonacci 5th = {}", fib(5));
    println!("Fibonacci 6th = {}", fib(6));
}

fn fib(n: u8) -> u64{
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
