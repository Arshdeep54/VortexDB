mod cli;
mod indexing;
mod types;
mod dbpath;
mod vectoriser;

mod kd_tree;

fn main() {
    cli::run_cli();
}
