// original: 0x00977030 audio_scaled_forward_or_tail (proposed)

/// Forward a scaled table value to the shared handler, or zero for the
/// sentinel tag.
///
/// When the tag byte at +0x4 is 0xFF the handler runs with 0. Otherwise the
/// index byte at +0x40 selects a cell `base[c * 0x6F40] + 0x6F14` (base from
/// a global), added to the global factor times the tag, and the handler runs
/// with that sum. Both exits are tail calls, expressed as calls.
/// Original: 0x00977030 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00977030(this: u32) -> u32 {
    unsafe {
        const TAG: u32 = 4;
        const INDEX: u32 = 0x40;
        const FACTOR: u32 = 0x115D968;
        const BASE: u32 = 0x115D988;
        const STRIDE: u32 = 0x6F40;
        const BIAS: u32 = 0x6F14;
        const HANDLE: u32 = 1;
        const SENTINEL: u32 = 0xFF;
        let tag = ((this.wrapping_add(TAG)) as *const u8).read() as u32;
        if tag == SENTINEL {
            return lf_checker_rt::callee_thiscall!(HANDLE, u32, 0);
        }
        let c = ((this.wrapping_add(INDEX)) as *const u8).read() as u32;
        let g1 = lf_checker_rt::global::<u32>(FACTOR).read_unaligned();
        let base = lf_checker_rt::global::<u32>(BASE).read_unaligned();
        let cell = (base
            .wrapping_add(c.wrapping_mul(STRIDE))
            .wrapping_add(BIAS)) as *const u32;
        let fwd = g1.wrapping_mul(tag).wrapping_add(cell.read_unaligned());
        lf_checker_rt::callee_thiscall!(HANDLE, u32, fwd)
    }
});
