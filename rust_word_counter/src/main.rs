use std::{sync::{Arc, Mutex}, thread};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    threads: usize,

    #[arg(short, long)]
    increments: u32,
}

fn main() {
    let args = Args::parse();

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..args.threads {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap();
                *num += args.increments;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Final counter value: {}", *counter.lock().unwrap());
}
