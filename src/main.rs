use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, grrrrrrs!");

    let args = Cli::parse();

    println!("pattern {}", args.pattern);
    println!("path {}\n", args.path.display());

    let result = std::fs::read_to_string(&args.path);
    let content = match result {
        Ok(content) => content,
        // there is a shortcut for this pattern of "if error, panic with error message", its called unwrap!
        Err(error) => {
            panic!("can't deal with {}.. just exiting here for now", error);
        }
    };
    println!("file content: {}", content);

    println!("results:");
    println!("---------------------");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
