mod extractor;
mod model;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input> <output_dir>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_dir = &args[2];

    if let Err(e) = extractor::extract(input_path, output_dir) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
