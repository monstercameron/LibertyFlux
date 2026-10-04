// original: 0x0088d310 rage::audVoiceDSoundAdpcm::vf7
/// Virtual slot 7 of `audVoiceDSoundAdpcm`: reports whether the voice's
/// indexed state word (selector at `this+0xF4`, words at `this+0x13C`,
/// stride 64) is zero.
export!(thiscall, rw_0088d310(this_ptr: *const u8) -> u32 {
    unsafe {
        let sel = *(this_ptr.add(0xF4) as *const u32);
        let off = sel.wrapping_mul(64).wrapping_add(0x13C);
        let v = *((this_ptr as u32).wrapping_add(off) as *const u32);
        (v == 0) as u32
    }
});
