use std::io;
use std::io::Write;

use nix::sys::wait::wait;
use nix::unistd::ForkResult::{Child, Parent};
use nix::unistd::{fork, getpid, getppid};

fn main() {
    main_loop()
}

fn main_loop() {
    loop {
        print!("% ");
        io::stdout().flush().expect("failed to flush output");
        let line = get_line();
        let args = split_line(&line);
        launch_process()
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

fn launch_process() {
    unsafe {
        let pid = fork();

        match pid.expect("Fork Failed: Unable to create child process!") {
            Child => println!(
                "Hello from child process with pid: {} and parent pid:{}",
                getpid(),
                getppid()
            ),
            Parent { child } => {
                wait();
                println!(
                    "Hello from parent process with pid: {} and child pid:{}",
                    getpid(),
                    child
                );
            }
        }
    }
}
