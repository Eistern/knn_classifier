extern crate core;

use std::time::Instant;
use crate::dataset_parser::{ClassifiedPicture, PictureClass};
use crate::dataset_transformer::{PictureVectorTransformer, run_transformer};
use crate::knn_classifier::{get_error_matrix, KnnClassifier};

mod dataset_parser;
mod dataset_transformer;
mod dataset_transformer_fn;
mod knn_classifier_metrics;
mod knn_classifier;

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

fn print_mx(error_matrix: Vec<(PictureClass, PictureClass, f32)>) {
    let mut result_matrix = [[0.0f32; 10]; 10];
    for error_entry in error_matrix {
        result_matrix[error_entry.0.numerical_value as usize][error_entry.1.numerical_value as usize] = error_entry.2;
    }
    for line in result_matrix {
        for element in line {
            print!("{:.3}\t", element)
        }
        print!("\n")
    }
}

fn normalize<const RESOLUTION: usize>(dataset: Vec<ClassifiedPicture<RESOLUTION>>) -> Vec<ClassifiedPicture<RESOLUTION>> {
    let mut transformer = PictureVectorTransformer::create(dataset);
    transformer.add_mutator(dataset_transformer_fn::bw);
    // transformer.add_mutator(dataset_transformer_fn::linear_noise::<4, RESOLUTION>);
    transformer.add_mutator(dataset_transformer_fn::nonlinear_noise::<128, 3, RESOLUTION>);

    return run_transformer(transformer);
}

fn main() {
    let start = Instant::now();
    let dataset = dataset_parser::parse_pic_dataset::<RESOLUTION>(
        "data/train-labels.idx1-ubyte".to_owned(),
        "data/train-images.idx3-ubyte".to_owned());
    let dataset_test = dataset_parser::parse_pic_dataset::<RESOLUTION>(
        "data/test-labels.idx1-ubyte".to_owned(),
        "data/test-images.idx3-ubyte".to_owned());
    let elapsed = start.elapsed();

    // Parse time
    println!("Parsed: {:?}", elapsed);

    let start = Instant::now();

    let transformed_dataset = normalize(dataset);
    let mut transformed_test = normalize(dataset_test);

    let elapsed = start.elapsed();

    let picture = transformed_dataset.get(0).unwrap();
    print_picture(picture);

    // Transform time
    println!("Transformed: {:?}", elapsed);

    let _ = transformed_test.split_off(100);

    let classifier = KnnClassifier::<RESOLUTION, 10> {
        train_vector: transformed_dataset,
        metrics_function: knn_classifier_metrics::euclidean_squared
    };
    let error_matrix = get_error_matrix(classifier, transformed_test);

    print_mx(error_matrix);
}
