use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
pub(crate) struct PictureClass {
    pub numerical_value: i32
}

#[derive(Debug)]
pub(crate) struct Picture<const RESOLUTION: usize> {
    pub data: [[u8; RESOLUTION]; RESOLUTION]
}

#[derive(Debug)]
pub(crate) struct ClassifiedPicture<const RESOLUTION: usize> {
    pub picture: Picture<RESOLUTION>,
    pub class: PictureClass
}

pub(crate) fn parse_pic_dataset<const RESOLUTION: usize>(
    label_path: String,
    picture_path: String
) -> Vec<ClassifiedPicture<RESOLUTION>> {
    let mut parsed = Vec::new();

    let mut parsed_labels = parse_labels(label_path);
    let mut parsed_pictures = parse_pictures::<RESOLUTION>(picture_path);

    assert_eq!(parsed_pictures.len(), parsed_labels.len());

    for _ in 0..parsed_pictures.len() {
        let parsed_picture = parsed_pictures.pop()
            .expect("Parsed picture should be in vector");
        let parsed_label = parsed_labels.pop()
            .expect("Parsed label should be in vector");

        parsed.push(ClassifiedPicture{
            picture: parsed_picture,
            class: parsed_label
        })
    }

    parsed
}

fn read_next_n_as<const N: usize, T>(file: &mut BufReader<File>, interpret: fn([u8; N]) -> T) -> T {
    let mut input_buffer = [0; N];

    let read_bytes = file.read(&mut input_buffer[..]).unwrap();
    assert_eq!(read_bytes, N, "File provided only {} bytes out of {}", read_bytes, N);

    interpret(input_buffer)
}

fn read_next_i32(file: &mut BufReader<File>) -> i32 {
    read_next_n_as::<4, i32>(file, i32::from_be_bytes)
}

fn read_next_u8(file: &mut BufReader<File>) -> u8 {
    read_next_n_as::<1, u8>(file, |buffer| -> u8 { buffer[0] })
}

fn parse_labels(label_path: String) -> Vec<PictureClass> {
    let label_file = File::open(label_path)
        .expect("Label file exist");
    let mut label_reader = BufReader::new(label_file);

    // Sanity check
    let magic_number = read_next_i32(&mut label_reader);
    assert_eq!(magic_number, 0x00000801);

    let label_vec_size = read_next_i32(&mut label_reader);
    let mut parsed_labels = Vec::with_capacity(label_vec_size as usize);

    for _ in 0..label_vec_size {
        let read_label = read_next_u8(&mut label_reader);
        parsed_labels.push(PictureClass{ numerical_value: read_label as i32 })
    }

    parsed_labels
}

fn parse_pictures<const RESOLUTION: usize>(picture_path: String) -> Vec<Picture<RESOLUTION>> {
    let picture_file = File::open(picture_path)
        .expect("Picture file exist");
    let mut picture_reader = BufReader::new(picture_file);

    let magic_number = read_next_i32(&mut picture_reader);
    assert_eq!(magic_number, 0x00000803);

    let picture_vec_size = read_next_i32(&mut picture_reader);
    let mut parsed_pictures = Vec::with_capacity(picture_vec_size as usize);

    let picture_height_size = read_next_i32(&mut picture_reader);
    let picture_width_size = read_next_i32(&mut picture_reader);
    assert_eq!(picture_width_size, picture_height_size);

    for _ in 0..picture_vec_size {
        let picture = read_next_picture::<RESOLUTION>(&mut picture_reader, picture_height_size);
        parsed_pictures.push(picture);
    }

    parsed_pictures
}

fn read_next_picture<const RESOLUTION: usize>(file: &mut BufReader<File>, side_length: i32) -> Picture<RESOLUTION> {
    let mut picture_scaled = [[0; RESOLUTION]; RESOLUTION];

    for i in 0..RESOLUTION {
        let mut row_raw = Vec::with_capacity(side_length as usize);

        for _ in 0..side_length {
            let pixel = read_next_u8(file);
            row_raw.push(pixel);
        }

        let row_scaled = scale_row_to::<RESOLUTION>(row_raw.as_slice());

        picture_scaled[i as usize] = row_scaled;
    }

    Picture { data: picture_scaled }
}

fn scale_row_to<const RESOLUTION: usize>(raw_row: &[u8]) -> [u8; RESOLUTION] {
    if raw_row.len() == RESOLUTION {
        clone_slice_into_const_array(raw_row)
    } else if raw_row.len() > RESOLUTION {
        scale_row_down_to::<RESOLUTION>(raw_row)
    } else {
        upscale_row_to::<RESOLUTION>(raw_row)
    }
}

fn clone_slice_into_const_array<const RESOLUTION: usize>(raw_row: &[u8]) -> [u8; RESOLUTION] {
    let mut raw_buffer: [u8; RESOLUTION] = [0; RESOLUTION];
    raw_buffer.copy_from_slice(raw_row);
    raw_buffer
}

fn upscale_row_to<const RESOLUTION: usize>(raw_row: &[u8]) -> [u8; RESOLUTION] {
    let mut scaled_buffer: [u8; RESOLUTION] = [0; RESOLUTION];

    let scale_factor = RESOLUTION / raw_row.len();

    for i in 0..RESOLUTION {
        let previous_value = raw_row[i / scale_factor];
        let next_value = raw_row[i / scale_factor + 1];

        let step = ((next_value - previous_value) as usize / scale_factor) as u8;
        let step_number = (i % scale_factor) as u8;

        let scaled_value = previous_value + step * step_number;
        scaled_buffer[i] = scaled_value;
    }

    scaled_buffer
}

fn scale_row_down_to<const RESOLUTION: usize>(raw_row: &[u8]) -> [u8; RESOLUTION] {
    let mut scaled_buffer: [u8; RESOLUTION] = [0; RESOLUTION];

    let scale_factor = raw_row.len() / RESOLUTION;

    for i in 0..RESOLUTION {
        scaled_buffer[i] = raw_row[i * scale_factor]
    }

    scaled_buffer
}