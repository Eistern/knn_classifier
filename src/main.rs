extern crate core;

use std::time::Instant;
use crate::dataset_parser::{ClassifiedPicture};

mod dataset_parser;

fn print_picture<const RESOLUTION: usize>(pic: &ClassifiedPicture<RESOLUTION>) {
    println!("Label: {}", pic.class.numerical_value);
    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            print!("{}\t", pic.picture.data[i][j])
        }
        print!("\n")
    }
}

fn main() {
    let start = Instant::now();
    let dataset = dataset_parser::parse_pic_dataset::<8>(
        "data/test-labels.idx1-ubyte".to_owned(),
        "data/test-images.idx3-ubyte".to_owned());
    let elapsed = start.elapsed();

    // Debug format
    println!("Elapsed: {:?}", elapsed);

    let picture = dataset.get(0).unwrap();
    print_picture(picture)
}
