// original: 0x008b0dc0 audio_meter_store_group3
/// Copies four words into the meter slot group at `this+0x1774`.
export!(thiscall, rw_008b0dc0(this: u32, src: u32) -> () {
    unsafe {
        st32(this.wrapping_add(0x1774), ld32(src));
        st32(this.wrapping_add(0x1778), ld32(src.wrapping_add(4)));
        st32(this.wrapping_add(0x177c), ld32(src.wrapping_add(8)));
        st32(this.wrapping_add(0x1780), ld32(src.wrapping_add(12)));
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
