fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}" , last);
    // first Some(1)
    // last none
    // Welcome to the Option type! It may be either Some or None

    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    // if you were to unwrap last, you would get a panic. But at least you can call is_some first to make sure
    let maybe_last = slice.get(5);
    println!("{:?}", maybe_last);
    let last_index = if maybe_last.is_some() {
        *maybe_last.unwrap()
    } else {
        -1
    };

    println!("{}", last_index)
}
