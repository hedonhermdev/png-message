use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn read_png_file(file: PathBuf) -> Result<Png, &'static str> {
    let file_buf = fs::read(file);

    if file_buf.is_err() {
        return Err("Cannot open the given file");
    }

    let file_buf = file_buf.unwrap();

    let png = Png::try_from(file_buf.as_ref());

    if png.is_err() {
        return Err("Cannot read the PNG file");
    }

    return png;
}

pub fn encode(
    file: PathBuf,
    message: String,
    chunk_type: String,
    output: Option<PathBuf>,
) -> Result<(), &'static str> {
    let png = read_png_file(file);
    if png.is_err() {
        return Err(png.err().unwrap());
    }
    let mut png = png.unwrap();

    let mut chunk_type_arr: [u8; 4] = [0; 4];
    chunk_type_arr.copy_from_slice(chunk_type.as_ref());

    let chunk_type = ChunkType::try_from(chunk_type_arr);

    if chunk_type.is_err() {
        return Err("Cannot read chunk type. Make sure it follows the PNG file specifications");
    }

    let chunk_type = chunk_type.unwrap();

    let chunk_data = Vec::try_from(message).unwrap();
    let chunk = Chunk::new(chunk_type, chunk_data);

    png.append_chunk(chunk);

    match output {
        Some(output) => {
            let outfile = fs::File::create(output);
            let mut outfile = outfile.unwrap();
            let result = outfile.write_all(png.as_bytes().as_ref());
            if result.is_err() {
                return Err("Cannot write to the output file");
            }
        }
        None => {
            println!("{:?}", png.as_bytes());
        }
    };
    return Ok(());
}

pub fn decode(file: PathBuf, chunk_type: String) -> Result<(), &'static str> {
    let png = read_png_file(file);
    if png.is_err() {
        return Err(png.err().unwrap());
    }

    let png = png.unwrap();

    let chunk = png.chunk_by_type(chunk_type.as_ref());

    match chunk {
        Some(c) => {
            println!("{}", String::from_utf8(c.data().to_vec()).unwrap());
            Ok(())
        }
        None => Err("Chunk not found"),
    }
}

pub fn remove(file: PathBuf, chunk_type: String) -> Result<(), &'static str> {
    let png = read_png_file(file);
    if png.is_err() {
        return Err(png.err().unwrap());
    }
    let mut png = png.unwrap();

    png.remove_chunk(chunk_type.as_ref())
}
