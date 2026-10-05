// original: 0x00c7ab90 CTaskComplexWaitForDoorToBeOpen::vf21

/// True when the door the task waits on has opened past its ratio.
/// A null owner link at `[owner+0xb30]` (or a null lookup result) ends
/// the wait at once (returns 1). Else the door handle at `this+0x14`
/// is resolved by the lookup callee and the float at result `+0x1c`
/// is compared against the 0.99 constant: 1 iff strictly above.
/// Only the low byte of EAX is set.
/// Original: 0x00c7ab90 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c7ab90(this: u32, owner: u32) -> u32 {
    unsafe {
        const OFF_OWNER: u32 = 0xb30;
        const OFF_DOOR: u32 = 0x14;
        const OFF_RATIO: u32 = 0x1c;
        const RATIO_OPEN: u32 = 0x00FE88D4;
        const LOOKUP: u32 = 1;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let handle = rd32(owner.wrapping_add(OFF_OWNER));
        if handle == 0 {
            return 1;
        }
        let door = rd32(this.wrapping_add(OFF_DOOR));
        let info = lf_checker_rt::callee_thiscall!(LOOKUP, u32, handle, door);
        if info == 0 {
            return 1;
        }
        let ratio = f32::from_bits(rd32(info.wrapping_add(OFF_RATIO)));
        let open = f32::from_bits(rd32(lf_checker_rt::relocated(RATIO_OPEN)));
        if ratio > open { 1 } else { 0 }
    }
});
