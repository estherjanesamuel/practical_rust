// Globals are declared outside all oter scopess.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big_int(n: i32) -> bool {
    // access constant in some function
    n > THRESHOLD
}

fn main() {
    let n  = 9;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big_int(n) {"big"} else {"small"});
}
