use kosalt;
use std::io;

fn main() {
    loop {
        println!("\tA. Generate new Key, Offset, and Salt.\n\tB. Encrypt with stored Key, Offset, and Salt.\n\tC. Decrypt with external Key, Offset, and Salt.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "A" => kosalt::gen(),
            "B" => encrypt(),
            "C" => decrypt(),
            _ => println!("Invalid option."),
        }
    }
}
fn encrypt() {
    println!("\tEnter Message:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    kosalt::encrypt(&input);
}
fn decrypt() {
    println!("{}", kosalt::decrypt());
}
