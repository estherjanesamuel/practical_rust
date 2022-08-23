use std::fmt;

fn main() {
    // in general, the `{}` will be automatically replaced with any
    // arguments. these will be stringified
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Argument start
    // at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // as can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jump over");
    
    // different formatting can invoked by specified format character a `:`
    println!("Base 10 repr:                  {}", 69420);
    println!("Base 10 (binary) repr:         {:b}", 69420);
    println!("Base 8 (octal) repr:           {:o}", 69420);
    println!("Base 16 (hexadecimal) repr:    {:x}", 69420);
    println!("Base 16 (hexadecimal) repr:    {:X}", 69420);

    // you can right align text with a specified width. this will output
    //""      1". 5 white spaces and a "1"
    println!("{number:>10}", number=1);

    // you can pad numbers with extra zeroes. this will be output "000001".
    println!("{number:0>10}", number=1);

    // you can used named arguments in the format specifier by appeinding a '$'
    println!("{number:0>width$}", number=1, width=10);

    //rust even checks to make sure the correct number of arguments are used
    println!("my name is {0}, {1} {0}", "Bond", "James");

    // only types that implement fmt::Display can be formatted with '{}'. User-defined 
    // types to not implement fmt::Display by default

    #[allow(dead_code)]
    struct Structure(i32);


    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    

    // this will not compile because 'Structure' does not implement fmt::Display
    //fmt::Display
    println!("this struct `{}` won't print....", Structure(3));

    // for rust 1.58 and above, you can directly capture the argument from 
    // surrounding variable. just like the above, this will output
    // "    1". 5 white spaces and a "1".
    let number = 1.0;
    let width:usize = 6;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("{:.3}", pi);

}

