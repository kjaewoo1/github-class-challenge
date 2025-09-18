// And some input routines
// So this is for demo purposes
use std::io;
use std::io::Write;

fn main() {
    let mut user_input = String::new();
    print!("What's your name? ");
    io::stdout().flush().expect("Error flushing");  // flush the output and print error if it fails
    let _ =io::stdin().read_line(&mut user_input);  // read the input and store it in user_input
    println!("Hello, {}!", user_input.trim());
}
