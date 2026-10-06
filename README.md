# rust-loadbench

A minimal HTTP load tester for local APIs.

## Goals

- Load test local HTTP APIs with controlled concurrency
- Answer one question: how many QPS can `rust-shortlink-api` handle on writes
- Stay small: single binary, std + few crates, no distributed mode

## MVP Steps

- Step 1: GET benchmarking + stats (RPS / P50 / P99 / status distribution)
- Step 2: POST with fixed body (`-m POST -H <header> -d <body>`)
- Step 3: Body templates with per-request variables (`{{uuid}}`, `{{seq}}`)
  for writes, e.g. POST /links with a different URL each time

## CLI

```text
rust-loadbench --url <URL> -n <total> -c <concurrency> [options]
```

| Flag | Default | Purpose |
| ---- | ------- | ------- |
| `--url` | (required) | Target URL, e.g. `http://localhost:3000/health` |
| `-n` | `100` | Total requests to send |
| `-c` | `10` | Max in-flight requests (tokio Semaphore) |
| `-m` | `GET` | HTTP method (`GET` in Step 1, `GET,POST` from Step 2) |
| `-H` | (none, repeatable) | Extra header, e.g. `-H 'Content-Type: application/json'` |
| `-d` | (none) | Request body (fixed string in Step 2, template in Step 3) |

Template variables (Step 3 only, inside `-d`):

- `{{uuid}}`: random UUID v4 per request
- `{{seq}}`: incrementing sequence number per request, starting at 0

Example:

```bash
rust-loadbench --url http://localhost:3000/links \
  -n 1000 -c 50 -m POST \
  -H 'Content-Type: application/json' \
  -d '{"url":"https://example.com/{{uuid}}/{{seq}}"}'
```

## Output Format

```text
Total:      1000 in 5.02s
RPS:        199.2
P50:        42ms
P99:        120ms
Status:     200x1000
```

- `Total`: completed requests and wall time
- `RPS`: completed / wall time
- `P50` / `P99`: latency percentiles across all completed requests
- `Status`: distribution by HTTP status code plus errors/timeouts

## Acceptance Criteria

- Step 1: `GET /health -n 100 -c 10` prints RPS/P50/P99/Status, exit 0
- Step 2: `POST /links` with `-d` fixed body returns `201` counted in Status line
- Step 3: `-d` with `{{uuid}}` / `{{seq}}` sends a different body per request
- `cargo fmt --check` clean
- `cargo clippy --all-targets -- -D warnings` zero warnings

## Development

```bash
cargo run -- --url http://localhost:3000/health -n 100 -c 10
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

Out of scope for now: runtime ramp-up, keep-alive tuning, latency graphs,
distributed load, POST file upload, response validation.
