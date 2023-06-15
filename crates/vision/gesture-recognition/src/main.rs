use clap::Parser;
use mediapipe_rs::tasks::vision::GestureRecognizerBuilder;

/// MediaPipe-rs demo: gesture recognition task.
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

    /// The maximum number of hands can be detected by the HandLandmarker.
    #[arg(long, default_value_t = 1)]
    num_hands: i32,

    /// The minimum confidence score for the hand detection to be considered successful.
    #[arg(long, default_value_t = 0.5)]
    min_hand_detection_confidence: f32,

    /// The minimum confidence score of hand presence score in the hand landmark detection.
    #[arg(long, default_value_t = 0.5)]
    min_hand_presence_confidence: f32,
}

fn main() {
    let args = Args::parse();

    let img = image::open(args.input_image).unwrap();

    let gesture_recognition_results = GestureRecognizerBuilder::new()
        .max_results(args.max_results) // set max result
        .score_threshold(args.score_threshold) // set score threshold
        .num_hands(args.num_hands) // set recognition number of hands
        .min_hand_detection_confidence(args.min_hand_detection_confidence)
        .min_hand_presence_confidence(args.min_hand_presence_confidence)
        .build_from_file(args.model)
        .unwrap() // create a task instance
        .recognize(&img)
        .unwrap(); // do inference and generate results

    println!("{}", gesture_recognition_results);
}
