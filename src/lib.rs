#![deny(clippy::nursery)]
#![deny(clippy::pedantic)]
#![doc(html_root_url = "https://docs.rs/lzo/0.1.0")]

use lzo_sys as ffi;

use std::io;

pub mod algorithms;
mod context;

pub use self::context::Context;

fn map_return_code(code: i32, output_buffer: Vec<u8>) -> io::Result<Vec<u8>> {
    match code {
        ffi::LZO_E_OK => Ok(output_buffer),
        ffi::LZO_E_OUT_OF_MEMORY => Err(io::Error::new(io::ErrorKind::Other, "Out of Memory!")),
        ffi::LZO_E_NOT_COMPRESSIBLE | ffi::LZO_E_NOT_YET_IMPLEMENTED => {
            Err(io::Error::new(io::ErrorKind::Other, "Unused!"))
        }
        ffi::LZO_E_OUTPUT_OVERRUN
        | ffi::LZO_E_INPUT_OVERRUN
        | ffi::LZO_E_LOOKBEHIND_OVERRUN
        | ffi::LZO_E_EOF_NOT_FOUND => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Input data is corrupted!",
        )),
        ffi::LZO_E_INPUT_NOT_CONSUMED => Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "More data after end of compressed block!",
        )),
        ffi::LZO_E_INVALID_ARGUMENT | ffi::LZO_E_INVALID_ALIGNMENT => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Your argument was invalid!",
        )),
        ffi::LZO_E_OUTPUT_NOT_CONSUMED => Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "Too much output was allocated!",
        )),
        ffi::LZO_E_ERROR | ffi::LZO_E_INTERNAL_ERROR => {
            Err(io::Error::new(io::ErrorKind::Other, "Unknown Error!"))
        }

        _ => Err(io::Error::new(io::ErrorKind::Other, "Unknown Error!")),
    }
}

#[derive(Copy, Clone)]
pub enum CompressionLevel {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Good = 10,
    Best = 11,
}

#[test]
fn cycle_one() {
    let mut context = Context::new(algorithms::ONE, CompressionLevel::Zero);
    let input = b"TEST TEST";
    let compressed = context.compress(input).unwrap();
    let decompressed = context.decompress(compressed.as_slice()).unwrap();
    assert_eq!(input, decompressed.as_slice())
}

#[test]
#[should_panic]
fn one_optimize() {
    let mut input = String::from("TEST TEST");
    let mut context = Context::new(algorithms::ONE, CompressionLevel::Zero);
    context.optimize(unsafe { input.as_bytes_mut() }).unwrap();
}

#[test]
#[should_panic]
fn one_unsupported_level() {
    Context::new(algorithms::ONE, CompressionLevel::Nine);
}
