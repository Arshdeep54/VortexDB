#[macro_use]
mod logger;

mod cli;
mod database;
mod indexer;
mod testing;
mod vectorisers;
// use std::{thread, time::Duration};

use log::info;

fn main() {
    init_module_logger!("main");
    info!("Starting application...");

    // Simulate logger presence in commented out threading
    // thread::spawn(move || {
    //     loop {
    //         read_from_named_pipe();
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });

    println!("Main thread is running");
    info!("Main thread reached CLI");

    cli::run_cli();
}
