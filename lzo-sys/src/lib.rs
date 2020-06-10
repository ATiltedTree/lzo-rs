#![allow(non_camel_case_types)]
#![no_std]

const SIZEOF_UCHAR_P: libc::c_uint = core::mem::size_of::<*const libc::c_uchar>() as libc::c_uint;
const SIZEOF_ULONG: libc::c_uint = core::mem::size_of::<libc::c_ulong>() as libc::c_uint;
const SIZEOF_SHORT: libc::c_uint = core::mem::size_of::<libc::c_short>() as libc::c_uint;

/////////////////////////////////////////////////////////////////////////////
// Return Codes
/////////////////////////////////////////////////////////////////////////////

pub const LZO_E_OK: libc::c_int = 0;
pub const LZO_E_ERROR: libc::c_int = -1;
pub const LZO_E_OUT_OF_MEMORY: libc::c_int = -2;
pub const LZO_E_NOT_COMPRESSIBLE: libc::c_int = -3;
pub const LZO_E_INPUT_OVERRUN: libc::c_int = -4;
pub const LZO_E_OUTPUT_OVERRUN: libc::c_int = -5;
pub const LZO_E_LOOKBEHIND_OVERRUN: libc::c_int = -6;
pub const LZO_E_EOF_NOT_FOUND: libc::c_int = -7;
pub const LZO_E_INPUT_NOT_CONSUMED: libc::c_int = -8;
pub const LZO_E_NOT_YET_IMPLEMENTED: libc::c_int = -9;
pub const LZO_E_INVALID_ARGUMENT: libc::c_int = -10;
pub const LZO_E_INVALID_ALIGNMENT: libc::c_int = -11;
pub const LZO_E_OUTPUT_NOT_CONSUMED: libc::c_int = -12;
pub const LZO_E_INTERNAL_ERROR: libc::c_int = -99;

/////////////////////////////////////////////////////////////////////////////
// WorkMEM sizes
/////////////////////////////////////////////////////////////////////////////

pub const LZO1_MEM_COMPRESS: libc::c_uint = 8192 * SIZEOF_UCHAR_P;
pub const LZO1_99_MEM_COMPRESS: libc::c_uint = 65536 * SIZEOF_UCHAR_P;
pub const LZO1_MEM_DECOMPRESS: libc::c_uint = 0;

pub const LZO1A_MEM_COMPRESS: libc::c_uint = 8192 * SIZEOF_UCHAR_P;
pub const LZO1A_99_MEM_COMPRESS: libc::c_uint = 65536 * SIZEOF_UCHAR_P;
pub const LZO1A_MEM_DECOMPRESS: libc::c_uint = 0;

pub const LZO1B_MEM_COMPRESS: libc::c_uint = 16384 * SIZEOF_UCHAR_P;
pub const LZO1B_99_MEM_COMPRESS: libc::c_uint = 65536 * SIZEOF_UCHAR_P;
pub const LZO1B_999_MEM_COMPRESS: libc::c_uint = 3 * 65536 * SIZEOF_ULONG;
pub const LZO1B_MEM_DECOMPRESS: libc::c_uint = 0;

pub const LZO1C_MEM_COMPRESS: libc::c_uint = 16384 * SIZEOF_UCHAR_P;
pub const LZO1C_99_MEM_COMPRESS: libc::c_uint = 65536 * SIZEOF_UCHAR_P;
pub const LZO1C_999_MEM_COMPRESS: libc::c_uint = 3 * 16384 * SIZEOF_SHORT;
pub const LZO1C_MEM_DECOMPRESS: libc::c_uint = 0;

pub const LZO1F_MEM_COMPRESS: libc::c_uint = 16384 * SIZEOF_UCHAR_P;
pub const LZO1F_999_MEM_COMPRESS: libc::c_uint = 5 * 16384 * SIZEOF_SHORT;
pub const LZO1F_MEM_DECOMPRESS: libc::c_uint = 0;

pub const LZO1X_MEM_COMPRESS: libc::c_uint = LZO1X_1_MEM_COMPRESS;
pub const LZO1X_1_MEM_COMPRESS: libc::c_uint = 16384 * SIZEOF_UCHAR_P;
pub const LZO1X_1_11_MEM_COMPRESS: libc::c_uint = 2048 * SIZEOF_UCHAR_P;
pub const LZO1X_1_12_MEM_COMPRESS: libc::c_uint = 4096 * SIZEOF_UCHAR_P;
pub const LZO1X_1_15_MEM_COMPRESS: libc::c_uint = 32768 * SIZEOF_UCHAR_P;
pub const LZO1X_1_999_MEM_COMPRESS: libc::c_uint = 14 * 16384 * SIZEOF_SHORT;
pub const LZO1X_MEM_DECOMPRESS: libc::c_uint = 0;
pub const LZO1X_MEM_OPTIMIZE: libc::c_uint = 0;

pub const LZO1Y_MEM_COMPRESS: libc::c_uint = 16384 * SIZEOF_UCHAR_P;
pub const LZO1Y_999_MEM_COMPRESS: libc::c_uint = 14 * 16384 * SIZEOF_SHORT;
pub const LZO1Y_MEM_DECOMPRESS: libc::c_uint = 0;
pub const LZO1Y_MEM_OPTIMIZE: libc::c_uint = 0;

pub const LZO1Z_999_MEM_COMPRESS: libc::c_uint = 14 * 16384 * SIZEOF_SHORT;
pub const LZO1Z_MEM_DECOMPRESS: libc::c_uint = 0;

pub const LZO2A_999_MEM_COMPRESS: libc::c_uint = 8 * 16384 * SIZEOF_SHORT;
pub const LZO2A_MEM_DECOMPRESS: libc::c_uint = 0;

/////////////////////////////////////////////////////////////////////////////
// Compression levels
/////////////////////////////////////////////////////////////////////////////

pub const LZO1B_BEST_SPEED: libc::c_int = 1;
pub const LZO1B_BEST_COMPRESSION: libc::c_int = 9;
pub const LZO1B_DEFAULT_COMPRESSION: libc::c_int = -1;

pub const LZO1C_BEST_SPEED: libc::c_int = 1;
pub const LZO1C_BEST_COMPRESSION: libc::c_int = 9;
pub const LZO1C_DEFAULT_COMPRESSION: libc::c_int = -1;

/////////////////////////////////////////////////////////////////////////////
// comperess/decompress/optimize function types
/////////////////////////////////////////////////////////////////////////////

pub type lzo_compress_t = unsafe extern "C" fn(
    src: *const libc::c_uchar,
    src_len: libc::c_ulong,
    dst: *mut libc::c_uchar,
    dst_len: *mut libc::c_ulong,
    wrkmem: *mut libc::c_void,
) -> libc::c_int;
pub type lzo_decompress_t = unsafe extern "C" fn(
    src: *const libc::c_uchar,
    src_len: libc::c_ulong,
    dst: *mut libc::c_uchar,
    dst_len: *mut libc::c_ulong,
    wrkmem: *mut libc::c_void,
) -> libc::c_int;
pub type lzo_optimize_t = unsafe extern "C" fn(
    src: *mut libc::c_uchar,
    src_len: libc::c_ulong,
    dst: *mut libc::c_uchar,
    dst_len: *mut libc::c_ulong,
    wrkmem: *mut libc::c_void,
) -> libc::c_int;
pub type lzo_compress_dict_t = unsafe extern "C" fn(
    src: *const libc::c_uchar,
    src_len: libc::c_ulong,
    dst: *mut libc::c_uchar,
    dst_len: *mut libc::c_ulong,
    wrkmem: *mut libc::c_void,
    dict: *const libc::c_uchar,
    dict_len: libc::c_ulong,
) -> libc::c_int;
pub type lzo_decompress_dict_t = unsafe extern "C" fn(
    src: *const libc::c_uchar,
    src_len: libc::c_ulong,
    dst: *mut libc::c_uchar,
    dst_len: *mut libc::c_ulong,
    wrkmem: *mut libc::c_void,
    dict: *const libc::c_uchar,
    dict_len: libc::c_ulong,
) -> libc::c_int;

/////////////////////////////////////////////////////////////////////////////
// callback function types
/////////////////////////////////////////////////////////////////////////////

pub type lzo_alloc_func_t = unsafe extern "C" fn(
    self_: *mut lzo_callback_t,
    items: libc::c_ulong,
    size: libc::c_ulong,
) -> *mut libc::c_void;
pub type lzo_free_func_t = unsafe extern "C" fn(self_: *mut lzo_callback_t, ptr: *mut libc::c_void);

pub type lzo_progress_func_t = unsafe extern "C" fn(
    self_: *mut lzo_callback_t,
    textsize: libc::c_ulong,
    codesize: libc::c_ulong,
    arg4: libc::c_int,
);

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lzo_callback_t {
    pub nalloc: lzo_alloc_func_t,
    pub nfree: lzo_free_func_t,
    pub nprogress: lzo_progress_func_t,
    pub user1: *mut libc::c_void,
    pub user2: libc::c_ulong,
    pub user3: libc::c_ulong,
}

/////////////////////////////////////////////////////////////////////////////
// public library functions
/////////////////////////////////////////////////////////////////////////////

extern "C" {
    pub fn lzo_version() -> libc::c_uint;
    pub fn lzo_version_string() -> *const libc::c_char;
    pub fn lzo_version_date() -> *const libc::c_char;

    pub fn lzo1_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1_99_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1a_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1a_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1a_99_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1b_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        compression_level: libc::c_int,
    ) -> libc::c_int;
    pub fn lzo1b_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_1_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_2_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_3_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_4_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_5_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_6_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_7_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_8_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_9_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_99_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1b_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1c_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        compression_level: libc::c_int,
    ) -> libc::c_int;
    pub fn lzo1c_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_1_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_2_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_3_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_4_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_5_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_6_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_7_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_8_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_9_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_99_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1c_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1f_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1f_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1f_1_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1f_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1x_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_1_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_1_11_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_1_12_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_1_15_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1x_999_compress_dict(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
    ) -> libc::c_int;
    pub fn lzo1x_999_compress_level(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
        cb: *mut lzo_callback_t,
        compression_level: libc::c_int,
    ) -> libc::c_int;
    pub fn lzo1x_decompress_dict_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
    ) -> libc::c_int;
    pub fn lzo1x_optimize(
        src: *mut libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1y_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1y_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1y_1_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1y_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1y_999_compress_dict(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
    ) -> libc::c_int;
    pub fn lzo1y_999_compress_level(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
        cb: *mut lzo_callback_t,
        compression_level: libc::c_int,
    ) -> libc::c_int;
    pub fn lzo1y_decompress_dict_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
    ) -> libc::c_int;
    pub fn lzo1y_optimize(
        src: *mut libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;

    pub fn lzo1z_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1z_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1z_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo1z_999_compress_dict(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
    ) -> libc::c_int;
    pub fn lzo1z_999_compress_level(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
        cb: *mut lzo_callback_t,
        compression_level: libc::c_int,
    ) -> libc::c_int;
    pub fn lzo1z_decompress_dict_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
        dict: *const libc::c_uchar,
        dict_len: libc::c_ulong,
    ) -> libc::c_int;

    pub fn lzo2a_decompress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo2a_decompress_safe(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
    pub fn lzo2a_999_compress(
        src: *const libc::c_uchar,
        src_len: libc::c_ulong,
        dst: *mut libc::c_uchar,
        dst_len: *mut libc::c_ulong,
        wrkmem: *mut libc::c_void,
    ) -> libc::c_int;
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn version() {
        unsafe {
            lzo_version();
            lzo_version_string();
            lzo_version_date();
        }
    }
}
