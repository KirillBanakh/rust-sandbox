use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
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
struct Complex {
    real: f64,
    imaginary: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

#[derive(Debug)]
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (counter, _v) in vec.iter().enumerate() {
            if counter != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", counter, _v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let my_struct = Structure(10);
    println!("{}", my_struct);

    let min_max = MinMax(1, 10);
    println!("Display vs Debug");
    println!("Display: {}", min_max);
    println!("Debug: {:?}", min_max);

    let point = Point2D { x: -10.0, y: -1.0 };
    println!("Display vs Debug");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imaginary: 7.2 };
    println!("Display vs Debug");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    let list = List(vec![0, 1, 2, 3, 4]);
    println!("Display vs Debug");
    println!("Display: {}", list);
    println!("Debug: {:?}", list);
}
