// original: 0x0088e430 rage::audVoiceSoft::vf6
/// Virtual slot 6 of `audVoiceSoft`: true when the voice's flag byte at
/// `this+0x8C` shows the stopping states, when the helper query over the
/// object at `this+0x130` answers nonzero, or when byte `0x18` of the
/// object at `this+4` has bit 6 set.
export!(thiscall, rw_0088e430(this_ptr: *const u8) -> u8 {
    unsafe {
        let flags = *this_ptr.add(0x8C);
        if flags & 1 != 0 && flags & 0x40 != 0 {
            return 1;
        }
        if flags & 8 != 0 {
            return 1;
        }
        let inner = *(this_ptr.add(0x130) as *const u32);
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
