fn main() {
    run(1000);
    faker(1000)
}

fn run(count:i64) {
    let min_batch_size:i64 = 50;
    for i in 0..(count / min_batch_size) {
        println!("{}", i)
    }
}

fn faker(count: i64) {
    // that is, we will batch 50 inserts of rows at once
    let min_batch_size: i64 = 50;
    if count < min_batch_size {
        panic!("count cant be less than min batch size");
    }
    let mut counter:i64 = 0;
    for j in 1..(count / min_batch_size + 1)  {
        for i in 1..(min_batch_size + 1) {
            println!("{}.{}", j,i);
            counter = counter + 1;
        }
    } 
    println!("total {}", counter)
}