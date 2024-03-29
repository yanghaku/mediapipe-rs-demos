use clap::Parser;
use mediapipe_rs::tasks::vision::ImageClassifierBuilder;

/// MediaPipe-rs demo: image classification task.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// Input image file path.
    #[arg(short, long)]
    input_image: String,

    /// The maximum number of top-scored classification results.
    #[arg(long, default_value_t = 1)]
    max_results: i32,

    /// The score threshold to override the one provided in the model metadata (if any).
    #[arg(long, default_value_t = -1.0)]
    score_threshold: f32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let classification_result = ImageClassifierBuilder::new()
        .max_results(args.max_results) // set max result
        .score_threshold(args.score_threshold) // set score threshold
        .build_from_file(args.model)? // create a image classifier
        .classify(&image::open(args.input_image)?)?; // do inference and generate results

    // show formatted result message
    println!("{}", classification_result);

    Ok(())
}
