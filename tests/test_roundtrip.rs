use std::fs::File;
use std::io::prelude::*;
use std::convert::TryInto;

use ::fastlz_rs::{
    compress as fastlz_compress_level,
    decompress as fastlz_decompress
};

mod refimpl;
use refimpl::{ref_level1_decompress, ref_level2_decompress};

fn compare(name: &str, a: &Vec<u8>, b: &Vec<u8>) -> bool {
    let a_iter = a.iter();
    let b_iter = b.iter();
    let res = a_iter.zip(b_iter)
        .enumerate()
        .find(|(_idx, (ea, eb))| {
            ea != eb
        });

    if let Some(invalid_element) = res {
        let (idx, (ea, eb)) = invalid_element;
        println!("Error on : {}", name);
        println!("Different at index {}: expecting {:02x}, actual {:02x}", idx, ea, eb);
        false
    } else {
        true
    }
}


// Same as test_roundtrip_level1 EXCEPT that the decompression is carried out
// using the highly-simplified, unoptimized vanilla reference decompressor.
fn test_ref_decompressor_level1(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];
    let compressed_size = fastlz_compress_level(
        1,
        &file_buffer,
        &mut compressed_buffer
    ).unwrap_or(0);

    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    ref_level1_decompress(
        &compressed_buffer,
        compressed_size,
        &mut uncompressed_buffer
    );

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}


// Same as test_roundtrip_level2 EXCEPT that the decompression is carried out
// using the highly-simplified, unoptimized vanilla reference decompressor.
fn test_ref_decompressor_level2(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];

    let compressed_size = fastlz_compress_level(
        2,
        &file_buffer,
        &mut compressed_buffer
    ).unwrap_or(0);
    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    /* intentionally mask out the block tag */
    compressed_buffer[0] = compressed_buffer[0] & 31u8;

    ref_level2_decompress(
        &compressed_buffer,
        compressed_size,
        &mut uncompressed_buffer
    );

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}


// Read the content of the file.
// Compress it first using the Level 1 compressor.
// Decompress the output with Level 1 decompressor.
// Compare the result with the original file content.
fn test_roundtrip_level1(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                        .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];
    let compressed_size = fastlz_compress_level(
        1,
        &file_buffer,
        &mut compressed_buffer
    ).unwrap_or(0);
    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    assert_ne!(compressed_size, 0);

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    let uncompressed_size = fastlz_decompress(&compressed_buffer, &mut uncompressed_buffer).unwrap_or(0);

    assert_eq!(file_size, uncompressed_size);

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}


// Read the content of the file.
// Compress it first using the Level 2 compressor.
// Decompress the output with Level 2 decompressor.
// Compare the result with the original file content.
fn test_roundtrip_level2(name: &str, file_name: &str) {
    let mut f = File::open(file_name)
                        .expect(&format!("Error: can not open {}", file_name));
    let file_size: usize = f.metadata().unwrap().len().try_into().unwrap();

    let mut file_buffer: Vec<u8> = Vec::new();
    let read = f.read_to_end(&mut file_buffer)
                    .expect("Error: Cannot read all file into memory");
    assert_eq!(read, file_size, "Error: cannot read all bytes!");

    let compressed_buffer_size = (1.05 * file_size as f64) as usize;
    let mut compressed_buffer: Vec<u8> = vec![0u8; compressed_buffer_size];
    let compressed_size = fastlz_compress_level(
        2,
        &file_buffer,
        &mut compressed_buffer
    ).unwrap_or(0);
    let ratio = 100.0 * compressed_size as f64 / file_size as f64;

    assert_ne!(compressed_size, 0);

    let mut uncompressed_buffer: Vec<u8>  = vec!['-' as u8; file_size];
    let uncompressed_size = fastlz_decompress(&compressed_buffer, &mut uncompressed_buffer).unwrap_or(0);

    assert_eq!(file_size, uncompressed_size);

    assert!(compare(
        file_name,
        &file_buffer,
        &uncompressed_buffer
    ));
    println!("{:25} {:10} -> {:10} ({:.2}%)", name, file_size, compressed_size, ratio);
}



const CORPORA_DIR: &'static str = "./data/compression-corpus/";
const CORPORA: &'static[&'static str] = &[
    "canterbury/alice29.txt",
    "canterbury/asyoulik.txt",
    "canterbury/cp.html",
    "canterbury/fields.c",
    "canterbury/grammar.lsp",
    "canterbury/kennedy.xls",
    "canterbury/lcet10.txt",
    "canterbury/plrabn12.txt",
    "canterbury/ptt5",
    "canterbury/sum",
    "canterbury/xargs.1",
    "silesia/dickens",
    "silesia/mozilla",
    "silesia/mr",
    "silesia/nci",
    "silesia/ooffice",
    "silesia/osdb",
    "silesia/reymont",
    "silesia/samba",
    "silesia/sao",
    "silesia/webster",
    "silesia/x-ray",
    "silesia/xml",
    "enwik/enwik8.txt"
];


#[test]
fn test_ref_impl_level1() {
    println!("Test reference decompressor for Level 1");
    CORPORA.iter().for_each(|corpus| {
        let f = format!("{}{}", CORPORA_DIR, corpus);
        test_ref_decompressor_level1(*corpus, &f);
    });
    println!();
}

#[test]
fn test_ref_impl_level2() {
    println!("Test reference decompressor for Level 2");
    CORPORA.iter().for_each(|corpus| {
        let f = format!("{}{}", CORPORA_DIR, corpus);
        test_ref_decompressor_level2(*corpus, &f);
    });
    println!();
}

#[test]
fn test_round_trip_level1() {
    println!("Test round-trip for Level 1");
    CORPORA.iter().for_each(|corpus| {
        let f = format!("{}{}", CORPORA_DIR, corpus);
        test_roundtrip_level1(*corpus, &f);
    });
    println!();
}

#[test]
fn test_round_trip_level2() {
    println!("Test round-trip for Level 2");
    CORPORA.iter().for_each(|corpus| {
        let f = format!("{}{}", CORPORA_DIR, corpus);
        test_roundtrip_level2(*corpus, &f);
    });
    println!();
}
