    /*
        All std library types are automatically printable with {:?} too:
        Derive the `fmt::Debuf implementation for `Structure`.
        `Structure` is a structure which contains a single `i32`
    */
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable also
    #[derive(Debug)]
    struct Deep(Structure);

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
            "Christian",
            actor="actor's");

    // `Structure is printable!
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} is the Structure value!", Structure(3).0);

    // The problem with `derive` is there is no control over how the
    // results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("Now {:?} will print!", Deep(Structure(7)).0.0);

    let name = "arief";
    let age = 32;
    let arief = Person{name: name, age: age};
    let samuel = Person{ name :"ephra", age: 2};

    println!("{:?}", arief);
    println!("{:?}", samuel);
    println!("my name is {:?}", samuel.name);
    println!("and my old is {:?}", samuel.age);

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    
    // Pretty print
    println!("{:#?}", peter);
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// One can manually implement fmt::Display to control the display.
