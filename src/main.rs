use clap::Parser;
use std::os::windows::process::CommandExt;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,

    /// Should the program call itself
    #[clap(long)]
    callself: bool,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    if args.callself {
        let args_old: Vec<String> = std::env::args().collect();
        if args_old.len() >= 1 {
            let prog_name = &args_old[0];
            println!("call self \"{}\"", prog_name);

            std::process::Command::new(prog_name)
                .arg("--name=Dwayne \"The Rock\" Johnson")
                .spawn()
                .expect("fail");

            std::process::Command::new(prog_name)
                .raw_arg("  --name=\"Dwayne \"\"The Rock\"\" Johnson\" --count 4   ")
                .spawn()
                .expect("fail");
        }
    }
}


//fn main() {
//    let args: Vec<String> = std::env::args().collect();
//    for arg in args.iter() {
//        println!("{}", arg);
//    }
//}


// Microsoft documentation
// - https://docs.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments?view=msvc-170
//
// Microsoft Blog:
// - https://docs.microsoft.com/en-us/archive/blogs/twistylittlepassagesallalike/everyone-quotes-command-line-arguments-the-wrong-way
//
// Rust standard library
// - https://github.com/rust-lang/rust/issues/29494
// - https://doc.rust-lang.org/std/os/windows/process/trait.CommandExt.html#tymethod.raw_arg
