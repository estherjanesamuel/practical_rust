use std::mem;

fn main() {
    let xs = [1,2,3,4,5];
    let ys = [0; 500];


    println!("analyze slice:{:?}", analyze_slice(&ys));
    println!("first element of the slice: {}", xs[0]);
    println!("second element of the slice: {}", xs[1]);

    // Arrays are stack allocated
    println!("array accupies {} bytes", mem::size_of_val(&xs));
    println!("array accupies {} bytes", mem::size_of_val(&ys));
    
    // array can be automatically borrowed as slices
    println!("borrow the whole of the array as a slice");
    analyze_slice(&xs);
    analyze_slice(&ys);
    println!("borrow a section of the array as a slice");
    analyze_slice(&xs[1..4]);
    analyze_slice(&ys[1..4]);
    // example of empty slice `&[]`
    let empty_array:[u32; 0] = [];
    let one_array:[u32; 3] = [1,2,3];
    let two_array:[u32; 3] = [1,2,3];
    assert_eq!(&two_array, &one_array);
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // same  but more verbose

    for i in 0..xs.len() + 1 {
        // OOPS, one element too far
        match xs.get(i) {
            Some(xval) => println!("{}:{}", i,xval),
            None => println!("Slow down! {} is too far!", i),
            
        }

        // xs.get(i).expect("Slow down! is too far!");

    }
    // out of bound indexing causes compile error
    println!("{}", xs[5])
}

// this function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len())
}
