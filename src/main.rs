use anyhow::Ok;
use clap::Parser;
use image::io::Reader as ImageReader;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    input: PathBuf,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut output = args.input.clone();
    output.set_extension("png");
    
    let img = ImageReader::open(args.input)?.decode()?;
    img.save(output)?;
    //path to save image

    Ok(())
}
