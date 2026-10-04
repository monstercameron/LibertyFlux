// original: 0x008ac6b0 rage::audCompressorEffect::vf2
/// rage::audCompressorEffect::vf2: slot = this + 4*(9*idx+30), sub at +0x74.
export!(thiscall, rw_008ac6b0(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let idx = *(this.add(0x2c) as *const u32);
        let sub = *(this.add(0x74) as *const u32);
        let slot = (this as u32)
            .wrapping_add(idx.wrapping_mul(9).wrapping_add(0x1e).wrapping_mul(4));
        callee_thiscall!(2, u32, sub, slot);
        callee_thiscall!(3, u32, this as u32)
    }
});

