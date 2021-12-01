mod print;
mod vars;
mod types;

fn main() {
    println!("Main rs print function");
    print::run();

    vars::run();

    types::run();
}

