### How to install

1. install the rust tool chain from https://rustup.rs/
2. clone this repository
3. run `cargo build --release` in the project root
4. Setup filibuster.json and run either `cargo run` or `./target/release/filibuster`

#### Sample configuration:

```json
{
  "url": "https://test.url/api/path",
  "headers": {
    "Authorization": "your token here"
  },
  "query_params": {
    "address": "foo-bar",
    "items_per_page": "10000"
  },
  "n_requests": 2
}

```

`n_requests` is the number of threads which will be launched in parallel.
