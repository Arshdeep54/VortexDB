mod db;
mod types;
mod cli;
mod indexing;
mod dbpath;
mod vectoriser;

fn main() {
    cli::run_cli();
}
