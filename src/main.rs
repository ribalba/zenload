use clap::Parser;
use std::thread;
use std::env;
use std::time::{Duration, Instant};
use std::fs::{OpenOptions};
use std::io::{Write};
use rand::Rng;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    verbose: bool,
    #[clap(short, long, default_value = "5")]
    cpu_secs: u64,
}

#[derive(Debug)]
struct Config {
    verbose: bool,
    cpu_secs: u64,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            verbose: args.verbose,
            cpu_secs: args.cpu_secs,
        }
    }
}

fn cpu_test(config: Config, duration: u64) {
    if duration == 0 {
        return
    }
    let target_iterations_per_sec = 1_000_000; // Fixed workload
    let start = Instant::now();

    while start.elapsed().as_secs() < duration {
        println!("CPU Test loop");
        let iteration_start = Instant::now();
        let mut iterations = 0;

        while iterations < target_iterations_per_sec {
            let s: u64 = (1..100).map(|x| x * x).sum();
            iterations += 1;
            let _a = s + 1; // make sure s gets not optimized away
        }
        if config.verbose {
            println!("Iterations done, {:?}", iteration_start.elapsed());
        }

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        } else {
            println!("CPU Test too long, does not fit in one second");
        }
    }
}

fn disk_io_test(file_path: &str, duration: u64) {
    if duration == 0 {
        return
    }
    let start = Instant::now();
    let buffer_size = 1024 * 1024; // 1 MB buffer
    let target_writes_per_sec = 10; // Fixed workload

    let mut file = OpenOptions::new().write(true).create(true).open(file_path).unwrap();

    while start.elapsed().as_secs() < duration {
        let iteration_start = Instant::now();
        for _ in 0..target_writes_per_sec {
            let data = vec![0u8; buffer_size];
            file.write_all(&data).unwrap();
            file.sync_all().unwrap();
        }

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        }
    }

    // Cleanup
    std::fs::remove_file(file_path).unwrap();
}

fn ram_test(duration: u64, size_mb: usize) {
    if duration == 0 {
        return
    }
    let target_allocations_per_sec = 10; // Fixed workload
    let buffer_size = size_mb * 1024 * 1024;
    let start = Instant::now();

    while start.elapsed().as_secs() < duration {
        let iteration_start = Instant::now();
        for _ in 0..target_allocations_per_sec {
            let mut vec = vec![0u8; buffer_size];
            for i in 0..vec.len() {
                vec[i] = rand::thread_rng().gen();
            }
        }

        // Sleep to ensure constant workload
        let elapsed = iteration_start.elapsed();
        if elapsed < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) - elapsed);
        } else {
            println!("RAM Test too long, does not fit in one second");

        }
    }
}

fn gpu_test(duration: u64) {
    // Placeholder for a GPU test using wgpu or other libraries
    println!("GPU test started for {} seconds. (Implement as needed)", duration);
    thread::sleep(Duration::from_secs(duration));
}

fn main() {
    let args = Args::parse();
    let config: Config = args.into();

    let args: Vec<String> = env::args().collect();

    let cpu_duration = config.cpu_secs;
    let disk_duration = 0; // seconds
    let ram_duration = 0; // seconds
    let gpu_duration = 0; // seconds
    let ram_test_size_mb = 0; // MB

    println!("Starting load scenario...");

    println!("Running CPU test for {} seconds", cpu_duration);
    cpu_test(config, cpu_duration);

    println!("Running Disk I/O test...");
    disk_io_test("testfile.tmp", disk_duration);

    println!("Running RAM test...");
    ram_test(ram_duration, ram_test_size_mb);

    println!("Running GPU test...");
    gpu_test(gpu_duration);

    println!("Load scenario complete.");
}
