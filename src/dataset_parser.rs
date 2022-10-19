use std::borrow::Borrow;
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
        let picture = read_next_picture::<RESOLUTION>(
            &mut picture_reader,
            picture_height_size as usize
        );
        parsed_pictures.push(picture);
    }

    parsed_pictures
}

fn read_next_picture<const RESOLUTION: usize>(file: &mut BufReader<File>, side_length: usize) -> Picture<RESOLUTION> {
    let mut picture_scaled = Vec::with_capacity(side_length);

    for _ in 0..side_length {
        let mut row_raw = Vec::with_capacity(side_length as usize);

        for _ in 0..side_length {
            let pixel = read_next_u8(file);
            row_raw.push(pixel);
        }

        let filled_raw_row = row_raw.clone();
        picture_scaled.push(filled_raw_row);
    }

    scale_raw_picture::<RESOLUTION>(picture_scaled)
}

fn scale_raw_picture<const RESOLUTION: usize>(raw_data: Vec<Vec<u8>>) -> Picture<{ RESOLUTION }> {
    if raw_data.len() == RESOLUTION {
        Picture { data: copy_into_fixed_array::<RESOLUTION>(raw_data) }
    } else if raw_data.len() > RESOLUTION {
        Picture { data: scale_down_to::<RESOLUTION>(raw_data) }
    } else {
        Picture { data: upscale_to::<RESOLUTION>(raw_data) }
    }
}

fn copy_into_fixed_array<const RESOLUTION: usize>(data: Vec<Vec<u8>>) -> [[u8; RESOLUTION]; RESOLUTION] {
    let mut fixed_buffer = [[0; RESOLUTION]; RESOLUTION];
    for i in 0..RESOLUTION {
        let mut fixed_row_buffer = [0; RESOLUTION];

        let row_clone = data[i].clone();
        fixed_row_buffer.copy_from_slice(row_clone.as_slice());

        fixed_buffer[i] = fixed_row_buffer;
    }
    fixed_buffer
}

fn upscale_to<const RESOLUTION: usize>(data: Vec<Vec<u8>>) -> [[u8; RESOLUTION]; RESOLUTION] {
    let mut scaled_buffer = [[0; RESOLUTION]; RESOLUTION];

    let scale_factor = RESOLUTION / data.len();

    for i in 0..RESOLUTION {
        for j in 0..RESOLUTION {
            let value = data[i / scale_factor][j / scale_factor];
            scaled_buffer[i][j] = value;
        }
    }

    scaled_buffer
}

fn scale_down_to<const RESOLUTION: usize>(data: Vec<Vec<u8>>) -> [[u8; RESOLUTION]; RESOLUTION] {
    let mut scaled_buffer = [[0; RESOLUTION]; RESOLUTION];

    let sliced_vec: Vec<&[u8]> = data.iter().map(Vec::as_slice).collect();
    let sliced_data = sliced_vec.as_slice();

    let scale_factor = sliced_data.len() / RESOLUTION;

    for i in 0..RESOLUTION {
        let y_start = i * scale_factor;
        let y_finish = y_start + scale_factor;
        for j in 0..RESOLUTION {
            let x_start = j * scale_factor;
            let x_finish = x_start + scale_factor;

            let mean = mean(sliced_data, y_start, y_finish, x_start, x_finish);
            scaled_buffer[i][j] = mean;
        }
    }

    scaled_buffer
}

fn mean(chunk: &[&[u8]], y_start: usize, y_finish: usize, x_start: usize, x_finish: usize) -> u8 {
    let mut sum : i32 = 0;
    for row in chunk[y_start..y_finish].borrow() {
        for pixel in (*row)[x_start..x_finish].borrow() {
            sum += *pixel as i32;
        }
    }
    (sum / chunk.len() as i32) as u8
}