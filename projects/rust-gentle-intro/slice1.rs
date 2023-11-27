fn main() {
    let ints = [1,2,3,4,5];
    let slc1 = &ints[0..2];
    let slc2 = &ints[1..]; // open range!

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slc1);
    println!("slice2 {:?}", slc2)
}
