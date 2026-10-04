// original: 0x0088d2e0 rage::audVoiceDSound::vf6
/// Virtual slot 6 of `audVoiceDSound`: true when any of three status flags
/// is set (bits of bytes at `this+0x8C`, `this+0x98`, or byte `0x18` of
/// the object at `this+4`).
export!(thiscall, rw_0088d2e0(this_ptr: *const u8) -> u8 {
    unsafe {
        if *this_ptr.add(0x8C) & 9 != 0 {
            return 1;
        }
        if *this_ptr.add(0x98) & 1 != 0 {
            return 1;
        }
        let obj = *(this_ptr.add(4) as *const u32) as *const u8;
        if *obj.add(0x18) & 0x40 != 0 {
            return 1;
        }
        0
    }
});
