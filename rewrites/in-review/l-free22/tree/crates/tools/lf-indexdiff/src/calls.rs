//! The verified rewrites of the instances, each behind one function of the same shape.
//!
//! Entry `i` calls the rewrite of entry `i` of `lf_core::indexed_read::instances::READS`.

/// The rewrite of `indexed_byte_fetch`, called with the object at `base` and the index; the result is its bits.
pub fn call_0(base: *mut u8, index: u32) -> u32 {
    crate::rewrites::m_009a9c60::rw_009A9C60(base as usize as u32, index)
}

/// The rewrite of `CPedModelInfo::vf14`, called with the object at `base` and the index; the result is its bits.
pub fn call_1(base: *mut u8, index: u32) -> u32 {
    crate::rewrites::m_009dc5d0::rw_009DC5D0(base as usize as u32, index)
}

/// The rewrite of `stream_entry_tail_float`, called with the object at `base` and the index; the result is its bits.
pub fn call_2(base: *mut u8, index: u32) -> u32 {
    crate::rewrites::m_00d9ec30::rw_s675_00d9ec30(base as usize as u32, index).to_bits()
}

/// The rewrite of `stream_sample_flag`, called with the object at `base` and the index; the result is its bits.
pub fn call_3(base: *mut u8, index: u32) -> u32 {
    crate::rewrites::m_00d9f020::rw_s675_00d9f020(base as usize as u32, index)
}

/// The rewrite of `stream_sample_float`, called with the object at `base` and the index; the result is its bits.
pub fn call_4(base: *mut u8, index: u32) -> u32 {
    crate::rewrites::m_00d9f040::rw_s675_00d9f040(base as usize as u32, index).to_bits()
}

/// The rewrite of `stream_sample_word`, called with the object at `base` and the index; the result is its bits.
pub fn call_5(base: *mut u8, index: u32) -> u32 {
    crate::rewrites::m_00d9f060::rw_s675_00d9f060(base as usize as u32, index)
}

/// The call table, in the order of the instances table.
pub const CALLS: [fn(*mut u8, u32) -> u32; 6] = [call_0, call_1, call_2, call_3, call_4, call_5];
