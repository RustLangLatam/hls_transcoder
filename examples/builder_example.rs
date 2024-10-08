use anyhow::{Context, Result};
use gst::prelude::*;
use hls_transcoder::PipelineBuilder;
use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Instant,
};

/// Main entry point for setting up the HLS pipeline.
fn main() -> Result<()> {
    // Start the timer to measure the total execution time.
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        return Err(anyhow::anyhow!(
            "Invalid arguments. Example: mp4_to_hls input.mp4 hls_output variant1 1280 720 1000000"
        ));
    }

    let input_file = PathBuf::from(args[1].clone());
    let output_dir = PathBuf::from(args[2].clone());
    let variant_name = args[3].clone();
    let width: i32 = args
        .get(4)
        .ok_or_else(|| anyhow::anyhow!("Missing width argument"))?
        .parse()?;
    let height: i32 = args
        .get(5)
        .ok_or_else(|| anyhow::anyhow!("Missing height argument"))?
        .parse()?;
    let bitrate: u32 = args
        .get(6)
        .ok_or_else(|| anyhow::anyhow!("Missing bitrate argument"))?
        .parse()?;

    // Validate and create necessary directories.
    validate_input_file(&input_file)?;
    create_output_dir(&output_dir, &variant_name)?;

    let pipeline_builder = PipelineBuilder::new(
        input_file.to_str().unwrap().to_string(),
        output_dir.to_str().unwrap().to_string(),
        variant_name,
        width,
        height,
        bitrate,
        true
    );
    let pipeline = pipeline_builder
        .build()
        .context("Failed to build pipeline")?;

    // Start the pipeline and measure the time taken.
    let playing_start_time = Instant::now();
    pipeline
        .set_state(gst::State::Playing)
        .context("Failed to set pipeline to playing state")?;
    println!(
        "Pipeline set to Playing state in {:.2?}",
        playing_start_time.elapsed()
    );

    let bus = pipeline
        .bus()
        .expect("Failed to retrieve bus from pipeline");
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(_) => break,
            gst::MessageView::Error(err) => return Err(anyhow::anyhow!("Error: {}", err.error())),
            _ => (),
        }
    }

    // Measure the time taken to set the pipeline to Null state.
    let stopping_start_time = Instant::now();
    pipeline.set_state(gst::State::Null)?;
    println!("Pipeline set to Null state in {:.2?}", stopping_start_time.elapsed());

    // Print the total execution time.
    println!("Total execution time: {:.2?}", start_time.elapsed());

    Ok(())
}

/// Validates the input file to ensure it exists.
fn validate_input_file(input_file: &Path) -> Result<()> {
    if !input_file.exists() {
        return Err(anyhow::anyhow!(
            "Input file does not exist: {}",
            input_file.display()
        ));
    }
    Ok(())
}

/// Creates the output directory if it doesn't already exist.
fn create_output_dir(output_dir: &Path, variant_name: &str) -> Result<()> {
    fs::create_dir_all(output_dir.join(variant_name))
        .context("Failed to create output directory")?;
    Ok(())
}
