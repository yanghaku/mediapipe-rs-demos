use clap::Parser;
use mediapipe_rs::tasks::vision::HandLandmarkerBuilder;

/// MediaPipe-rs demo: hand landmark detection task.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// Input image file path.
    #[arg(short, long)]
    input_image: String,

    /// Input image file path.
    #[arg(short, long)]
    output_image: Option<String>,

    /// The maximum number of hands can be detected by the HandLandmarker.
    #[arg(long, default_value_t = 1)]
    pub num_hands: i32,

    /// The minimum confidence score for the hand detection to be considered successful.
    #[arg(long, default_value_t = 0.5)]
    pub min_hand_detection_confidence: f32,

    /// The minimum confidence score of hand presence score in the hand landmark detection.
    #[arg(long, default_value_t = 0.5)]
    pub min_hand_presence_confidence: f32,
}

fn main() {
    let args = Args::parse();

    // read input image
    let mut input_img = image::open(args.input_image).unwrap();
    let results = HandLandmarkerBuilder::new()
        .model_asset_path(args.model)
        .num_hands(args.num_hands)
        .min_hand_detection_confidence(args.min_hand_detection_confidence)
        .min_hand_presence_confidence(args.min_hand_presence_confidence)
        .finalize()
        .unwrap() // create a task instance
        .detect(&input_img)
        .unwrap(); // do inference and generate results

    // show formatted result message
    println!("{}", results);

    if let Some(output_path) = args.output_image {
        // draw hand landmark results to image
        for res in results.iter() {
            res.draw(&mut input_img);
        }
        // save output image
        input_img.save(output_path.as_str()).unwrap();
        println!("Save image to `{}` success.", output_path);
    }
}
