fn main() {
    let nths = [1,2,3,4,5,6,7,8,9,10,90,91,92,93,94,95,96];
    for nth in nths {
        println!("Fibonacci {} = {}", nth, fib(nth));
    }
}

fn fib(n: u8) -> u64{
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    for _ in 1..n {
        let result = prev.checked_add(curr);
        match result {
            Some(next) => {
                prev = curr;
                curr = next;
            }
            None => {
                curr = 0;
                break;
            }
        }
    }
    curr
}
