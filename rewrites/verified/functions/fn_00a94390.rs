// original: 0x00a94390 fiStreamingDevice::vf19

/// Quarter-size of the entry at `base + delta`, 0 when unresolvable.
///
/// Adds the argument to the base key at `this+0x08` and resolves the entry
/// through the slot callee. A null resolution gives 0; otherwise the shift
/// callee's result is returned with the high half cleared (the original
/// zeroes `edx`, so the comparison covers `edx:eax`).
///
/// Original: thiscall, one stack argument. Two callees (thiscall, 1/0 args).
lf_checker_rt::export!(thiscall, rw_00a94390(this: u32, delta: u32) -> u64 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const RESOLVE: u32 = 0;
        const SHIFTED: u32 = 1;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let t = rd32(this.wrapping_add(BASE_KEY)).wrapping_add(delta);
        let ent: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, t);
        if ent == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SHIFTED, u32, ent) as u64
    }
});
