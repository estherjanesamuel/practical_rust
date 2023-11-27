fn main() {
    println!("Hello, world!");
    println!("Fibonacci 1th = {}", fib(1));
    println!("Fibonacci 2nd = {}", fib(2));
    println!("Fibonacci 3rd = {}", fib(3));
    println!("Fibonacci 4rd = {}", fib(4));
    println!("Fibonacci 5rd = {}", fib(5));
    println!("Fibonacci 6rd = {}", fib(6));
    println!("Fibonacci 7rd = {}", fib(7));
    println!("Fibonacci 12rd = {}", fib(12));

    println!("=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*");

    // main loop
    let nths = [1,2,3,4,5,6,7,8,9,10];
    for nth in nths {
        println!("Fibonacci {} = {}", nth, fib(nth))
    }

    println!("=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*");

    // main loop + checked_add
    let nths = [1,2,3,4,5,6,7,8,9,10,90,91,92,93,94,95,96];
    for nth in nths {
        println!("Fibonacci {} = {}", nth, fib2(nth))
    }

    println!("=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*");

    // main loop + checked_add + result + matc to result to avoid panic
    let nths = [1,2,3,4,5,6,7,8,9,10,90,91,92,93,94,95,96];
    for nth in nths {
        match fib3(nth) {
            Ok(result) => println!("Fibonacci {} = {}", nth, result),
            Err(e) => println!("Error at fibonacci {} = {}", nth, e)
        }
    }

    println!("=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*=*");

    // main loop + checked_add + result 
    let nths = [1,2,3,4,5,6,7,8,9,10,90,91,92,93,94,95,96];
    for nth in nths {
        println!("Fibonacci {} = {}", nth, fib3(nth).expect("fibonacci calculation failed"))
    }
}

// basic
fn fib(n: u8) -> u64 {
    let mut prev: u64 = 0;
    let mut curr: u64 = 1;

    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}

// checked add
fn fib2(n: u8) -> u64 {
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

// add result 
fn fib3(n: u8) -> Result<u64, &'static str> {
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
    match curr == 0 {
        false => Ok(curr),
        true => Err("Calculation overflow")
    }
}
    
