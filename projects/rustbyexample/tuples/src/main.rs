use std::fmt::{Display, Formatter};

fn main() {
    let long_tuple = (1u8, 2u16, 3u32,4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}\n", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16,2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?} \n", tuple_of_tuples);

    let pair = (1,false);
    println!("pair is: {:?}", pair);
    println!("reverse: {:?}\n", reverse(pair));
    
    let matrix = Matrix(1.1, 1.2, 2.1 , 2.2);
    println!("{:?}\n", matrix);
    println!("{}\n", matrix);
    println!("Transpose:\n{}", transpose(matrix));

    let matrix_two = Matrix(1.1, 1.2, 2.1 , 2.2);

    println!("Transpose2:\n{}", transpose_two(matrix_two));

    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}\n", (5u32));

    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;

    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d)
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;

    (boolean, integer)
}

fn transpose(matrix: Matrix) -> (Matrix) {
    return Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn reverse_two(pair: (f32, f32)) -> (f32, f32) {
    let (a,b) = pair;
    (b,a)
}

fn transpose_two(matrix: Matrix) -> (Matrix) {
    let (a,b) = reverse_two((matrix.1, matrix.2));

    Matrix(matrix.0, a, b, matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f,"( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

