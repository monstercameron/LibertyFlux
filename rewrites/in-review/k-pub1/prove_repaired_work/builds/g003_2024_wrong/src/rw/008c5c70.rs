use crate::{read_u32, write_u8, write_u16, write_u32};

/// Deliberately wrong version: return one-based positions for matching keys.
lf_checker_rt::export!(thiscall, rw_008c5c70(this: u32, label: u32) -> u32 {
    unsafe { search_hash_index(this, label, true) }
});

/// Hash a label and search active sixteen-byte records for its exact key.
lf_checker_rt::export!(thiscall, lane_base_008c5c70(this: u32, label: u32) -> u32 {
    unsafe { search_hash_index(this, label, false) }
});

unsafe fn search_hash_index(this: u32, label: u32, mutant: bool) -> u32 {
    unsafe {
        let hash = lf_checker_rt::callee_cdecl!(HASH_HELPER, u32, label, 0);
        let count = read_u32(this + 0x268);
        let records = read_u32(this + 0x264);
        for index in 0..count {
            let record = records.wrapping_add(index.wrapping_mul(TEXT_RECORD_STRIDE));
            if read_u32(record) == hash {
                return if mutant { index.wrapping_add(1) } else { index };
            }
        }
        u32::MAX
    }
}

const HASH_HELPER: u32 = 1;

const TEXT_RECORD_STRIDE: u32 = 16;

#[inline(always)]
unsafe fn read_u32(address: u32) -> u32 {
    unsafe { (address as *const u32).read_unaligned() }
}
