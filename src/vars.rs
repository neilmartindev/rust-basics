// Holding primitive data or references to data
// Variables are immutable by default
// Rust is a block scoped language

pub fn run() {
    let qualityevent = "errors on control panel";
    let jobnumber = 152;
    let projectmanager = "Dave Shepard"; 

    // By default the variables are not mutable 
    // To make mutable (changeable) then put "let mut"

    println!("The quality event is {}", qualityevent);
    println!("The job number is {}", jobnumber);
    println!("The PM is {0}", projectmanager);

    // Define constant 

    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once

    let (project, job, learning) = ("SWE", "Developer", "Rust, Javascript and C#");
    println!("{}, {}, {}", project, job, learning);
}

