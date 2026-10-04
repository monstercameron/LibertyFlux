// original: 0x0088F330 rage::audVoicePcAdpcm::vf3
// 0088F330 rage::audVoicePcAdpcm::vf3: restart the voice (live path) or
// retune it (starting path), push the current gain through, then latch
// whether the gain is exactly zero.
export!(thiscall, rw_0088f330(this: *mut u8, arg: u32) -> () {
    unsafe {
        if (*this.add(0x8C) & 0x10) != 0 {
            callee_thiscall!(1, u32, this as u32, 1);
        } else {
            let scaled = callee_cdecl!(2, u32, arg, *(this.add(0xC) as *const u32));
            let child = *(this.add(0x140) as *const u32);
            callee_thiscall!(3, u32, child, scaled.wrapping_mul(2));
        }
        let gain = *(*(this.add(4) as *const u32) as *const f32);
        callee_thiscall!(4, u32, this as u32, gain.to_bits());
        if gain != *global::<f32>(0xFE8628) {
            *this.add(0x8C) |= 8;
        } else {
            *this.add(0x8C) |= 0x41;
        }
    }
});
