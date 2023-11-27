fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn by_val(x: i32) -> i32 {
    x + 1
}
fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    let res3 = by_val(i);
    println!(" {}, {}, {}", res1, res2, res3)
}
