use clap::Parser;
use mediapipe_rs::tasks::text::TextClassifierBuilder;
use std::io::{stdin, BufRead, Write};

/// MediaPipe-rs demo: text classification task.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// The maximum number of top-scored classification results.
    #[arg(long, default_value_t = 1)]
    max_results: i32,

    /// The score threshold to override the one provided in the model metadata (if any).
    #[arg(long, default_value_t = -1.0)]
    score_threshold: f32,
}

fn main() {
    let args = Args::parse();

    let text_classifier = TextClassifierBuilder::new()
        .model_asset_path(args.model) // set model path
        .max_results(args.max_results) // set max result
        .score_threshold(args.score_threshold) // set score threshold
        .finalize()
        .unwrap(); // create a text classifier

    // create a text classifier session
    let mut session = text_classifier.new_session().unwrap();

    // read text from stdin
    let mut lines = stdin().lock().lines();
    loop {
        print!("Input the text: ");
        std::io::stdout().flush().unwrap();

        if let Some(line) = lines.next() {
            let text = &line.unwrap();
            if !text.is_empty() {
                let result = session.classify(text).unwrap();
                println!("{}", result);
            }
        } else {
            break;
        }
    }
}
