use crate::ClassifiedPicture;

pub struct PictureVectorTransformer<const RESOLUTION: usize> {
    picture_vector: Vec<ClassifiedPicture<RESOLUTION>>,
    queued_functions: Vec<fn(ClassifiedPicture<RESOLUTION>) -> ClassifiedPicture<RESOLUTION>>
}

impl<const RESOLUTION: usize> PictureVectorTransformer<RESOLUTION> {
    fn create(transforming_vec: Vec<ClassifiedPicture<RESOLUTION>>) -> PictureVectorTransformer<RESOLUTION> {
        PictureVectorTransformer{ picture_vector: transforming_vec, queued_functions: Vec::new() }
    }

    fn add_mutator(&mut self, mutator: fn(ClassifiedPicture<RESOLUTION>) -> ClassifiedPicture<RESOLUTION>) {
        self.queued_functions.push(mutator)
    }
}

pub fn run_transformer<const RESOLUTION: usize>(transformer: PictureVectorTransformer<RESOLUTION>)
    -> Vec<ClassifiedPicture<RESOLUTION>> {
    let iterable_vector = transformer.picture_vector.into_iter();

    iterable_vector.map(|pic: ClassifiedPicture<RESOLUTION>| -> ClassifiedPicture<RESOLUTION> {
        let functions = transformer.queued_functions.clone();

        let mut result_pic = pic;
        for mutator in functions {
            result_pic = (mutator)(result_pic);
        }
        return result_pic;
    }).collect()
}