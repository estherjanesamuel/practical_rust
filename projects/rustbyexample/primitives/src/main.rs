fn main() {
    // variables can be type anotated.
    let logical: bool = true;
    let a_float: f64 = 1.0; // regular anotation
    let an_integer = 5i32; // suffix annotation

    // or a default will be used.
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    // a type can also be inferred from context
    let mut inferred_type = 12; // type i64 is inferred from another line 
    println!("inferred_type {}",inferred_type);

    inferred_type = 4294967296i64;

    // a mutable variable's value can be changed.
    let mut mutable = 12; // mutable `i32`
    println!("mutable {}",mutable);

    mutable = 21;
    println!("mutable {}",mutable);


    // error! the type of variable can't be changed.
    // mutable = true;

    // variables can be overwritten with shadowing
    let mutable = true;


    println!("logical {}",logical);
    println!("a_float {}",a_float);
    println!("an_integer {}",an_integer);
    println!("default_float {}",default_float);
    println!("default_integer {}",default_integer);
    println!("inferred_type {}",inferred_type);
    println!("mutable {}",mutable);
}
