// original: 0x008ac970 rage::audDelayEffect::vf2
/// rage::audDelayEffect::vf2: slot = this + 8*(9*idx+15), sub at +0x74.
export!(thiscall, rw_008ac970(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let idx = *(this.add(0x2c) as *const u32);
        let sub = *(this.add(0x74) as *const u32);
        let slot = (this as u32)
            .wrapping_add(idx.wrapping_mul(9).wrapping_add(0x0f).wrapping_mul(8));
        callee_thiscall!(2, u32, sub, slot);
        callee_thiscall!(3, u32, this as u32)
    }
});

