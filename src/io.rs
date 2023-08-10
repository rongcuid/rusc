use anyhow::*;

use flate2::read::MultiGzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub type LineReader = Box<dyn Iterator<Item = Result<String, std::io::Error>>>;
pub type LineWriter = Box<dyn Write>;

pub enum FileFormat {
    Text,
    Gzip,
    Lz4,
    Zstd,
}

impl TryFrom<&Path> for FileFormat {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self> {
        let ext = path.extension().and_then(OsStr::to_str).unwrap_or_default();
        match ext {
            "txt" => Ok(FileFormat::Text),
            "csv" => Ok(FileFormat::Text),
            "json" => Ok(FileFormat::Text),
            "jsonl" => Ok(FileFormat::Text),
            "gz" => Ok(FileFormat::Gzip),
            "lz4" => Ok(FileFormat::Lz4),
            "zstd" => Ok(FileFormat::Zstd),
            "" => Ok(FileFormat::Text),
            ext => Err(anyhow!("unsupported extension `.{ext}`")),
        }
    }
}

pub fn open_lines_input(path: Option<&Path>) -> Result<LineReader> {
    match path {
        // Stdin if no path specified
        None => Ok(Box::new(stdin().lines())),
        // Stdin if `-` is specified
        Some(input) if input.to_str() == Some("-") => Ok(Box::new(stdin().lines())),
        // Otherwise, open file
        Some(input) => match input.try_into().unwrap() {
            FileFormat::Text => Ok(Box::new(BufReader::new(File::open(input)?).lines())),
            FileFormat::Gzip => {
                let reader = MultiGzDecoder::new(File::open(input)?);
                let bufr = BufReader::new(reader);
                Ok(Box::new(bufr.lines()))
            }
            FileFormat::Lz4 => {
                let reader = lz4_flex::frame::FrameDecoder::new(File::open(input)?);
                Ok(Box::new(BufReader::new(reader).lines()))
            }
            FileFormat::Zstd => {
                let reader = zstd::stream::read::Decoder::new(File::open(input)?)?;
                Ok(Box::new(BufReader::new(reader).lines()))
            }
        },
    }
}

pub fn open_lines_output(path: Option<&Path>) -> Result<LineWriter> {
    let writer: Box<dyn Write> = match path {
        None => Box::new(std::io::stdout()),
        Some(output) if output.to_str() == Some("-") => Box::new(BufWriter::new(std::io::stdout())),
        Some(output) => match output.try_into().unwrap() {
            FileFormat::Text => Box::new(BufWriter::new(File::create(output)?)),
            FileFormat::Gzip => Box::new(BufWriter::with_capacity(
                1024 * 1024 * 16,
                GzEncoder::new(File::create(output)?, Compression::default()),
            )),
            FileFormat::Lz4 => Box::new(BufWriter::new(
                lz4_flex::frame::FrameEncoder::new(File::create(output)?).auto_finish(),
            )),
            FileFormat::Zstd => Box::new(BufWriter::with_capacity(
                1024 * 1024 * 16,
                zstd::stream::write::Encoder::new(File::create(output)?, 9)?.auto_finish(),
            )),
        },
    };
    Ok(writer)
}

pub fn create_or_stdout(path: Option<&Path>) -> Result<Box<dyn Write>> {
    match path {
        Some(p) => Ok(Box::new(File::create(p)?)),
        None => Ok(Box::new(stdout())),
    }
}
