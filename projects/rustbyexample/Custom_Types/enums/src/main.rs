/*
 * The enum keyword allows the creation of type which
 * may be one of a few different variants. 
 * Any variants which as valid as a struct is also valid as an enum 
*/


/*
 * Create an `enum` to classify a web event. note how bothnames and type
 * information together specify the variant:
 * `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
 * each is different and independent.
*/

#![allow(dead_code)]
enum WebEvent {
    // an `enum` may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click {x: i64, y:i64},
}

// a function which takes a `WebEvent` enum as an argument and returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Desturcture `c` from inside the `enum`.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y` .
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y)
        },
    }
}

/*
 * Type Aliases
 * if you use type alias, you can refer to each enum variant via its alias. 
 * this might be useful if the enum's name is too long or too generic,
 * and you want to rename it.
 * 
 */

enum VeryVeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Substract,
}

// Create type alias
type Operations = VeryVeryVerboseEnumOfThingsToDoWithNumbers;

// * The most common place you'll se this is in impl blocks using Self alias
impl VeryVeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self,  x:i32, y:i32) ->i32 {
        match self {
            Self::Add => x + y,
            Self::Substract => x - y,
        }
    }
}
fn main() {
    let pressed = WebEvent::KeyPress('x');
    // to_owned(), creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;


    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // we can refer to each variant via its alias, not its long and inconvenien name.
    let add = Operations::Add.run(10, 20);
    println!("the result of 10 + 20 is {}", add)
}

/* To learn more about enums and type aliases, you can read the stabilization 
report from when this feature was stabilized into Rust. 
https://github.com/rust-lang/rust/pull/61682/#issuecomment-502472847 */