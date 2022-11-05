use std::borrow::Borrow;
use std::cmp::Reverse;
use std::collections::HashMap;
use crate::ClassifiedPicture;
use crate::dataset_parser::{Picture, PictureClass};

pub struct KnnClassifier<const RESOLUTION: usize, const K: usize> {
    pub train_vector: Vec<ClassifiedPicture<RESOLUTION>>,
    pub metrics_function: fn(&Picture<RESOLUTION>, &Picture<RESOLUTION>) -> f32
}


impl<const RESOLUTION: usize, const K: usize> KnnClassifier<RESOLUTION, K> {
    fn classify(&self, input_pic: Picture<RESOLUTION>) -> PictureClass {
        let mut train_pic_to_dist = HashMap::new();

        for train_pic in self.train_vector.as_slice() {
            let dist = (self.metrics_function)(train_pic.picture.borrow(), &input_pic);
            train_pic_to_dist.insert(train_pic, dist);
        }

        let mut sorted: Vec<_> = train_pic_to_dist.iter().collect();
        sorted.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        let _ = sorted.split_off(K);

        let mut neighbours = HashMap::<i32, i32>::new();

        for pair in sorted {
            let option = neighbours.get(pair.0.class.numerical_value.borrow()).unwrap_or(&0);
            let result = option.clone().checked_add(1).unwrap();
            neighbours.insert(pair.0.class.numerical_value, result);
        }

        let mut sorted: Vec<_> = neighbours.iter().collect();
        sorted.sort_by_key(|a| Reverse(*(a.1)));

        return PictureClass{ numerical_value: *sorted.first().unwrap().clone().0 };
    }
}

pub fn get_error_matrix<const RESOLUTION: usize, const K: usize>(
    classifier: KnnClassifier<RESOLUTION, K>,
    test_vector: Vec<ClassifiedPicture<RESOLUTION>>
) -> Vec<(PictureClass, PictureClass, f32)> {
    let mut result: Vec<(PictureClass, PictureClass, f32)> = Vec::new();

    let mut results = [[0.0f32; 10]; 10];
    let mut tests = [0.0f32; 10];

    for test_case in test_vector {
        let result_class = classifier.classify(test_case.picture);
        results[test_case.class.numerical_value as usize][result_class.numerical_value as usize] += 1.0;
        tests[test_case.class.numerical_value as usize] += 1.0;
    }

    for i in 0..10usize {
        for j in 0..10usize {
            let percent = results[i][j] / tests[i];
            result.push((PictureClass{ numerical_value: i as i32 }, PictureClass{ numerical_value: j as i32 }, percent))
        }
    }

    return result;
}