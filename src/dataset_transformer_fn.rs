use rand::distributions::Uniform;
use rand::Rng;
use crate::ClassifiedPicture;
use crate::dataset_parser::Picture;

pub fn bw<const RESOLUTION: usize>(input: ClassifiedPicture<RESOLUTION>) -> ClassifiedPicture<RESOLUTION> {
    let mut output_buffer = input.picture.data;

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            output_buffer[i][j] = if output_buffer[i][j] == 0 { 0 } else { 255 };
        };
    }
    let picture = Picture{ data : output_buffer };
    return ClassifiedPicture{ picture, class : input.class };
}

pub fn linear_noise<const RAND_RANGE: usize, const RESOLUTION: usize>(input: ClassifiedPicture<RESOLUTION>) -> ClassifiedPicture<RESOLUTION> {
    let mut output_buffer = input.picture.data;

    let between = Uniform::from(0..=RAND_RANGE);
    let mut rand = rand::thread_rng();

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            output_buffer[i][j] = if rand.sample(between) == 0 { 255 } else { output_buffer[i][j] };
        };
    }
    let picture = Picture{ data : output_buffer };
    return ClassifiedPicture{ picture, class : input.class };
}