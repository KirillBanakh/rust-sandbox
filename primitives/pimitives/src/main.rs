fn main() {
    let _bool_var: bool = false;
    println!("{}", _bool_var);

    let _float_var: f64 = 1.0;
    println!("{}", _float_var);

    let _int_var: i32 = 1;
    println!("{}", _int_var);

    let _int_var_suffix = 2i32;
    println!("{}", _int_var_suffix);

    let _float_var_default = 1.0;
    println!("{}", _float_var_default);

    let _int_var_default = 1;
    println!("{}", _int_var_default);

    let mut _int_var_inferred = 1;
    println!("{}", _int_var_inferred);

    _int_var_inferred = 4294967296i64;
    println!("{}", _int_var_inferred);

    let mut _var_mutable = 1;
    println!("{}", _var_mutable);

    _var_mutable = 10;
    println!("{}", _var_mutable);

    // _var_mutable = true;
    // println!("{}", _var_mutable);

    let _var_mutable = true;
    println!("{}", _var_mutable);
}
