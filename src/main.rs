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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("url={} n={} c={} (sequential, c ignored for now)", args.url, args.n, args.c);

    for i in 0..args.n {
        let res = reqwest::get(&args.url).await;
        match res {
            Ok(resp) => println!("[{i}] {}", resp.status()),
            Err(e) => println!("[{i}] error: {e}"),
        }
    }
}
