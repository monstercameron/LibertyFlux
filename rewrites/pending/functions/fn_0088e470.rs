// original: 0x0088e470 rage::audVoiceSoft::vf7
/// Virtual slot 7 of `audVoiceSoft`: true only when bit 4 of the flag byte
/// at `this+0x8C` is set and the indexed state word (selector at
/// `this+0x118`, words at `this+0xCC`, stride 64) is zero.
export!(thiscall, rw_0088e470(this_ptr: *const u8) -> u8 {
    unsafe {
        if *this_ptr.add(0x8C) & 0x10 == 0 {
            return 0;
        }
        let sel = *(this_ptr.add(0x118) as *const u32);
        let off = sel.wrapping_mul(64).wrapping_add(0xCC);
        let v = *((this_ptr as u32).wrapping_add(off) as *const u32);
        (v == 0) as u8
    }
});
