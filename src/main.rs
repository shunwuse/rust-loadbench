use clap::Parser;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
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

    let sem = Arc::new(Semaphore::new(args.c));
    let mut tasks = Vec::with_capacity(args.n);
    let start = Instant::now();

    for _ in 0..args.n {
        let url = args.url.clone();
        let slot = Arc::clone(&sem);
        tasks.push(tokio::spawn(async move {
            let _permit = slot.acquire_owned().await.unwrap();
            let t = Instant::now();
            let status = match reqwest::get(&url).await {
                Ok(resp) => resp.status().as_u16().to_string(),
                Err(_) => "error".to_string(),
            };
            (t.elapsed(), status)
        }));
    }

    let mut latencies: Vec<Duration> = Vec::with_capacity(args.n);
    let mut counts: HashMap<String, usize> = HashMap::new();
    for t in tasks {
        let (d, s) = t.await.unwrap();
        latencies.push(d);
        *counts.entry(s).or_insert(0) += 1;
    }

    let total = start.elapsed();
    latencies.sort();
    let p50 = latencies[args.n * 50 / 100];
    let p99 = latencies[args.n * 99 / 100];
    let rps = args.n as f64 / total.as_secs_f64();

    let mut status: Vec<String> = counts
        .into_iter()
        .map(|(k, v)| format!("{k}x{v}"))
        .collect();
    status.sort();

    println!("Total:      {} in {:.2?}", args.n, total);
    println!("RPS:        {rps:.1}");
    println!("P50:        {:.0?}", p50);
    println!("P99:        {:.0?}", p99);
    println!("Status:     {}", status.join(" "));
}
