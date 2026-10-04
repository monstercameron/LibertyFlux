// original: 0x0088F540 rage::audVoicePcAdpcm::vf11
// 0088F540 rage::audVoicePcAdpcm::vf11: when live, reconcile the level flag
// (restart, count down, or call the voice hook), push the gain through, then
// tail into the voice poll and return its result.
export!(thiscall, rw_0088f540(this: *mut u8) -> u32 {
    unsafe {
        if (*this.add(0x8C) & 0x10) != 0 {
            let level = callee_thiscall!(1, u32, *(this.add(0x140) as *const u32));
            let flag = if level >= 0x10000 { 1u32 } else { 0u32 };
            if flag != *(this.add(0x110) as *const u32) {
                if *this.add(0x14C) == 0 {
                    callee_thiscall!(2, u32, this as u32, 0);
                } else if *(this.add(0x114) as *const u32) > 0 {
                    callee_thiscall!(2, u32, this as u32, 0);
                    let left = this.add(0x114) as *mut u32;
                    *left = left.read().wrapping_sub(1);
                } else {
                    let vtable = *(this as *const u32);
                    let target = *((vtable as *const u8).add(0x14) as *const u32);
                    let hook: extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(target as usize);
                    hook(this as u32);
                }
            }
        }
        let gain = *(*(this.add(4) as *const u32) as *const u32);
        callee_thiscall!(4, u32, this as u32, gain);
        callee_thiscall!(5, u32, this as u32)
    }
});
