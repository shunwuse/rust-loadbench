use clap::Parser;
use std::sync::Arc;
use tokio::sync::Semaphore;

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
    println!(
        "url={} n={} c={} (concurrent, no stats yet)",
        args.url, args.n, args.c
    );

    let sem = Arc::new(Semaphore::new(args.c));
    let mut tasks = Vec::with_capacity(args.n);

    for i in 0..args.n {
        let url = args.url.clone();
        let permit_slot = Arc::clone(&sem);
        tasks.push(tokio::spawn(async move {
            let _permit = permit_slot.acquire_owned().await.unwrap();
            match reqwest::get(&url).await {
                Ok(resp) => println!("[{i}] {}", resp.status()),
                Err(e) => println!("[{i}] error: {e}"),
            }
        }));
    }

    for t in tasks {
        t.await.unwrap();
    }
}
