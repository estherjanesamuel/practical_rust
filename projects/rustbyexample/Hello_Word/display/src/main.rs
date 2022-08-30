use std::fmt;

fn main() {
    let min_max = min_max(0, 14);
    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);
    
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3,3);

    print!("The big range is {big} and the small is {small} \n",
            small = small_range,
            big= big_range);
    
    let point = Point2D {x: 3.3, y: 7.2};
    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = ComplexPoint(point);
    println!("Debug: {:?}", complex);

    let complex_point = Complex {real: 3.3, imag: 7.2};
    println!("Compare points:");
    println!("Display: {} + {}i", complex_point.real, complex_point.imag);
    println!("Debug: {:?}", complex_point);


    // Error. Both `Display` and `Debug` were implemented, but `{:b}`
    // requires `fmt::Binary to be implemented. this will not work
    // println!("what does Point2D look like in binary: {:b}", point)
}

fn min_max(arg_1: i32, arg_2: i32) -> i32 {
    return arg_1 + arg_2;
}

struct Structure(i32);

impl fmt::Display for Structure {
    // this trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which 
        // is very similar to `println!`
        write!(f, "{}", self.0)
    }
}

// a structure holding two numbers. `Debug` will derived so the results can contrasted with Display
#[derive(Debug)]
struct MinMax(i64, i64);

// implement Displlay for MinMax
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // use `self.number` to refer to each positional data point.
        write!(f, "{}, {}", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct ComplexPoint (Point2D);


#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real {}, imag: {}", self.real, self.imag)
    }
}

