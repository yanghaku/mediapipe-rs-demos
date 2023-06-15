use clap::Parser;
use mediapipe_rs::tasks::vision::FaceDetectorBuilder;

/// MediaPipe-rs demo: face detection task.
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

    /// The maximum number of faces output by the detector, default is -1 (no limit).
    #[arg(long, default_value_t = -1)]
    num_faces: i32,

    /// The minimum confidence score for the face detection to be considered successful.
    #[arg(long, default_value_t = 0.5)]
    min_detection_confidence: f32,

    /// The minimum non-maximum-suppression threshold for face detection to be considered overlapped.
    #[arg(long, default_value_t = 0.5)]
    min_suppression_threshold: f32,
}

fn main() {
    let args = Args::parse();

    // read input image
    let mut input_img = image::open(args.input_image).unwrap();
    let detection_result = FaceDetectorBuilder::new()
        .num_faces(args.num_faces)
        .min_detection_confidence(args.min_detection_confidence)
        .min_suppression_threshold(args.min_suppression_threshold)
        .build_from_file(args.model)
        .unwrap() // create a face detector
        .detect(&input_img)
        .unwrap(); // do inference and generate results

    // show formatted result message
    println!("{}", detection_result);

    if let Some(output_path) = args.output_image {
        // draw detection result to image
        detection_result.draw(&mut input_img);
        // save output image
        input_img.save(output_path.as_str()).unwrap();
        println!("Save image to `{}` success.", output_path);
    }
}
