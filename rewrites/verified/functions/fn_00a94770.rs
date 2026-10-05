// original: 0x00a94770 fiStreamingDevice::vf33

/// Data address of the entry at `base + delta` via the address callee.
///
/// Adds the argument to the base key at `this+0x08`, resolves the entry
/// through the slot callee, and returns the address callee's answer for
/// that entry. The resolution is passed on unchecked, as in the original.
///
/// Original: thiscall, one stack argument. Two callees (thiscall, 1/0 args).
lf_checker_rt::export!(thiscall, rw_00a94770(this: u32, delta: u32) -> u32 {
    unsafe {
        const BASE_KEY: u32 = 0x08;
        const RESOLVE: u32 = 0;
        const DATA_ADDRESS: u32 = 1;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        let t = rd32(this.wrapping_add(BASE_KEY)).wrapping_add(delta);
        let ent: u32 = lf_checker_rt::callee_thiscall!(RESOLVE, u32, this, t);
        lf_checker_rt::callee_thiscall!(DATA_ADDRESS, u32, ent)
    }
});
