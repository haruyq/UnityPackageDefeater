use clap::Parser;
use std::path::PathBuf;

mod extractor;
mod model;

#[derive(Parser)]
struct Args {
    #[arg(help = ".unitypackageのパス")]
    input: String,

    #[arg(help = "出力先ディレクトリ")]
    output_dir: String,

    #[arg(long, help = ".metaファイルを出力するかどうか")]
    meta: bool,
}

fn main() {
    let args = Args::parse();

    let output_dir = PathBuf::from(args.output_dir);

    if let Err(e) = extractor::extract(&args.input, &output_dir, &args.meta) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}