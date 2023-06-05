use clap::Parser;
use mediapipe_rs::ffmpeg_next;
use mediapipe_rs::preprocess::vision::FFMpegVideoData;
use mediapipe_rs::tasks::vision::ImageClassifierBuilder;

/// MediaPipe-rs demo: image classification task.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// Input video file path.
    #[arg(short, long)]
    input_video: String,

    /// The maximum number of top-scored classification results.
    #[arg(long, default_value_t = 1)]
    max_results: i32,

    /// The score threshold to override the one provided in the model metadata (if any).
    #[arg(long, default_value_t = -1.0)]
    score_threshold: f32,
}

fn main() {
    let args = Args::parse();

    ffmpeg_next::init().unwrap();
    let input =
        FFMpegVideoData::new(ffmpeg_next::format::input(&args.input_video).unwrap()).unwrap();

    let classification_results = ImageClassifierBuilder::new()
        .model_asset_path(args.model) // set model path
        .max_results(args.max_results) // set max result
        .score_threshold(args.score_threshold) // set score threshold
        .finalize()
        .unwrap() // create a image classifier
        .classify_for_video(input)
        .unwrap(); // do inference and generate results

    // show formatted result message
    for r in classification_results {
        println!("{}", r);
    }
}
