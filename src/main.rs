use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-loadbench", version, about = "minimal HTTP load tester")]
struct Args {
    #[arg(long)]
    url: String,

    #[arg(short = 'n', default_value_t = 100)]
    n: usize,

    #[arg(short = 'c', default_value_t = 10)]
    c: usize,
}

fn main() {
    let args = Args::parse();
    println!("url={} n={} c={}", args.url, args.n, args.c);
}
