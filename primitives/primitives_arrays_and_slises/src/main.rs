use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice is: {}", slice[0]);
    println!("Length of the slice is: {}", slice.len());
}

fn main() {
    let xs: [i32; 5] = [0, 1, 2, 3, 4];
    println!("xs len: {}", xs.len());
    println!("xs size: {}", mem::size_of_val(&xs));
    analyze_slice(&xs);
    analyze_slice(&xs[2..5]);
    println!("xs: {:?}", xs);

    let ys: [i32; 500] = [0; 500];
    println!("ys len: {}", ys.len());
    println!("ys size: {}", mem::size_of_val(&ys));
    analyze_slice(&ys);
    analyze_slice(&ys[250 .. 500]);
    println!("ys: {:?}", ys);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(val) => println!("[{}]: {}", i, val),
            None => println!("Slow down, boy! {} is too far!", i),
        }
    }
    // Index out of bounds!
    // println!("{}", xs[5]);
}
