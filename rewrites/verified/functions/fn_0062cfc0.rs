// original: 0x0062CFC0 rage::ProceduralTextureVerletWater::vf2 (symbols)

/// Notify the bound hook unless this object is in its default state.
///
/// When the flag at `+0x98` is set, or the tag at `+0x90` differs from the
/// default-hook constant, the hook at `+0x9c` runs (cdecl: `this + 0x90`,
/// `this`); in the default state (flag clear and tag equal) it is skipped.
/// Either way the base implementation runs (patched callee, answer dropped)
/// and the result is 1 in the low byte over the base answer's upper bytes
/// (thiscall, no arguments). Note: the dead stack scratch the original builds
/// (constant copies that are never read) is not reproduced.
lf_checker_rt::export!(thiscall, rw_0062cfc0(this: u32) -> u32 {
    unsafe {
        const TAG: u32 = 0x90;
        const FLAG: u32 = 0x98;
        const HOOK: u32 = 0x9c;
        const DEFAULT_HOOK: u32 = 0x4016A0;
        const CALLEE_BASE: u32 = 2;
        let flag = ((this + FLAG) as *const u32).read_unaligned();
        let tag = ((this + TAG) as *const u32).read_unaligned();
        let skip = flag == 0 && tag == lf_checker_rt::relocated(DEFAULT_HOOK);
        if !skip {
            let tgt = ((this + HOOK) as *const u32).read_unaligned();
            let hook: extern "cdecl" fn(u32, u32) -> u32 =
                core::mem::transmute(tgt as usize);
            hook(this.wrapping_add(TAG), this);
        }
        let base: u32 = lf_checker_rt::callee_thiscall!(CALLEE_BASE, u32, this);
        (base & 0xFFFFFF00) | 1
    }
});
