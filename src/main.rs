mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;

fn main() -> Result<(), &'static str> {
    return cli::CLI::run();
}
