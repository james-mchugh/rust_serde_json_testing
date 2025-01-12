use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;
use std::time::{Duration, Instant};

// #[derive(Deserialize)]
// struct Data {
//     data: Vec<HashMap<String, Value>>,
// }
//
// #[derive(Deserialize)]
// struct Record {
//     #[serde(rename(deserialize = "@timestamp"))]
//     timestamp: Option<String>,
// }

fn main() {
    let contents = String::from_str("{\"foo\": \"bar\"}").unwrap();
    const MAX_ITER: usize = 10_000_000;
    const NUM_LOOPS: usize = 7;

    let mut min_duration = Duration::new(5, 0);
    for _ in 0..NUM_LOOPS {
        let start_time = Instant::now();

        for i in 0..MAX_ITER {
            let result: Value = serde_json::from_str(&contents).unwrap();
            let _ = std::hint::black_box(result);
        }

        let duration = start_time.elapsed();
        if duration < min_duration {
            min_duration = duration;
        }
    }

    println!("Best duration: {:?}", min_duration / MAX_ITER as u32);
}

// fn parse_files() {
//     for arg in env::args().skip(1) {
//         let path = Path::new(&arg);
//         if !path.exists() {
//             println!("{arg} does not exist");
//             continue;
//         }
//
//         let start_time = Instant::now();
//         let data = match parse_json_file(path) {
//             Ok(data) => data,
//             Err(err) => {
//                 println!("Failed to parse JSON file {arg}! {err}");
//                 continue;
//             }
//         };
//         let elapsed = start_time.elapsed();
//         println!("Parsed file in {elapsed:#?}");
//     }
// }
//
// fn parse_json_file(path: &Path) -> Result<Value, Box<dyn Error>> {
//     let mut contents = String::new();
//     File::open(path)?.read_to_string(&mut contents)?;
//     let result = serde_json::from_str(&contents)?;
//
//     Ok(result)
// }
