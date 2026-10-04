// original: 0x00cf53d0 climb_task_effect_apply (proposed)

/// Applies a climb task's effect to its target: runs the effect chain, probes
/// the three-word effect vector into a stack slot (the slot address is passed
/// to the probe and the matcher but skipped by the comparison, which
/// snapshots the slot words instead; the probe's own return, the same
/// address, is replaced by a scripted pointer for the same reason), and when
/// the matcher reports zero sets bit 2 of the flag byte at `+0x89` and writes
/// the low-bits pattern 2 to the target's word at `+0x26c`, returning that
/// updated word; otherwise the matcher's result is returned. Always writes 8
/// to `+0x14`.
///
/// Original: 0x00cf53d0 (thiscall: ecx holds the object, one stack word).
lf_checker_rt::export!(thiscall, rw_00cf53d0(this: u32, target: u32) -> u32 {
    unsafe {
        const CHAIN_CALLEE: u32 = 1;
        const PROBE_CALLEE: u32 = 2;
        const MATCH_CALLEE: u32 = 3;
        lf_checker_rt::callee_thiscall!(CHAIN_CALLEE, u32, this, target);
        let slot = [0u32; 3];
        // NOTE: the real probe returns the slot address and the original
        // forwards it to the matcher; the stub instead returns a scripted
        // heap pointer. Both addresses are skipped by the comparison, which
        // snapshots the pointed-to words instead.
        let probe_out: u32 = lf_checker_rt::callee_stdcall!(
            PROBE_CALLEE, u32, &slot as *const u32 as u32, target);
        let matched: u32 = lf_checker_rt::callee_cdecl!(MATCH_CALLEE, u32, target, probe_out);
        if (matched & 0xff) == 0 {
            let flags = ((this + 0x89) as *const u8).read();
            ((this + 0x89) as *mut u8).write(flags | 4);
            let w = ((target + 0x26c) as *const u32).read_unaligned();
            let updated = (w & 0xffff_fffe) | 2;
            ((target + 0x26c) as *mut u32).write_unaligned(updated);
            ((this + 0x14) as *mut u32).write_unaligned(8);
            return updated;
        }
        ((this + 0x14) as *mut u32).write_unaligned(8);
        matched
    }
});
