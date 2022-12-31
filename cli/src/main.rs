use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;
use log::info;
use ndarray::{Array, Array0, Array3};
use ndarray_npy::{NpzReader, NpzWriter};

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::BufReader,
    io::BufWriter,
    path::PathBuf,
};

use compressed_dynamic_word_embeddings::{
    embedding_file::{builder::write_compressed_dwe_file, EmbeddingFile, FileHeader, HEADER_SIZE},
    tensors::RankThreeTensor,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Args {
    /// Creates a compressed dynamic embedding file from an uncompressed quantized
    /// tensor.
    Create(CreateArgs),

    /// Decodes a compressed dynamic embedding file into an uncompressed quantized
    /// tensor.
    Decode(DecodeArgs),

    /// Prints out the trajectories of the dot product between pairs of words.
    PairwiseTrajectories(PairwiseTrajectoriesArgs),

    /// Prints out the file header of a compressed dynamic word embedding file.
    Inspect(InspectArgs),
}

#[derive(Parser, Debug)]
struct CreateArgs {
    /// Number of words that comprise a compressed chunk. For now, this must be a
    /// divisor of the vocabulary size. Larger chunk sizes slightly improve
    /// compression rate but slow down tasks that don't need access to all
    /// embedding vectors in a time step.
    #[arg(long, short = 'C', default_value = "100")]
    jump_interval: u32,

    /// Path to output file [defaults to input file with extension replaced by
    /// ".dwe"].
    #[arg(long, short)]
    output: Option<PathBuf>,

    /// Path to a `.npz` file containing a rank-three tensor `uncompressed_quantized`
    /// with dtype `numpy.int16` and a 32-bit precision float scalar value
    /// `scale_factor` (which is typically < 1). Create with:
    /// `np.savez_compressed('filename.npz', scale_factor=scale_factor,
    /// uncompressed_quantized=uncompressed_quantized)`.
    input: PathBuf,
}

#[derive(Parser, Debug)]
struct DecodeArgs {
    /// Path to output file [defaults to input file with extension replaced by
    /// ".npz"].
    #[arg(long, short)]
    output: Option<PathBuf>,

    /// Path to a `.dwe` input file.
    input: PathBuf,
}

#[derive(Parser, Debug)]
struct PairwiseTrajectoriesArgs {
    /// Space separated list of zero based word IDs. For each word in the list, the
    /// program calculates the trajectory of its cosine similarity with the
    /// corresponding word (at the same index in the list) provided with --words2.
    #[arg(long)]
    words1: Vec<u32>,

    /// Space separated list of zero based word IDs. Must have the same length as
    /// --words1.
    #[arg(long)]
    words2: Vec<u32>,

    /// Path to a compressed dynamic word embeddings file. Separate from the word
    /// lists with " -- " or provide this argument first.
    input: PathBuf,
}

#[derive(Parser, Debug)]
struct InspectArgs {
    /// Path to a compressed dynamic word embeddings file.
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    stderrlog::new()
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    match args {
        Args::Create(create_args) => create(create_args),
        Args::Decode(decode_args) => decode(decode_args),
        Args::PairwiseTrajectories(pairwise_trajectories_args) => {
            pairwise_trajectories(pairwise_trajectories_args)
        }
        Args::Inspect(inspect_args) => inspect(inspect_args),
    }
}

fn create(mut args: CreateArgs) -> Result<(), Box<dyn Error>> {
    // Fail early if we can't open output file (e.g., if it already exists).
    let output_path = args.output.take().unwrap_or_else(|| {
        let mut output_path = args.input.clone();
        output_path.set_extension("dwe");
        output_path
    });
    let output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output_path)?;

    info!(
        "Loading uncompressed tensor from file at {} ...",
        args.input.display()
    );

    let mut npz_reader = NpzReader::new(File::open(&args.input)?)?;

    let uncompressed: Array3<i16> = npz_reader.by_name("uncompressed_quantized.npy")?;
    if !uncompressed.is_standard_layout() {
        Err("Tensor `uncompressed_quantized` must be stored in standard layout.")?;
    }
    let (num_timesteps, vocab_size, embedding_dim) = uncompressed.dim();
    info!(
        "Found `uncompressed_quantized` tensor with {} time steps, \
            vocabulary size {}, and embedding dimension {}.",
        num_timesteps, vocab_size, embedding_dim
    );
    let uncompressed = RankThreeTensor::from_flattened(
        uncompressed.into_raw_vec(),
        num_timesteps,
        vocab_size,
        embedding_dim,
    );

    let scale_factor: Array0<f32> = npz_reader.by_name("scale_factor.npy")?;
    let scale_factor = scale_factor.into_scalar();
    info!("scale_factor = {}", scale_factor);

    std::mem::drop(npz_reader);

    info!(
        "Building compressed representation and saving to {}...",
        output_path.display()
    );

    let output_file = BufWriter::new(output_file);
    write_compressed_dwe_file(
        uncompressed.as_view(),
        args.jump_interval,
        scale_factor,
        output_file,
    )
    .map_err(|()| "Error when compressing file")?;

    info!("Done.");
    Ok(())
}

fn decode(mut args: DecodeArgs) -> Result<(), Box<dyn Error>> {
    // Fail early if we can't open output file (e.g., if it already exists).
    let output_path = args.output.take().unwrap_or_else(|| {
        let mut output_path = args.input.clone();
        output_path.set_extension("npz");
        output_path
    });
    let output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output_path)?;
    let mut npz_writer = NpzWriter::new_compressed(BufWriter::new(output_file));

    info!(
        "Opening compressed dynamic embeddings file at {} ...",
        args.input.display()
    );
    let file = BufReader::new(File::open(args.input)?);
    let embedding_file = EmbeddingFile::from_reader(file).map_err(|()| "Error loading file.")?;
    let header = embedding_file.header();
    println!("{:#?}", header);

    let num_timesteps = header.num_timesteps;
    let vocab_size = header.vocab_size;
    let embedding_dim = header.embedding_dim;
    let scale_factor = header.scale_factor;

    let mut uncompressed =
        Vec::with_capacity(num_timesteps as usize * vocab_size as usize * embedding_dim as usize);

    info!("Decoding .dwe file...");

    let reader = embedding_file.into_random_access_reader();
    for t in 0..num_timesteps {
        // This is very inefficient as it both copies data around unnecessarily and decodes many
        // time steps multiple times. But we prioritize correctness (and therefore simplicity) here
        // since this function is not meant to be used frequently.
        uncompressed.extend(reader.get_embeddings_at(t).into_inner());
    }

    let uncompressed = Array::from_shape_vec(
        (
            num_timesteps as usize,
            vocab_size as usize,
            embedding_dim as usize,
        ),
        uncompressed,
    )
    .expect("size and shape must match by construction");

    info!(
        "Writing uncompressed representation and saving to {}...",
        output_path.display()
    );

    npz_writer.add_array("uncompressed_quantized.npy", &uncompressed)?;
    let scale_factor =
        Array::from_shape_vec((), vec![scale_factor]).expect("scalars have shape `()`");
    npz_writer.add_array("scale_factor.npy", &scale_factor)?;

    std::mem::drop(npz_writer);

    info!("Done.");
    Ok(())
}

fn pairwise_trajectories(args: PairwiseTrajectoriesArgs) -> Result<(), Box<dyn Error>> {
    info!(
        "Loading compressed dynamic embeddings from {} ...",
        args.input.display()
    );
    let file = BufReader::new(File::open(args.input)?);
    let embedding_file = EmbeddingFile::from_reader(file).map_err(|()| "Error loading file.")?;

    info!("Calculating trajectories ...");

    let trajectories = embedding_file
        .into_random_access_reader()
        .pairwise_trajectories(args.words1, args.words2);

    println!("[");
    for trajectory in trajectories.as_view().iter_subviews() {
        println!("    {:?},", trajectory);
    }
    println!("]");

    info!("Done.");

    Ok(())
}

fn inspect(args: InspectArgs) -> Result<(), Box<dyn Error>> {
    info!(
        "Peeking into compressed dynamic embeddings at {} ...",
        args.input.display()
    );
    let mut file = File::open(args.input)?;
    let mut buf = [0u32; HEADER_SIZE as usize];
    file.read_u32_into::<LittleEndian>(&mut buf)?;
    let header = unsafe {
        // SAFETY: `buf` has correct size.
        FileHeader::memory_map_unsafe(&buf)
    };
    println!("{:#?}", header);

    Ok(())
}
