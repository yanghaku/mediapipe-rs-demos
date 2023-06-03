use clap::Parser;
use mediapipe_rs::ffmpeg_next;
use mediapipe_rs::preprocess::audio::FFMpegAudioData;
use mediapipe_rs::tasks::audio::AudioClassifierBuilder;

/// MediaPipe-rs demo: audio classification task using `FFMpeg`.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// Input audio file path.
    #[arg(short, long)]
    input_audio: String,

    /// The maximum number of top-scored classification results.
    #[arg(long, default_value_t = 1)]
    max_results: i32,

    /// The score threshold to override the one provided in the model metadata (if any).
    #[arg(long, default_value_t = -1.0)]
    score_threshold: f32,
}

fn main() {
    let args = Args::parse();

    // read the audio file using ffmpeg and create FFMpegAudioData
    ffmpeg_next::init().unwrap();
    let ffmpeg_input = ffmpeg_next::format::input(&args.input_audio.as_str()).unwrap();
    let audio = FFMpegAudioData::new(ffmpeg_input).unwrap();

    let classification_results = AudioClassifierBuilder::new()
        .model_asset_path(args.model) // set model path
        .max_results(args.max_results) // set max result
        .score_threshold(args.score_threshold) // set score threshold
        .finalize()
        .unwrap() // create a audio classifier
        .classify(audio)
        .unwrap(); // do inference and generate results

    // show formatted result message
    for c in classification_results {
        println!("{}", c);
    }
}
