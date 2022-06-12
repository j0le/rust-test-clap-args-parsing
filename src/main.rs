
fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter() {
        println!("{}", arg);
    }
}

// Microsoft documentation
// - https://docs.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments?view=msvc-170
//
// Microsoft Blog:
// - https://docs.microsoft.com/en-us/archive/blogs/twistylittlepassagesallalike/everyone-quotes-command-line-arguments-the-wrong-way
//
// Rust standard library
// - https://github.com/rust-lang/rust/issues/29494
// - https://doc.rust-lang.org/std/os/windows/process/trait.CommandExt.html#tymethod.raw_arg
