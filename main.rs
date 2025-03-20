mod cli;
mod database;
mod indexer;
mod testing;
mod vectorisers;
use std::{thread, time::Duration};

use vectorisers::vectoriser::read_from_named_pipe;
fn main() {
    thread::spawn(move || {
        loop {
            read_from_named_pipe();
            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("Main thread is running");

    cli::run_cli();
}
