// use clap::Parser;

// /// Search for a pattern in a file and display the lines that contain it.
// #[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");

    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("argssssss {}", args.pattern);
    println!("path {}", args.path.display());
}
