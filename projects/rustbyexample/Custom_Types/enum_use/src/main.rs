/*
    The use declaration can be used so manual scoping isn't needed
*/

// An attribut to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}


#[derive(Debug)]
enum Work {
    Civilian,
    Soldier,
}

fn main() {

    // Explicitly 'use' each name so ther are available without manual scoping
    use crate::Status::{Poor, Rich};
    // automatically 'use' each name inside 'Work
    use crate::Work::*;

    /*
        // without use
        let status = Status::Poor;
        let work = Work::Civilian;
    */ 
    // With `use`
    // Equivalent to 'Status::Poor
    let status = Rich;

    // Equivalent to Work::Civilian
    let work = Soldier;

    match status {
        // note the lack of scoping because the explicit 'use' above.
        Rich => println!("The rich have lots money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }

    println!("Status : {:?} , Work : {:?}", status, work);
}
