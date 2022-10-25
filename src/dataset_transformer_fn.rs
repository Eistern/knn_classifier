use std::borrow::Borrow;
use std::cmp::{max, min};
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

    let between = Uniform::from(0..RAND_RANGE);
    let mut rand = rand::thread_rng();

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            output_buffer[i][j] = if rand.sample(between) == 0 { 255 } else { output_buffer[i][j] };
        };
    }
    let picture = Picture{ data : output_buffer };
    return ClassifiedPicture{ picture, class : input.class };
}

pub fn nonlinear_noise<const RAND_RANGE: usize, const NOISE_R: usize, const RESOLUTION: usize>(input: ClassifiedPicture<RESOLUTION>) -> ClassifiedPicture<RESOLUTION> {
    let mut output_buffer = input.picture.data;

    let hit_distro = Uniform::from(0..RAND_RANGE);
    let mut rand = rand::thread_rng();

    let mut centers: Vec<(usize, usize)> = Vec::new();

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            if rand.sample(hit_distro) == 0 {
                centers.push((i, j));
            }
        };
    }

    let distance = |(i, j): (usize, usize), centers: &[(usize, usize)]| -> u8 {
        let mut min_range = NOISE_R;

        for center in centers {
            let i_s = i as i32;
            let j_s = j as i32;
            let center_i_s = center.0 as i32;
            let center_j_s = center.1 as i32;
            let distance_from_center =
                ((i_s - center_i_s).pow(2) + (j_s - center_j_s).pow(2)) as f32;

            min_range = min(min_range, distance_from_center.sqrt() as usize);
        }

        return ((NOISE_R - min_range) * 255 / NOISE_R) as u8;
    };

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            output_buffer[i][j] = max(output_buffer[i][j], distance((i, j), centers.borrow()));
        };
    }

    let picture = Picture{ data : output_buffer };
    return ClassifiedPicture{ picture, class : input.class };
}