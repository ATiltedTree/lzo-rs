use crate::algorithms::{Algorithm, ONE};
use crate::{map_return_code, CompressionLevel};
use lzo_sys as ffi;
use std::io;

pub struct Context {
    compress_function: ffi::lzo_compress_t,
    decompress_function: ffi::lzo_decompress_t,
    optimize_function: Option<ffi::lzo_optimize_t>,
    workmem: Vec<std::ffi::c_void>,
}

impl Context {
    pub fn new(algorithm: Algorithm, level: CompressionLevel) -> Self {
        let compress = algorithm.compress_functions[level as usize]
            .expect("This algorithm does not support this compression level");
        let workmem = algorithm.workmem_size[level as usize]
            .expect("This algorithm does not support this compression level");
        Self {
            compress_function: compress,
            decompress_function: algorithm.decompress_function,
            optimize_function: algorithm.optimize_function,
            workmem: Vec::with_capacity(workmem as usize),
        }
    }
    pub fn compress(&mut self, src: &[u8]) -> io::Result<Vec<u8>> {
        let compress_function = self.compress_function;
        let src_len = src.len();

        let mut buffer = Vec::with_capacity(src_len + src_len / 16 + 64 + 3);
        let buffer_len = Box::into_raw(Box::new(0));
        unsafe {
            let code = compress_function(
                src.as_ptr(),
                src_len as u64,
                buffer.as_mut_ptr(),
                buffer_len,
                self.workmem.as_mut_ptr(),
            );
            buffer.set_len(*buffer_len as usize);
            map_return_code(code, buffer)
        }
    }
    pub fn decompress(&mut self, src: &[u8]) -> io::Result<Vec<u8>> {
        let decompress_function = self.decompress_function;
        let src_len = src.len();

        let mut buffer = Vec::with_capacity(src_len);
        let buffer_len = Box::into_raw(Box::new(0));
        unsafe {
            let code = decompress_function(
                src.as_ptr(),
                src_len as u64,
                buffer.as_mut_ptr(),
                buffer_len,
                self.workmem.as_mut_ptr(),
            );
            buffer.set_len(*buffer_len as usize);
            map_return_code(code, buffer)
        }
    }
    pub fn optimize(&mut self, src: &mut [u8]) -> io::Result<Vec<u8>> {
        if let Some(optimize_function) = self.optimize_function {
            let src_len = src.len();

            let mut buffer = Vec::with_capacity(src_len + src_len / 16 + 64 + 3);
            let buffer_len = Box::into_raw(Box::new(0));
            unsafe {
                let code = optimize_function(
                    src.as_mut_ptr(),
                    src_len as u64,
                    buffer.as_mut_ptr(),
                    buffer_len,
                    self.workmem.as_mut_ptr(),
                );
                buffer.set_len(*buffer_len as usize);
                map_return_code(code, buffer)
            }
        } else {
            unimplemented!()
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        let compress = ONE.compress_functions[0].unwrap();
        let workmem = ONE.workmem_size[0].unwrap();
        Self {
            compress_function: compress,
            decompress_function: ONE.decompress_function,
            optimize_function: ONE.optimize_function,
            workmem: Vec::with_capacity(workmem as usize),
        }
    }
}
