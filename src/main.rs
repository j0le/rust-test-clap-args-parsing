
fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter() {
        println!("{}", arg);
    }
}

// https://docs.microsoft.com/en-us/archive/blogs/twistylittlepassagesallalike/everyone-quotes-command-line-arguments-the-wrong-way
// https://docs.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments?view=msvc-170
