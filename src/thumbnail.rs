use anyhow::{Context, Result};
use opencv::{core, imgcodecs, imgproc};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy)]
enum Size {
    Small,
    Middle,
    Large,
}
impl Size {
    pub fn size(&self) -> (i32, i32) {
        match self {
            Size::Small => (360, 240),
            Size::Middle => (480, 320),
            Size::Large => (1920, 1280),
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Size::Small => "SYNOPHOTO_THUMB_SM.jpg",
            Size::Middle => "SYNOPHOTO_THUMB_M.jpg",
            Size::Large => "SYNOPHOTO_THUMB_XL.jpg",
        }
    }
}

fn output_dir(img_path: &Path) -> Result<PathBuf> {
    let parent_dir = img_path
        .parent()
        .context("Failed to get parent directory")?;
    let filename = img_path
        .file_name()
        .context("Failed to get file stem")?
        .to_str()
        .context("Failed to convert file stem to string")?;
    let output_dir = parent_dir.join(format!("@eaDir/{}", filename));
    Ok(output_dir)
}

fn read_file(img_path: &Path) -> Result<(bool, core::Mat)> {
    let size = imagesize::size(img_path).context("get image size failed")?;
    let portrait = size.width < size.height;
    const MIN: usize = 1280;
    let flag = match size.height.min(size.width) {
        x if x > MIN * 8 => imgcodecs::IMREAD_REDUCED_COLOR_8,
        x if x > MIN * 4 => imgcodecs::IMREAD_REDUCED_COLOR_4,
        x if x > MIN * 2 => imgcodecs::IMREAD_REDUCED_COLOR_2,
        _ => imgcodecs::IMREAD_COLOR,
    };
    let img = imgcodecs::imread(img_path.to_str().context("img_path to_str failed")?, flag)
        .with_context(|| format!("imread({}) failed", img_path.display()))?;

    Ok((portrait, img))
}

pub fn process_file(img_path: PathBuf, force: bool) -> Result<()> {
    // Determine the output path
    let output_dir = output_dir(&img_path)?;
    std::fs::create_dir_all(&output_dir)?;

    let tasks = [Size::Small, Size::Middle, Size::Large]
        .map(|size| (size, output_dir.join(size.name())))
        .into_iter()
        .filter(|(_, path)| force || !path.exists())
        .collect::<Vec<_>>();
    if tasks.is_empty() {
        debug!("{} thumbnail exists, skipped.", img_path.display());
        return Ok(());
    }

    // get size first
    let (portrait, img) = read_file(&img_path)?;

    for (size, output_path) in tasks {
        let output = output_path.to_str().context("output.to_str() failed")?;
        // Resize the image to fit within the specified dimensions
        let (a, b) = size.size();
        let size = if portrait {
            core::Size::new(b, a)
        } else {
            core::Size::new(a, b)
        };
        let mut resized_img = core::Mat::default();
        imgproc::resize(
            &img,
            &mut resized_img,
            size,
            1.0,
            1.0,
            imgproc::INTER_LINEAR,
        )
        .with_context(|| format!("imgproc::resize({}) failed", img_path.display()))?;

        // Save the resized image
        imgcodecs::imwrite_def(output, &resized_img).with_context(|| {
            format!(
                "imwrite {} => {} failed",
                img_path.display(),
                output_path.display()
            )
        })?;
        debug!("{} => {}", img_path.display(), output_path.display());
    }

    info!("{} generated thumbnails", img_path.display());
    Ok(())
}
