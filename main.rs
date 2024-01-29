mod cli;
mod indexing;
mod kd_tree;
mod keygen;
mod testing;
mod types;
mod dbpath;
mod vectoriser;

fn main() {
    cli::run_cli();
}
