// original: 0x008ac340 rage::audReverbEffect::vf4
/// Returns `this + 4 * (5 * index + 29)` where `index` is the u32 at `this+0x30`.
export!(thiscall, rw_008ac340(this: *const u8) -> u32 {
    unsafe {
        let idx = *(this.add(0x30) as *const u32);
        (this as u32).wrapping_add(idx.wrapping_mul(5).wrapping_add(0x1d).wrapping_mul(4))
    }
});

