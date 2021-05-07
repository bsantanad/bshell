use std::io;
use std::io::Write;

fn main() {
    main_loop()
}

fn main_loop() {
    loop {
        print!("% ");
        io::stdout().flush().expect("failed to flush output");
        let line = get_line();
        let args = split_line(&line);
        //status
    }
}

// get line from stdin, and return it
fn get_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    let input = input;
    input
}

// split line with spaces, store it in
// a Vector of slices and return it
//
// :param line: &String
//      string input sent by the user, collected
//      by get_line() function
fn split_line(line: &String) -> Vec<&str> {
    let iterator = line.split(" ");
    let args: Vec<&str> = iterator.collect();
    args
}
