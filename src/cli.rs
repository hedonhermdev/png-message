use crate::commands;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "PngMe", about = "Encode/Decode secret messages in PNG files")]
pub enum CLI {
    #[structopt(name = "encode", about = "Encode a secret message in the given file")]
    Encode {
        #[structopt(name = "FILE")]
        file: PathBuf,

        #[structopt(name = "MESSAGE")]
        message: String,

        #[structopt(name = "CHUNK TYPE")]
        chunk_type: String,

        #[structopt(name = "OUTPUT FILE")]
        output: Option<PathBuf>,
    },
    #[structopt(name = "decode", about = "Decode a secret message from the given file")]
    Decode {
        #[structopt(name = "FILE")]
        file: PathBuf,

        #[structopt(name = "CHUNK TYPE")]
        chunk_type: String,
    },
    #[structopt(name = "remove", about = "Remove a chunk from the given file")]
    Remove {
        #[structopt(name = "FILE")]
        file: PathBuf,

        #[structopt(name = "CHUNK TYPE")]
        chunk_type: String,
    },
}

impl CLI {
    pub fn run() -> Result<(), &'static str> {
        let args: Self = Self::from_args();

        let result = match args {
            CLI::Encode {
                file,
                message,
                chunk_type,
                output,
            } => commands::encode(file, message, chunk_type, output),
            CLI::Decode { file, chunk_type } => commands::decode(file, chunk_type),
            CLI::Remove { file, chunk_type } => commands::remove(file, chunk_type),
        };
        return result;
    }
}
