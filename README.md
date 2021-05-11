# bshell

a small shell written in rust

## journal

I started learning rust a few weeks ago, you can go to [my blog][1] to see
the process. I wanted a fun project to actually use rust and thought a shell
will do it.

The shell is really simple, parse the input and start the new process as a
child of the shell. To be honest, I wanted to use `fork()` and then `execvp`,
meaning, I wanted to fork the shell process and then replace the child with
the new process by using `execvp` (like one would do in C). Nevertheless, I
found out that in order to use `fork()` from [nix][2] in rust, I had to use
the `unsafe` keyword, and that didn't seem to clean.

Therefore, I decided to use the built-in `std::process` to start the processes,
I guess this does the same `fork()` and `execvp()` under the hood, but don't
doing it by hand feels like cheating. On the other hand the code looks cleaner
this way.

## install & run

you have to have rust & cargo installed, just clone the repository and run
```shell
cargo build
```
and
```shell
cargo run
```

The project is still in progress, so this section will probably change in
the near future

[1]: "https://bsantanad.com"
[2]: "https://crates.io/crates/nix"
