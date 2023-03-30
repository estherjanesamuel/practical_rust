fn main() {
    let nths = [1,2,3,4,5,6,7,8,9,10];
    for nth in nths {
        println!("Fibonacci {} = {}", nth, fib(nth));
    }
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
