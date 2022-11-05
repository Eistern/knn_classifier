use crate::dataset_parser::Picture;

pub fn euclidean<const RESOLUTION: usize>(first: &Picture<RESOLUTION>, second: &Picture<RESOLUTION>) -> f32 {
    return euclidean_squared(first, second).sqrt();
}

pub fn euclidean_squared<const RESOLUTION: usize>(first: &Picture<RESOLUTION>, second: &Picture<RESOLUTION>) -> f32 {
    let mut sum : f32 = 0.0;
    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let diff = (first.data[i][j] as f32) - (second.data[i][j] as f32);
            sum += diff.powi(2);
        }
    }
    return sum;
}

pub fn l1<const RESOLUTION: usize>(first: &Picture<RESOLUTION>, second: &Picture<RESOLUTION>) -> f32 {
    let mut sum : f32 = 0.0;
    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let diff = first.data[i][j].abs_diff(second.data[i][j]);
            sum += diff as f32;
        }
    }
    return sum;
}

pub fn chebyshev<const RESOLUTION: usize>(first: &Picture<RESOLUTION>, second: &Picture<RESOLUTION>) -> f32 {
    let mut max : u8 = 0;
    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let diff = first.data[i][j].abs_diff(second.data[i][j]);
            max = max.max(diff);
        }
    }
    return max as f32;
}