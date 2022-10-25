extern crate core;

use std::time::Instant;
use crate::dataset_parser::{ClassifiedPicture};
use crate::dataset_transformer::{PictureVectorTransformer, run_transformer};

mod dataset_parser;
mod dataset_transformer;
mod dataset_transformer_fn;

const RESOLUTION : usize = 28;

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
    let dataset = dataset_parser::parse_pic_dataset::<RESOLUTION>(
        "data/train-labels.idx1-ubyte".to_owned(),
        "data/train-images.idx3-ubyte".to_owned());
    let elapsed = start.elapsed();

    // Parse time
    println!("Elapsed: {:?}", elapsed);

    let mut transformer = PictureVectorTransformer::create(dataset);
    transformer.add_mutator(dataset_transformer_fn::bw);
    // transformer.add_mutator(dataset_transformer_fn::linear_noise::<4, RESOLUTION>);
    transformer.add_mutator(dataset_transformer_fn::nonlinear_noise::<128, 3, RESOLUTION>);

    let transformed_dataset = run_transformer(transformer);

    let picture = transformed_dataset.get(0).unwrap();
    print_picture(picture);
}
