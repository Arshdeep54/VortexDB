mod db;
mod types;
mod cli;
mod indexing;
mod dbpath;
mod vectoriser;

mod kd_tree;

fn main() {
    cli::run_cli();
}