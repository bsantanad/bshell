use std::io;
use std::io::Write;
use std::process;

fn main() {
    main_loop()
}

fn main_loop() {
    loop {
        print!("% ");
        io::stdout().flush().expect("failed to flush output");
        let line = get_line();
        let args = split_line(&line);
        let mut child = launch_process(args);
        child.wait();
        //status
    }
}

// get line from stdin, and return it
fn get_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.pop(); // remove newline
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

fn launch_process(args: Vec<&str>) -> process::Child {
    let err = format!("{} failed to start", arg);
    let err = &err[..];
    process::Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .expect(err)
}
