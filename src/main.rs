#[macro_use]
extern crate tracing;

use anyhow::Result;
use clap::Parser;
use rayon::prelude::*;
use std::path::{Path, PathBuf};

mod thumbnail;

#[derive(clap::Parser)]
#[clap(version, author)]
struct Args {
    #[clap(help = "Path to generate thumbnails")]
    path: PathBuf,

    #[clap(long, short, help = "Ignore existing thumbnails and overwrite.")]
    force: bool,
}

fn collect(path: &Path) -> Result<Vec<PathBuf>> {
    let entries = std::fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()?;

    // Partition entries into files and directories
    let (dirs, mut files): (Vec<_>, Vec<_>) = entries.into_par_iter().partition(|p| p.is_dir());
    files = files
        .into_par_iter()
        .filter(|p| {
            let ext = p.extension().unwrap_or_default().to_ascii_lowercase();
            ext == "jpg" || ext == "jpeg"
        })
        .collect();
    let more = dirs
        .into_par_iter()
        .filter(|p| p.file_name().unwrap_or_default() != "@eaDir")
        .map(|f| collect(&f))
        .try_reduce(Vec::new, |mut a, b| {
            a.extend(b.into_iter());
            anyhow::Ok(a)
        })?;
    files.extend(more);
    Ok(files)
}

fn recursive_run<F>(path: &Path, f: F) -> Result<()>
where
    F: Fn(PathBuf) -> Result<()> + Send + Sync,
{
    let files = collect(path)?;
    info!("collected {} files to process", files.len());
    files.into_par_iter().try_for_each(f)?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt::init();

    recursive_run(&args.path, |p| thumbnail::process_file(p, args.force))?;

    Ok(())
}
