fn main() {
    let nths = [1,2,3,4,5,6,7,8,9,10,90,91,92,93,94,95,96];
    for nth in nths {
        match fib(nth) {
            Ok(result) => println!("Fibonacci {} = {}", nth, result),
            Err(err) => println!("Fibonacci {} = {}", nth, err),
        
        }
    }
}

fn fib(n: u8) -> Result<u64, &'static str>{
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;
    let mut overflow: bool = false;
    for _ in 1..n {
        let result = prev.checked_add(curr);
        match result {
            Some(next) => {
                prev = curr;
                curr = next;
            }
            None => {
                overflow = true;
                break;
            }
        }
    }
    match overflow {
        false => Ok(curr),
        true => Err("Calculation overflow")
    }
}
