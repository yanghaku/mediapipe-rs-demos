use clap::Parser;
use mediapipe_rs::tasks::vision::ImageEmbedderBuilder;

/// MediaPipe-rs demo: image embedding task, calculate the `Cosine Similarity` for two images.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// Input image file path.
    #[arg(long)]
    image_1: String,

    /// Input image file path.
    #[arg(long)]
    image_2: String,
}

fn main() {
    let args = Args::parse();

    let img_1 = image::open(args.image_1).unwrap();
    let img_2 = image::open(args.image_2).unwrap();

    let task = ImageEmbedderBuilder::new()
        .model_asset_path(args.model) // set model path
        .finalize()
        .unwrap(); // create a task instance

    // create a new session to perform task
    let mut session = task.new_session().unwrap();

    // do inference and generate results
    let res_1 = session.embed(&img_1).unwrap();
    let res_2 = session.embed(&img_2).unwrap();

    println!(
        "Cosine Similarity = {}",
        res_1.embeddings[0]
            .cosine_similarity(&res_2.embeddings[0])
            .unwrap()
    );
}
