use byteorder::{
    ByteOrder,
    BigEndian,
};
use std::fs::{
    File,
    OpenOptions,
};
use std::io::{
    self,
    BufReader,
    prelude::*,
};

// A helper macro for loading a given size ofbytes from the buffered reader.
macro_rules! load_bytes {
    ($size:literal; and $reader:expr) => {
        // The whole body goes into a scope so that it is a valid
        // expression when the macro gets expanded.
        {
            // Create a buffer array of the given size. This works because `$size`
            // gets expanded in this code at comiple time as a literal number.
            let mut buf = [0u8; $size];

            $reader.read_exact(&mut buf).unwrap();

            buf
        }
    }
}

struct MNISTImageHeader {
    _magic_number: u32,
    number_of_images: u32,
    n_rows: u32,
    n_cols: u32,
}

impl MNISTImageHeader {
    fn load<T: Read>(reader: &mut T) -> MNISTImageHeader {  
        MNISTImageHeader {
            _magic_number: BigEndian::read_u32(&load_bytes!(4; and reader)),
            number_of_images: BigEndian::read_u32(&load_bytes!(4; and reader)),
            n_rows: BigEndian::read_u32(&load_bytes!(4; and reader)),
            n_cols: BigEndian::read_u32(&load_bytes!(4; and reader)),
        }
    }
}

struct MNISTLabelHeader {
    _magic_number: u32,
    number_of_labels: u32,
}

impl MNISTLabelHeader {
    fn load<T: Read>(reader: &mut T) -> MNISTLabelHeader {
        MNISTLabelHeader {
            _magic_number: BigEndian::read_u32(&load_bytes!(4; and reader)),
            number_of_labels: BigEndian::read_u32(&load_bytes!(4; and reader)),
        }
    }
}

// Read and return a vector of vectors containing the pixel values of each
// image from a file of MNIST images.
pub fn read_mnist_image(path: &str) -> Vec<Vec<u8>> {
    let mut buf_reader = match read_file(path) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(buf_reader) => buf_reader
    };

    let header: MNISTImageHeader = MNISTImageHeader::load(&mut buf_reader);

    let mut images: Vec<Vec<u8>> = 
    vec![
        vec![
            0u8; 
            header.n_cols as usize * header.n_rows as usize + 1
        ]; 
        header.number_of_images as usize
    ];

    for i in 0..header.number_of_images {
        // Add a dummy variable for calculting the bias
        images[i as usize][0] = 1;

        for j in 1..header.n_rows * header.n_cols + 1 {
            let mut buf = [0u8; 1];
            buf_reader.read_exact(&mut buf).unwrap();
            images[i as usize][j as usize] = buf[0];
        }
    }

    images
}

// Read and return a vector of labels from a file of MNIST labels.
pub fn read_mnist_label(path: &str) -> Vec<u8> {
    let mut buf_reader = match read_file(path) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(buf_reader) => buf_reader
    };

    let header: MNISTLabelHeader = MNISTLabelHeader::load(&mut buf_reader);

    let mut labels = vec![0u8; header.number_of_labels as usize];

    for i in 0..header.number_of_labels {
        let mut buf = [0u8; 1];
        buf_reader.read_exact(&mut buf).unwrap();
        labels[i as usize] = buf[0];
    }

    labels
}

// Open a file and return a `BufReader` for reading data 
// from a MNIST labels set.
fn read_file(path: &str) -> io::Result<BufReader<File>> {
    let file = OpenOptions::new().read(true).open(path)?;
    Ok(BufReader::new(file))
}