use std::{collections::LinkedList, os::raw::c_void};

pub struct MsfGifResult {
    pub data: Vec<u8>,

    /// internal use
    alloc_size: usize,
    /// internal use
    context_pointer: *mut c_void,
}

/// internal use
struct MsfCookedFrame {
    pub pixels: Vec<u32>,
    pub depth: u32,
    pub count: u32,
    pub rbits: u32,
    pub gbits: u32,
    pub bbits: u32,
}

pub struct MsfGifBuffer {
    pub data: Vec<u8>,
}

/// buffer, size, count, stream
pub type MsfGifFileWriteFunc = dyn FnMut(&[u8], usize, usize, &mut [u8]) -> usize;

pub struct MsfGifState<'a> {
    pub file_write_func: &'a mut MsfGifFileWriteFunc,
    pub file_write_data: Vec<u8>,
    pub previous_frame: MsfCookedFrame,
    pub current_frame: MsfCookedFrame,
    pub lzw_mem: Vec<i16>,
    pub list: LinkedList<MsfGifBuffer>,
    pub width: u32,
    pub height: u32,
    pub custom_allocator_context: *mut c_void,
    /// needed for transparency to work correctly (because we reach into the previous frame)
    pub frames_submitted: usize,
}
