use clap::Parser;
use mediapipe_rs::tasks::vision::ObjectDetectorBuilder;

/// MediaPipe-rs demo: object detection task.
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

    /// The maximum number of top-scored classification results.
    #[arg(long, default_value_t = 1)]
    max_results: i32,

    /// The score threshold to override the one provided in the model metadata (if any).
    #[arg(long, default_value_t = -1.0)]
    score_threshold: f32,
}

fn main() {
    let args = Args::parse();

    // read input image
    let mut input_img = image::open(args.input_image).unwrap();
    let detection_result = ObjectDetectorBuilder::new()
        .model_asset_path(args.model)
        .max_results(args.max_results)
        .score_threshold(args.score_threshold)
        .finalize()
        .unwrap() // create a object detector
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
