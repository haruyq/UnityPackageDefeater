use clap::Parser;
use std::path::PathBuf;
use std::process;

mod extractor;
mod model;
mod dialog;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help = ".unitypackageのパス")]
    input: Option<String>,

    #[arg(help = "出力先ディレクトリ")]
    output_dir: Option<String>,

    #[arg(long, help = ".metaファイルを出力する")]
    meta: bool,
}

fn main() {
    let raw_arg_count = std::env::args_os().count();
    let args = Args::parse();

    let input = if raw_arg_count == 1 {
        match dialog::open_file_dialog() {
            Some(path) => path,
            None => {
                process::exit(1);
            }
        }
    } else {
        match args.input {
            Some(path) => path,
            None => {
                process::exit(2);
            }
        }
    };

    let output_dir = args
        .output_dir
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./export"));

    if let Err(e) = extractor::extract(&input, &output_dir, &args.meta) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}