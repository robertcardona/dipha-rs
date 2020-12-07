use std::fs;
use std::fs::File;
use std::io::Read;

extern crate binary_reader;
use binary_reader::{Endian, BinaryReader};

use std::fmt;
use std::fmt::Write as FmtWrite;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

extern crate image;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

static DIPHA_MAGIC_NUMBER : i64 = 8067171840;
static DIPHA_MAGIC_NUMBER_POSITION : usize = 0;
static DIPHA_TYPE_POSITION : usize = 8;
static DIPHA_DATA_POSITION : usize = 16;

#[derive(Debug, FromPrimitive)]
enum FileType {
    Unknown = -1,
    WeightedBoundaryMatrix = 0,
    ImageData = 1,
    PersistenceDiagram = 2,
    DistanceMatrix = 7,
    SparseDistanceMatrix = 8
}

// TODO : For each of the structs, implement to_vec() -> Vec<u8> which we then
//  use to write to binary file;

struct Image {
    n : i64,
    d : i64,
    g : Vec<i64>,
    v : Vec<f64>
}

// TODO make an iterator for the image;

impl Image {
    pub fn save(&self) {
        if self.d == 2 {
            let width : u32 = self.g[0] as u32;
            let height : u32 = self.g[1] as u32;

            let mut img : RgbImage = ImageBuffer::new(width, height);
            for row in 0..height {
                for column in 0..width {
                    // img.put_pixel(row, column, self.v[row * width + column] as u8);
                    // println!("{}, {}", row, column);
                }
            }
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut image_data : String = String::new();

        let n : String = format!("voxels({})\n", self.n);
        image_data.push_str(&n);

        let d : String = format!("dim({})\n", self.d);
        image_data.push_str(&d);


        for i in 0..self.d {
            let dimension : i64 = self.g[i as usize];

            let entry : String = format!("resolution[{}]={} ", i, dimension);
            image_data.push_str(&entry);
        }

        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        return write!(f, "{}", image_data);
    }
}

// TODO : for image, print actual png for 1, 2d or voxel for 3d?

struct WeightedBoundaryMatrix {
    coboundaries : i64,
    n : i64,
    d : i64,
    dimension : Vec<i64>,
    value : Vec<f64>,
    offset : Vec<i64>,
    m : i64,
    entry : Vec<f64>
}

struct DistanceMatrix {
    n : i64,
    d : Vec<Vec<f64>>
}

struct PersistenceDiagram {
    p : i64, // number of points in the diagram p
    dimension : Vec<i64>,
    birth : Vec<f64>,
    death : Vec<f64>
}

// TODO : print barcode?

impl fmt::Display for PersistenceDiagram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut diagram_string : String = String::new();

        let p : usize = self.p as usize;

        for i in 0..p {
            let dimension : i64 = self.dimension[i];
            let birth : f64 = self.birth[i];
            let death : f64 = self.death[i];

            let entry : String = format!(" dim({}) : [{},{})\n", dimension, birth, death);
            diagram_string.push_str(&entry);
        }

        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        return write!(f, "{}", diagram_string);
    }
}

// Write function to read in dipha file and split the data.

// Write function to structure data based on filetype.

// Write function to print data.

pub fn read_binary_dipha(file_name : &str) {
    let raw_dipha_file = get_file_as_byte_vec(file_name);

    // println!("The size of the dipha file is {}", raw_dipha_file.len());

    let mut binary = BinaryReader::from_vec(&raw_dipha_file);
    binary.set_endian(Endian::Little);

    is_valid_dipha(&mut binary);
    let file_type : FileType = get_dipha_type(&mut binary);

    println!("File Type is '{:?}'; {}", file_type, file_name);

    match file_type {
        FileType::WeightedBoundaryMatrix => {get_weighted_boundary_matrx(&mut binary);},
        FileType::ImageData => {
            // get_image_data(&mut binary);
            println!("{}", get_image_data(&mut binary));
            // get_image_data(&mut binary).save();
        },
        FileType::PersistenceDiagram => {get_persistence_digram(&mut binary);},
        FileType::DistanceMatrix => {get_distance_matrix(&mut binary);},
        FileType::SparseDistanceMatrix => {},
        FileType::Unknown => {},
        _ => {}
    };

    // println!("{}", get_persistence_digram(&mut binary));
    // println!("{}", get_image_data(&mut binary));
    // get_image_data(&mut binary);

}

fn get_file_as_byte_vec(filename: &str) -> Vec<u8> {


    let mut f = File::open(&filename).expect("no file found");

    let metadata = fs::metadata(&filename).expect("unable to read metadata");

    // println!("metadata.len = {}", metadata.len());
    let mut buffer = vec![0; metadata.len() as usize];

    f.read(&mut buffer).expect("buffer overflow");

    return buffer;
}

fn is_valid_dipha(binary : &mut BinaryReader) -> bool {
    // let position : usize = binary.pos;
    binary.jmp(DIPHA_MAGIC_NUMBER_POSITION); // set location of dipha magic number

    let magic_number : i64 = binary.i64();
    // binary.jmp(position);

    return magic_number == DIPHA_MAGIC_NUMBER;
}

#[test]
fn is_valid_dipha_test() {
    let vector : Vec<u8> = vec![0x00, 0x46, 0xD7, 0xE0, 0x01, 0x00, 0x00, 0x00];

    let mut binary = BinaryReader::from_vec(&vector);
    binary.set_endian(Endian::Little);

    assert_eq!(DIPHA_MAGIC_NUMBER, binary.i64());
    // assert!(true);
}

fn get_dipha_type(binary : &mut BinaryReader) -> FileType {
        binary.jmp(DIPHA_TYPE_POSITION); // set location of dipha type position
        let file_type : i64 = binary.i64();

        return match FromPrimitive::from_i64(file_type) {
            Some(FileType::WeightedBoundaryMatrix) => FileType::WeightedBoundaryMatrix,
            Some(FileType::ImageData) => FileType::ImageData,
            Some(FileType::PersistenceDiagram) => FileType::PersistenceDiagram,
            Some(FileType::DistanceMatrix) => FileType::DistanceMatrix,
            Some(FileType::SparseDistanceMatrix) => FileType::SparseDistanceMatrix,
            Some(FileType::Unknown) => FileType::Unknown,
            None => FileType::Unknown
        };
}

fn get_image_data(binary : &mut BinaryReader) -> Image {
    binary.jmp(DIPHA_DATA_POSITION); // set location of dipha data

    let n : i64 = binary.i64();
    let d : i64 = binary.i64();
    // println!("d={}", d);

    let mut g : Vec<i64> = Vec::new();
    for i in 0..d {
        g.push(binary.i64());
    }

    let mut v : Vec<f64> = Vec::new();
    for i in 0..n {
        v.push(f64::from_bits(binary.u64()));
    }

    assert_eq!(n, v.len() as i64);

    return Image {
        n : n,
        d : d,
        g : g,
        v : v
    };
}

fn get_weighted_boundary_matrx(binary : &mut BinaryReader) -> WeightedBoundaryMatrix {
    binary.jmp(DIPHA_DATA_POSITION); // set location of dipha data

    let coboundaries : i64 = binary.i64();
    let n : i64 = binary.i64();
    let d : i64 = binary.i64();

    let mut dimension : Vec<i64> = Vec::new();
    for i in 0..n {
        dimension.push(binary.i64());
    }

    let mut value : Vec<f64> = Vec::new();
    for i in 0..n {
        value.push(f64::from_bits(binary.u64()));
    }

    let mut offset : Vec <i64> = Vec::new();
    for i in 0..n {
        offset.push(binary.i64());
    }

    let m : i64 = binary.i64();

    let mut entry : Vec<f64> = Vec::new();
    for i in 0..m {
        entry.push(f64::from_bits(binary.u64()));
    }

    // println!("{}", binary.pos);

    return WeightedBoundaryMatrix {
        coboundaries : coboundaries,
        n : n,
        d : d,
        dimension : dimension,
        value : value,
        offset : offset,
        m : m,
        entry : entry
    };
}

fn get_distance_matrix(binary : &mut BinaryReader) -> DistanceMatrix {
    binary.jmp(DIPHA_DATA_POSITION); // set location of dipha data

    let n : i64 = binary.i64();

    let mut d : Vec<Vec<f64>> = Vec::new();
    for row in 0..n {
        let mut row_values : Vec<f64> = Vec::new();
        for column in 0..n {
            row_values.push(f64::from_bits(binary.u64()));
            // println!{"({}, {})", row, column};
        }
        d.push(row_values);
    }
    // println!("{:?}", d.len());

    return DistanceMatrix {
        n : n,
        d : d
    };
}

fn get_persistence_digram(binary : &mut BinaryReader) -> PersistenceDiagram{
    binary.jmp(DIPHA_DATA_POSITION); // set location of dipha data

    let p : i64 = binary.i64();
    let mut dimension : Vec<i64> = Vec::new();
    let mut birth : Vec<f64> = Vec::new();
    let mut death : Vec<f64> = Vec::new();

    for i in 0..(p as usize){
        dimension.push(binary.i64());
        birth.push(f64::from_bits(binary.u64()));
        death.push( f64::from_bits(binary.u64()));
    }

    return PersistenceDiagram {
        p : p,
        dimension : dimension,
        birth : birth,
        death : death
    };

}
