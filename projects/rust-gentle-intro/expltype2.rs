fn main() {
    let x = 2.0 * std::f64::consts::PI;
    
    let abs_difference = (x.cos() - 1.0).abs();
    assert!(abs_difference < 1e-10);
    println!("abs_difference {}", abs_difference)
}
