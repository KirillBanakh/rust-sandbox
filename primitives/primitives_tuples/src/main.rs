use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )\n", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
   Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    println!("long_tuple: {:?}", long_tuple);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);
    println!("tuple_of_tuples: {:?}", tuple_of_tuples);

    // Too long Tuples (more than 12 elements) cannot be printed. Code below drops error.
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:#?}", too_long_tuple);

    let pair = (1i32, true);
    println!("pair: {:?}", pair);
    println!("Reversed pair: {:?}", reverse(pair));

    println!("One-element tuple: {:?}", (1u8,));
    println!("Default integer: {:?}", (1u8));

    let tuple = (1, "hello", 4.5, true);
    println!("tuple: {:?}", tuple);

    let (a, b, c, d) = tuple;
    println!("Destructured tuple: {:?}, {:?}, {:?}, {:?},", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("matrix:\n{}", matrix);
    println!("matrix:\n{}", transpose(matrix));
}
