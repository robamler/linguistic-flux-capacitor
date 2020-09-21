use ndarray::{Array0, Array3};
use ndarray_npy::NpzReader;

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::BufReader,
    io::BufWriter,
    path::PathBuf,
};

use byteorder::{LittleEndian, ReadBytesExt};
use log::info;
use structopt::StructOpt;

use compressed_dynamic_word_embeddings::{
    embedding_file::{builder::write_compressed_dwe_file, EmbeddingFile, FileHeader, HEADER_SIZE},
    tensors::RankThreeTensor,
};

#[derive(StructOpt)]
#[structopt(about = "TODO")]
enum Opt {
    /// Creates a compressed dynamic embedding file from an uncompressed quantized
    /// tensor.
    Create(CreateOpt),

    /// Prints out the trajectories of the dot product between pairs of words.
    PairwiseTrajectories(PairwiseTrajectoriesOpt),

    /// Prints out the file header of a compressed dynamic word embedding file.
    Inspect(InspectOpts),
}

#[derive(StructOpt)]
struct CreateOpt {
    /// Number of words that comprise a compressed chunk. For now, this must be a
    /// divisor of the vocabulary size. Larger chunk sizes slightly improve
    /// compression rate but slow down tasks that don't need access to all
    /// embedding vectors in a time step.
    #[structopt(long, short = "C", default_value = "100")]
    jump_interval: u32,

    /// Path to output file [defaults to input file with extension replaced by
    /// ".dwe"].
    #[structopt(long, short)]
    output: Option<PathBuf>,

    /// Path to a `.npz` file containing a rank-three tensor `uncompressed_quantized`
    /// with dtype `numpy.int16` and a 32-bit precision float scalar value
    /// `scale_factor` (which is typically < 1). Create with:
    /// `np.savez_compressed('filename.npz', scale_factor=scale_factor,
    /// uncompressed_quantized=uncompressed_quantized)`.
    input: PathBuf,
}

#[derive(StructOpt)]
struct PairwiseTrajectoriesOpt {
    /// Space separated list of zero based word IDs. For each word in the list, the
    /// program calculates the trajectory of its cosine similarity with the
    /// corresponding word (at the same index in the list) provided with --words2.
    #[structopt(long)]
    words1: Vec<u32>,

    /// Space separated list of zero based word IDs. Must have the same length as
    /// --words1.
    #[structopt(long)]
    words2: Vec<u32>,

    /// Path to a compressed dynamic word embeddings file. Separate from the word
    /// lists with " -- " or provide this argument first.
    input: PathBuf,
}

#[derive(StructOpt)]
struct InspectOpts {
    /// Path to a compressed dynamic word embeddings file.
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    stderrlog::new()
        .verbosity(2)
        .timestamp(stderrlog::Timestamp::Second)
        .init()?;

    match opt {
        Opt::Create(create_opt) => create(create_opt),
        Opt::PairwiseTrajectories(pairwise_trajectories_opt) => {
            pairwise_trajectories(pairwise_trajectories_opt)
        }
        Opt::Inspect(inspect_opt) => inspect(inspect_opt),
    }
}

fn create(mut opt: CreateOpt) -> Result<(), Box<dyn Error>> {
    // Fail early if we can't open output file (e.g., if it already exists).
    let output_path = opt.output.take().unwrap_or_else(|| {
        let mut output_path = opt.input.clone();
        output_path.set_extension("dwe");
        output_path
    });
    let output_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output_path)?;

    info!(
        "Loading uncompressed tensor from file at {} ...",
        opt.input.display()
    );

    let mut npz_reader = NpzReader::new(File::open(&opt.input)?)?;

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
        opt.jump_interval,
        scale_factor,
        output_file,
    )
    .map_err(|()| "Error when compressing file")?;

    info!("Done.");
    Ok(())
}

fn pairwise_trajectories(opt: PairwiseTrajectoriesOpt) -> Result<(), Box<dyn Error>> {
    info!(
        "Loading compressed dynamic embeddings from {} ...",
        opt.input.display()
    );
    let file = BufReader::new(File::open(opt.input)?);
    let embedding_file = EmbeddingFile::from_reader(file).map_err(|()| "Error loading file.")?;

    info!("Calculating trajectories ...");

    let trajectories = embedding_file
        .into_random_access_reader()
        .pairwise_trajectories(opt.words1, opt.words2);

    println!("[");
    for trajectory in trajectories.as_view().iter_subviews() {
        println!("    {:?},", trajectory);
    }
    println!("]");

    info!("Done.");

    Ok(())
}

fn inspect(opt: InspectOpts) -> Result<(), Box<dyn Error>> {
    info!(
        "Peeking into compressed dynamic embeddings at {} ...",
        opt.input.display()
    );
    let mut file = File::open(opt.input)?;
    let mut buf = [0u32; HEADER_SIZE as usize];
    file.read_u32_into::<LittleEndian>(&mut buf)?;
    let header = unsafe {
        // SAFETY: `buf` has correct size.
        FileHeader::memory_map_unsafe(&buf)
    };
    println!("{:#?}", header);

    Ok(())
}
