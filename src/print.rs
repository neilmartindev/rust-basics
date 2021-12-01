pub fn run() {
    // Printing to console
    println!("Hello from the print.rs file");
    println!("Test is a reference test");

    // Printing integers
    println!("{}", 1);
    println!("{} is from {}", "Neil", "England"); // Multiple placeholders

    // Positional arguments and indexing
    println!("{0} is a {2} {1}", "Rust", "safe", "memory");

    // Named arguments
    println!("{name} likes to play to play {activity}", name = "Neil", activity = "Football");

    // Basic maths
    println!("0.08962 * 17000000 = {}", 0.08962 * 17000000.00);
}