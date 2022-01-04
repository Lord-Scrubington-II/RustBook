# Chapter 1: Using Cargo to Build Rust Projects

Given a directory with a structure similar to this one (produced via `cargo new <project_name>`), the following basic commands can be used to produce build files:  

1. `cargo build` builds the project.
2. `cargo run` builds and runs the resultant executable in a single step.
3. `cargo check` parses the source files for compile errors but does not produce the executable.
   1. This is very useful when developing in a terminal-based text editor! We can quickly check for compile-time errors without the computational overhead needed to produce new executables every time.
4. `cargo build --release` produces a build in `target/release` with compiler optimizations enabled. This will take longer than producing debug builds, and should be used when benchmarking.

When the build is completed, Cargo will store our build files in the `target/debug` directory. In general, the standard pattern for working on any Rust project goes like this:

```bash
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```

Additionally, we can always produce executables directly using `rustc`, but this is generally only suitable for projects with very few source files.