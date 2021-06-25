use std::{fs, thread};
use std::collections::HashMap;
use std::convert::TryInto;
use std::error::Error;
use std::time::Instant;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Configuration {
    url: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    n_requests: u8,
}

fn query(configuration: Configuration) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();

    loop {
        let req = client.get(&configuration.url)
            .headers((&configuration.headers).try_into()?);

        let v: Vec<(&str, &str)> = configuration.query_params
            .iter()
            .map(|(k, v)| (k.as_ref(), v.as_ref()))
            .collect();

        println!("sending request");
        let now = Instant::now();
        let response = req.query(&v).send();
        let time_taken = now.elapsed().as_secs_f64();
        match response {
            Ok(response) => println!("response {} in {} seconds", response.status(), time_taken),
            Err(err) => println!("request failed: {} in {} seconds", err, time_taken),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let config_file = "filibuster.json";

    let configuration: Configuration = serde_json::from_str(&fs::read_to_string(config_file)
        .expect("failed to read config file"))
        .expect("failed to parse json");

    let mut handles = vec![];
    for _ in 0..configuration.n_requests {
        let c = configuration.clone();
        handles.push(thread::spawn(move || {
            match query(c) {
                Ok(_) => {}
                Err(err) => println!("{}", err),
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    Ok(())
}
