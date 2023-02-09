fn main() {
    println!("Hello, fizzbuzz!");
    let mut x = 0;
    let n = 1000000;
    while x <= n {
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }

        x += 1
    }
    /*
    (1..1000000).for_each(|x| {
        if x % 3 == 0 && x % 5 == 0 {
            println!("FizzBuzz")
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", x)
        }
    })
    */
}

