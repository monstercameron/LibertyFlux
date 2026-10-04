// original: 0x0088c320 rage::audVoiceDSound::vf7
/// Virtual slot 7 of `audVoiceDSound`: reports whether the voice's indexed
/// state word (selector at `this+0xE0`, words at `this+0x128`, stride 64)
/// is zero.
export!(thiscall, rw_0088c320(this_ptr: *const u8) -> u32 {
    unsafe {
        let sel = *(this_ptr.add(0xE0) as *const u32);
        let off = sel.wrapping_mul(64).wrapping_add(0x128);
        let v = *((this_ptr as u32).wrapping_add(off) as *const u32);
        (v == 0) as u32
    }
});
