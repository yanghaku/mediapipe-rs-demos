use clap::Parser;
use mediapipe_rs::preprocess::audio::SymphoniaAudioData;
use mediapipe_rs::tasks::audio::AudioClassifierBuilder;

/// MediaPipe-rs demo: audio classification task using `Symphonia`.
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

fn read_audio(audio_path: String) -> SymphoniaAudioData {
    let file = std::fs::File::open(audio_path).unwrap();
    let probed = symphonia::default::get_probe()
        .format(
            &Default::default(),
            symphonia::core::io::MediaSourceStream::new(Box::new(file), Default::default()),
            &Default::default(),
            &Default::default(),
        )
        .unwrap();
    let codec_params = &probed.format.default_track().unwrap().codec_params;
    let decoder = symphonia::default::get_codecs()
        .make(codec_params, &Default::default())
        .unwrap();
    SymphoniaAudioData::new(probed.format, decoder)
}

fn main() {
    let args = Args::parse();

    // read the audio file and create SymphoniaAudioData
    let audio = read_audio(args.input_audio);

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
