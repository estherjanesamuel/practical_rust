fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}" , last)
    // first Some(1)
    // last none
    // Welcome to the Option type! It may be either Some or None
}
