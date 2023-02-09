/*
    Rust provides type safety via static typing, variable bindings can be type annotated when declared.
    However, in most cases, the compiler will be able to infer the type of the variable from the context,
    heavily reducing te annotation burden.
*/

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;

    println!("An Integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // the compiler warns about unused variable bindings; these warnings can be
    // silenced by prefixing the variable na,e an underscore
    let _unused_variable = 3u32;
    let _unnoisy_unused_variable = 2u32;
    // Fixme ^ Prefix with an underscore to suppress the warning
    // please note that warnings may not be shown in a browser 
}
