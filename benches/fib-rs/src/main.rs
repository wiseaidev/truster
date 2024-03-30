use std::fs::File;
use std::io::prelude::*;
use std::hint::black_box;

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

fn main() {
    let mut total: f64 = 0.0;
    let mut file = File::create("fib-rs.csv").unwrap();
    file.write_all(b"Run,Time\n").unwrap();
    for i in 1..=20 {
        let start = std::time::Instant::now();
        black_box(fibonacci(black_box(45)));
        let elapsed = start.elapsed().as_secs_f64();
        println!("Run {} Time taken: {} s", i, elapsed);
        total += elapsed;
        let row = format!("{},{}\n", i, elapsed);
        file.write_all(row.as_bytes()).unwrap();
    }
    let avg_time = total / 20.0;
    println!("Average time taken: {} s", avg_time);
}
