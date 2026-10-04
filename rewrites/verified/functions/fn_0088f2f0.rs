// original: 0x0088f2f0 rage::audVoicePcAdpcm::vf6
/// Virtual slot 6 of `audVoicePcAdpcm`: same three-way status check as its
/// sibling, with the helper object at `this+0x140`.
export!(thiscall, rw_0088f2f0(this_ptr: *const u8) -> u8 {
    unsafe {
        let flags = *this_ptr.add(0x8C);
        if flags & 1 != 0 && flags & 0x40 != 0 {
            return 1;
        }
        if flags & 8 != 0 {
            return 1;
        }
        let inner = *(this_ptr.add(0x140) as *const u32);
        let ans: u32 = callee_thiscall!(1, u32, inner);
        if ans & 0xFF != 0 {
            return 1;
        }
        let obj = *(this_ptr.add(4) as *const u32) as *const u8;
        if *obj.add(0x18) & 0x40 != 0 {
            return 1;
        }
        0
    }
});
