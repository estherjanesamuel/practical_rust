fn main() {
    let mut res = 0.0;
    modifies(&mut res);
    println!("res is {} ", res)
}

fn modifies(x: &mut f64) {
    *x = 1.0
}
