mod generator;
use clap::Parser;

#[derive(Parser)]
#[command(
    author = "locusst",
    version = "0.1.0",
    about = "A static site generator written in Rust",
    long_about = None
)]
struct Args {
    #[arg(short = 's', long = "source", default_value = "source")]
    source: String,

    #[arg(short = 'o', long = "output", default_value = "output")]
    output: String,
}
fn main() {
    let start = std::time::Instant::now();
    let args: Args = Args::parse();
    let mut site = generator::Site::new();
    site.generate(&args.source, &args.output);
    println!("Elapsed: {}ms", start.elapsed().as_millis());
}