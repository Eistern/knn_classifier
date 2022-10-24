use crate::ClassifiedPicture;
use crate::dataset_parser::Picture;

pub fn bw<const RESOLUTION: usize>(input: ClassifiedPicture<RESOLUTION>) -> ClassifiedPicture<RESOLUTION> {
    let mut output_buffer = input.picture.data;

    for i in 0..output_buffer.len() {
        for j in 0..output_buffer[i].len() {
            output_buffer[i][j] = if output_buffer[i][j] == 0 { 0 } else { 255 };
        };
    }
    let picture = Picture::<RESOLUTION>{ data : output_buffer };
    return ClassifiedPicture::<RESOLUTION>{ picture, class : input.class };
}