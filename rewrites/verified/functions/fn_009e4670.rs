// original: 0x009e4670 ped_state_probe (proposed)

/// Probe a ped's iterator state, returning 1 or 0 in `al`.
///
/// Returns 0 when the ready byte at `this + 0x219` is set and the signed
/// level at `this + 0xb80` exceeds 3. Otherwise resolves the iterator
/// (`thiscall` on `this + 0x2b0`, no stack words): a null iterator gives
/// 1. Up to three lookups (`cdecl`, one word from `[iter + 0x18]`,
/// re-read before each) follow: 1 when the first result's word at +8 is
/// 0, 1 when the second's is 6, else bit 15 of the third's word at +0x20
/// decides (1 set, 0 clear). Only `al` is defined on return. `thiscall`,
/// no stack words.
lf_checker_rt::export!(thiscall, rw_009e4670(this: u32) -> u32 {
    unsafe {
        const READY: u32 = 0x219;
        const LEVEL: u32 = 0xb80;
        const ITER_OFF: u32 = 0x2b0;
        const ARG_OFF: u32 = 0x18;
        const FIRST: u32 = 1;
        const LOOKUP: u32 = 2;
        if ((this + READY) as *const u8).read() != 0
            && (((this + LEVEL) as *const u32).read_unaligned() as i32) > 3
        {
            return 0;
        }
        let iter = lf_checker_rt::callee_thiscall!(FIRST, u32, this.wrapping_add(ITER_OFF));
        if iter == 0 {
            return 1;
        }
        let arg = ((iter.wrapping_add(ARG_OFF)) as *const u32).read_unaligned();
        let r1 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg);
        if ((r1.wrapping_add(8)) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        let arg = ((iter.wrapping_add(ARG_OFF)) as *const u32).read_unaligned();
        let r2 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg);
        if ((r2.wrapping_add(8)) as *const u32).read_unaligned() == 6 {
            return 1;
        }
        let arg = ((iter.wrapping_add(ARG_OFF)) as *const u32).read_unaligned();
        let r3 = lf_checker_rt::callee_cdecl!(LOOKUP, u32, arg);
        let bits = ((r3.wrapping_add(0x20)) as *const u32).read_unaligned();
        if ((bits >> 15) & 1) == 1 { 1 } else { 0 }
    }
});
