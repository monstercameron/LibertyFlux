// original: 0x00cf6e30 climb_task_effect_gate (proposed)

/// Gates a climb task's effect on the state float: when the state at `+0x10`
/// is present and its float at `+0x4c` exceeds 0.8, the effect vector is
/// probed into a stack slot (addresses skipped, slot words snapshotted, the
/// probe's address return replaced by a scripted pointer as in `rw_00cf53d0`)
/// and matched; a zero match returns early. Otherwise the worker runs with
/// the state float (or 1.0 when no state), bit 2 of `+0x89` is set, the
/// target's word at `+0x26c` gets low-bits pattern 2, the effect chain runs,
/// 8 is written to `+0x14`, and the chain's result is returned. Early exits
/// return the state pointer or the matcher's result respectively.
///
/// Original: 0x00cf6e30 (thiscall: ecx holds the object, three stack words of
/// which the third is unread).
lf_checker_rt::export!(thiscall, rw_00cf6e30(this: u32, target: u32, extra: u32, _unused: u32) -> u32 {
    unsafe {
        const GATE_ADDR: u32 = 0x00fe8898; // 0.8f
        const UNIT_ADDR: u32 = 0x00fe88e8; // 1.0f
        const PROBE_CALLEE: u32 = 1;
        const MATCH_CALLEE: u32 = 2;
        const WORKER_CALLEE: u32 = 3;
        const CHAIN_CALLEE: u32 = 4;
        let state = ((this + 0x10) as *const u32).read_unaligned();
        if state != 0 {
            let gate = f32::from_bits((lf_checker_rt::global::<u32>(GATE_ADDR)).read_unaligned());
            let f = f32::from_bits(((state + 0x4c) as *const u32).read_unaligned());
            if f > gate {
                let slot = [0u32; 3];
                let probe_out: u32 = lf_checker_rt::callee_stdcall!(
                    PROBE_CALLEE, u32, &slot as *const u32 as u32, target);
                let matched: u32 =
                    lf_checker_rt::callee_cdecl!(MATCH_CALLEE, u32, target, probe_out);
                if (matched & 0xff) == 0 {
                    return matched;
                }
            } else {
                return state;
            }
        }
        let unit = f32::from_bits((lf_checker_rt::global::<u32>(UNIT_ADDR)).read_unaligned());
        let g = if state != 0 {
            f32::from_bits(((state + 0x4c) as *const u32).read_unaligned())
        } else {
            unit
        };
        let w78 = ((this + 0x78) as *const u32).read_unaligned();
        let w7c = ((this + 0x7c) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(WORKER_CALLEE, u32, this, extra, target, w78, w7c, g.to_bits());
        let flags = ((this + 0x89) as *const u8).read();
        ((this + 0x89) as *mut u8).write(flags | 4);
        let w = ((target + 0x26c) as *const u32).read_unaligned();
        ((target + 0x26c) as *mut u32).write_unaligned((w & 0xffff_fffe) | 2);
        let out = lf_checker_rt::callee_thiscall!(CHAIN_CALLEE, u32, this, target);
        ((this + 0x14) as *mut u32).write_unaligned(8);
        out
    }
});
