// original: 0x008b0df0 audio_meter_store_clamped4
/// Clamps four input floats into [0,1] and stores them at `this+0x1784`.
export!(thiscall, rw_008b0df0(this: u32, src: u32) -> () {
    unsafe {
        let one = *global::<f32>(0xFE88E8);
        for i in 0..4u32 {
            let v = rdf(src.wrapping_add(i * 4));
            let c = if v < 0.0 {
                0.0
            } else if v > one {
                one
            } else {
                v
            };
            wrf(this.wrapping_add(0x1784 + i * 4), c);
        }
    }
});

/// Unaligned f32 load. (Carried from r-s35.)
#[inline(always)]
pub unsafe fn rdf(addr: u32) -> f32 {
    f32::from_bits(ld32(addr))
}

/// Unaligned f32 store. (Carried from r-s35.)
#[inline(always)]
pub unsafe fn wrf(addr: u32, v: f32) {
    st32(addr, v.to_bits());
}

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
