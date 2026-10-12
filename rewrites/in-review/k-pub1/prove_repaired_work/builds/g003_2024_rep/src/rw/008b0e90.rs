// original: 0x008b0e90 audio_meter_store_group2
/// Copies four words into the meter slot group at `this+0x1764`.
export!(thiscall, rw_008b0e90(this: u32, src: u32) -> () {
    unsafe {
        st32(this.wrapping_add(0x1764), ld32(src));
        st32(this.wrapping_add(0x1768), ld32(src.wrapping_add(4)));
        st32(this.wrapping_add(0x176c), ld32(src.wrapping_add(8)));
        st32(this.wrapping_add(0x1770), ld32(src.wrapping_add(12)));
    }
});

/// Unaligned 32-bit load. (Carried from r-s35.)
#[inline(always)]
pub unsafe fn ld32(addr: u32) -> u32 {
    (addr as *const u32).read_unaligned()
}

/// Unaligned 32-bit store. (Carried from r-s35.)
#[inline(always)]
pub unsafe fn st32(addr: u32, v: u32) {
    (addr as *mut u32).write_unaligned(v);
}
