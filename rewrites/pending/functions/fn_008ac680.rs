// original: 0x008ac680 rage::audCompressorEffect::vf4
/// Returns `this + 4 * (9 * index + 30)` where `index` is the u32 at `this+0x30`.
export!(thiscall, rw_008ac680(this: *const u8) -> u32 {
    unsafe {
        let idx = *(this.add(0x30) as *const u32);
        (this as u32).wrapping_add(idx.wrapping_mul(9).wrapping_add(0x1e).wrapping_mul(4))
    }
});

