fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

fn clamp(x: f64, x1: f64, x2: f64) -> f64 {
    if x < x1 {
        x1
    } else if x > x2 {
        x2
    } else {
        x
    }
}

fn main() {
    println!("abs(-10.0): {}, clamp(1.0, 2.0, 5.0): {}", abs(-10.0), clamp(1.0, 2.0, 5.0))
}
