use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (first, second) = pair;    
    return (second, first);
}

fn transpose(matrix: Matrix) -> Matrix {
    return Matrix(matrix.0, matrix.2, matrix.1, matrix.3);
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result { 
        writeln!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
     }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is: {:?}", pair);

    println!("reverse pair: {:?}", reverse(pair));

    let tuple = (1, "hello", 4.5, true);

    let (a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix: \n{matrix}", matrix = matrix);
    println!("Transpose: \n{}", transpose(matrix));
}
