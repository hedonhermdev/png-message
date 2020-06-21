mod chunk;
mod chunk_type;
mod cli;
mod commands;
mod png;

use anyhow::Result;

fn main() -> Result<()> {
    return cli::CLI::run();
}
