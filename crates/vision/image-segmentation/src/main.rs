use std::collections::HashMap;

use clap::Parser;
use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use mediapipe_rs::postprocess::ImageCategoryMask;
use mediapipe_rs::tasks::vision::ImageSegmenterBuilder;
use rusttype::Scale;

/// MediaPipe-rs demo: image segmentation task.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Model file path.
    #[arg(short, long)]
    model: String,

    /// Input image file path.
    #[arg(short, long)]
    input_image: String,

    /// Output image file path.
    #[arg(short, long)]
    output_image: String,
}

fn main() {
    let args = Args::parse();

    // load image
    let img = image::open(args.input_image).unwrap();

    let task = ImageSegmenterBuilder::new()
        .model_asset_path(args.model) // set model path
        .output_confidence_masks(false)
        .output_category_mask(true)
        .finalize()
        .unwrap(); // create a task instance

    let labels = task.labels();
    let segmentation_result = task.segment(&img).unwrap(); // do segment

    draw_category_mask(
        segmentation_result.category_mask.unwrap(),
        args.output_image.as_str(),
        labels,
    );
}

fn draw_category_mask(mask: ImageCategoryMask, output_path: &str, labels: &Vec<String>) {
    let mut output_img = RgbImage::new(mask.width(), mask.height());

    let mut label_map = HashMap::new();
    let mut number_map = HashMap::new();

    for w in 0..mask.width() {
        for h in 0..mask.height() {
            let category = mask.get_pixel(w, h)[0] as usize;
            let label = labels.get(category).unwrap().as_str();
            let pixel = Rgb::from(COLORS_RGB[category].clone());

            label_map.insert(pixel, label);
            if !number_map.contains_key(&pixel) {
                number_map.insert(pixel, 0);
            }
            *number_map.get_mut(&pixel).unwrap() += 1;

            output_img.put_pixel(w, h, pixel);
        }
    }
    let mut sorted_numbers = number_map.into_iter().collect::<Vec<(Rgb<u8>, u32)>>();
    sorted_numbers.sort_by(|a, b| b.1.cmp(&a.1));

    // draw labels
    const TEXT_COLOR: Rgb<u8> = Rgb([255, 255, 255]); // white
    const SHOW_THRESHOLD: f32 = 0.01;
    let field_size: u32 = std::cmp::max(20, (mask.height() as f32 * 0.02) as u32);

    let mut start_y = 0;
    let scale = Scale {
        x: field_size as f32,
        y: field_size as f32,
    };
    let total_pixel = (mask.width() * mask.height()) as f32;
    for (pixel, number) in sorted_numbers {
        let percent = number as f32 / total_pixel;
        if percent < SHOW_THRESHOLD {
            break;
        }

        imageproc::drawing::draw_hollow_rect_mut(
            &mut output_img,
            Rect::at(0, start_y).of_size(field_size + 2, field_size + 2),
            TEXT_COLOR,
        );
        imageproc::drawing::draw_filled_rect_mut(
            &mut output_img,
            Rect::at(1, start_y + 2).of_size(field_size, field_size),
            pixel,
        );

        let label = label_map.get(&pixel).unwrap();
        let text = format!("{:.1}% - {}", percent * 100., label);
        imageproc::drawing::draw_text_mut(
            &mut output_img,
            TEXT_COLOR,
            (field_size + 5) as i32,
            start_y,
            scale,
            &mediapipe_rs::postprocess::utils::default_font(),
            text.as_str(),
        );
        start_y += (field_size + 5) as i32;
    }

    output_img.save(output_path).unwrap();
    println!("Save image to `{}` success.", output_path);
}

const COLORS_RGB: &[[u8; 3]] = &[
    [0, 0, 0],       // black
    [128, 128, 128], // gray
    [0, 0, 255],     // blue
    [0, 255, 255],   // cyan
    [0, 128, 0],     // green
    [255, 0, 0],     // red
    [0, 255, 0],     // lime
    [128, 0, 0],     // maroon
    [255, 215, 0],   // gold
    [128, 0, 128],   // purple
    [255, 20, 147],  // deep pink
    [255, 192, 203], // pink
    [255, 255, 0],   // yellow
    [240, 255, 255], // azure
    [46, 139, 87],   // sea green
    [255, 105, 180], // hot pink
    [173, 216, 230], // lite blue
    [0, 0, 128],     // navy
    [165, 42, 42],   // brown
    [139, 0, 0],     // dark red
    [255, 69, 0],    // orange red
    [255, 165, 0],   // orange
];
