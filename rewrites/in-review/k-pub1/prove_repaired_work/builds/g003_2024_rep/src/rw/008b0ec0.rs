// original: 0x008b0ec0 audio_param_store_174c
/// Stores one float at `this+0x174c`.
export!(thiscall, rw_008b0ec0(this: u32, v: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x174c), v);
    }
});

/// Unaligned f32 store. (Carried from r-s35.)
#[inline(always)]
pub unsafe fn wrf(addr: u32, v: f32) {
    st32(addr, v.to_bits());
}

/// Unaligned 32-bit store. (Carried from r-s35.)
#[inline(always)]
pub unsafe fn st32(addr: u32, v: u32) {
    (addr as *mut u32).write_unaligned(v);
}
