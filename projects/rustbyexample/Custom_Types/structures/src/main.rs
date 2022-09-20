
// An attribute to hide warnings for unused code.`#![allow(dead_code)]`
#![allow(dead_code)]

fn main() {
    // create struct with field init shorthand
    let name = String::from("Ariefs");
    let age = 27;
    let ariefs = Person {name, age};
    //print debug struct
    println!("{:?}", ariefs);

    // instantiate a Point
    let point = Point{x:10.3, y:0.4};

    // access the field of the point
    println!("point coordinates: ({},{})", point.x, point.y);

    // make a new point by using struct update syntax to use the fields of our other one
    let bottom_right = Point{x:5.2, ..point};

    // bottom right.y will be the same as `point.y` because we used that field from point
    println!("second point: ({},{})", bottom_right.x, bottom_right.y);

    // destructure the point using a let binding
    let Point {x: left_edge, y: top_edge} = point;
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left:Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right
    };

    // instantiate a unit struct
    let _unit = Unit;

    // instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // acces the fields of tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);


}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x:f32,
    y:f32,
}

// Struct can be reused as fields of another struct
struct Rectangle {
    // a rectangle can be specified by where the top left and bottom right corners are in space
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(_rectangle: Rectangle) -> f64{
    // Add a function rect_area which calculates the area of
    //  a Rectangle (try using nested destructuring).
    todo!()
}

fn square(_point:Point, _decimal:f32) {
    // Add a function square which takes a Point and a f32 as arguments, 
    // and returns a Rectangle with its top left corner on the point, 
    // and a width and height corresponding to the f32.
    todo!()
}



