/*
    Variable bindings have a scope, and are constrained to live in a block, a block is a collection of statement enclosed by braces {}
*/

fn main() {
    // this binding lives in the main function
    let long_lived_binding = 1;

    // this is a block, and has a smaller scope than the main function
    {
        // this binding only  exists in this block
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);
    }
    // End of the block
    
    // Error! short _lived_binding doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);


    /* Also, variable shadowing is allowed */
    let shadowed_binding = 1;

    {
        println!("before being shadowed: {}", shadowed_binding);
        // this binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);

    // tis binding *shadow* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);

    /* Declare First 
    
    it's possible to declate variable bindings first, and initialize them later.
    however, this form is seldom used, as it mau lead to use of uninitialized variables. 
    
    */

    let a_binding;

    {
        let x = 2;

        // initialize the binding
        a_binding = x + x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // error! use of uninitialized binding
    // println!("aanother binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
    // the compiler forbids use uninitialized variables, asthis would lead to undefined behavior


    /* Freezing 
    
    When data is bound by the same name immutably, it also freezes, frozozen data can't be modified until the immutable binding goes out of scope

    */
    let mut _mutable_integer = 7i32;
    {
        // shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;
        // error! `_mutale_integer` is frozen in this scope
        // _mutable_integer = 50;

        // `_mutable_integer` goes out of scope
    }

    // Ok ! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("_mutable integer: {}", _mutable_integer);

    
}
