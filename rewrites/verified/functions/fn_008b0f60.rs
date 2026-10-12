// original: 0x008b0f60 audio_param_store_1724_1728
/// Stores two floats at `this+0x1724` and `this+0x1728`.
export!(thiscall, rw_008b0f60(this: u32, a: f32, b: f32) -> () {
    unsafe {
        wrf(this.wrapping_add(0x1724), a);
        wrf(this.wrapping_add(0x1728), b);
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
