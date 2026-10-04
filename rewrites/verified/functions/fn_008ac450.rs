// original: 0x008ac450 rage::audReverbEffect::vf2
/// rage::audReverbEffect::vf2: slot = this + 4*(5*idx+29), sub-object at +0xc8.
export!(thiscall, rw_008ac450(this: *mut u8) -> u32 {
    unsafe {
        callee_thiscall!(1, u32, this as u32);
        let idx = *(this.add(0x2c) as *const u32);
        let sub = *(this.add(0xc8) as *const u32);
        let slot = (this as u32)
            .wrapping_add(idx.wrapping_mul(5).wrapping_add(0x1d).wrapping_mul(4));
        callee_thiscall!(2, u32, sub, slot);
        callee_thiscall!(3, u32, this as u32)
    }
});

