use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{prelude::*, BufWriter};
use std::path::PathBuf;

use byteorder::{LittleEndian, WriteBytesExt};
use log::info;
use structopt::StructOpt;

use compressed_dynamic_word_embeddings::{embedding_file::EmbeddingFile, tensors::RankThreeTensor};

#[derive(StructOpt)]
#[structopt(about = r#"TODO"#)]
struct Opt {
    /// Number of time steps.
    #[structopt(long, short = "T")]
    num_timesteps: u32,

    /// Vocabulary size.
    #[structopt(long, short = "V")]
    vocab_size: u32,

    /// Embedding dimension.
    #[structopt(long, short = "K")]
    embedding_dim: u32,

    /// Number of words that comprise a compressed chunk. For now, this must be a
    /// divisor of the vocabulary size. Larger chunk sizes slightly improve
    /// compression rate but slow down tasks that don't need access to all
    /// embedding vectors in a time step.
    #[structopt(long, short = "C", default_value = "100")]
    chunk_size: u32,

    /// Scale factor: the number by which dot products of the integer quantized
    /// word embeddings has to be multiplied to approximate the original dot
    /// product (i.e., one divided by the square of the coefficient that was
    /// multiplied to the embedding vectors before they were rounded to integers).
    #[structopt(long, short = "s")]
    scale_factor: f32,

    /// Path to output file. [defaults to input file with extension replaced by
    /// ".dwe"]
    #[structopt(long, short)]
    output: Option<PathBuf>,

    /// Path to uncompressed input file.
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opt = Opt::from_args();

    stderrlog::new()
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    info!(
        "Loading uncompressed tensor from file at {} ...",
        opt.input.display()
    );
    let mut input_file = File::open(&opt.input).unwrap();

    // Fail early if we cannot open output file (e.g., if it already exists).
    let input_path = &mut opt.input;
    let output_path = opt.output.as_ref().unwrap_or_else(|| {
        input_path.set_extension("dwe");
        input_path
    });
    let output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_path)?;

    let mut input_buf = Vec::new();
    input_file.read_to_end(&mut input_buf).unwrap();
    std::mem::drop(input_file);
    if input_buf.len() != (opt.num_timesteps * opt.vocab_size * opt.embedding_dim) as usize {
        return Err("File size does not match product of parameters -T, -V, and -K.".into());
    }

    let uncompressed = RankThreeTensor::from_flattened(
        u8_slice_to_i8_slice(&input_buf).to_vec(),
        opt.num_timesteps as usize,
        opt.vocab_size as usize,
        opt.embedding_dim as usize,
    );

    info!("Building compressed representation ...");
    let compressed = EmbeddingFile::from_uncompressed_quantized(
        uncompressed.as_view(),
        opt.chunk_size,
        opt.scale_factor,
    )
    .map_err(|()| "Error when compressing file")?;

    info!(
        "Saving compressed representation to {} ...",
        output_path.display()
    );
    let mut output_file = BufWriter::new(output_file);
    for i in compressed.as_slice_u32() {
        output_file.write_u32::<LittleEndian>(*i)?;
    }
    output_file.flush()?;
    std::mem::drop(output_file);

    info!("Done.");
    Ok(())
}

fn u8_slice_to_i8_slice(data: &[u8]) -> &[i8] {
    unsafe {
        let ptr = data.as_ptr();
        std::slice::from_raw_parts_mut(ptr as *mut i8, data.len())
    }
}
