// original: 0x00cfd950 task_state_remap_by_mode (proposed)

/// Remaps the state id stored at `out` according to the task mode and a
/// probed flag, leaving ids outside the remapped pair untouched.
///
/// `obj` is the task object with a mode word at `+0x2b0`; `out` points at a
/// state id. The ready probe (callee 1, thiscall, one stack arg = 1) runs
/// with ECX = `obj + 0x2b0`; a nonzero low byte sets the flag, otherwise the
/// lookup (callee 2, thiscall, no stack args) runs twice with the same ECX
/// and the flag comes from bit 13 of the dword at result `+0x20` of the
/// resolve step (callee 3, cdecl, one word = the dword at lookup result
/// `+0x18`), or is clear when the first lookup returns null. Then, only when
/// the stored id is 0x100 or 0x108: mode 3 stores 0x112, else mode 5 or a set
/// flag stores 0x110, else mode 4 stores 0x111. No return value.
///
/// Original: 0x00cfd950 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00cfd950(obj: u32, out: u32) -> u32 {
    unsafe {
        const MODE_SLOT: u32 = 0x2B0;
        const LOOKUP_FIELD: u32 = 0x18;
        const RESOLVE_FIELD: u32 = 0x20;
        const PROBE_CALLEE: u32 = 1;
        const LOOKUP_CALLEE: u32 = 2;
        const RESOLVE_CALLEE: u32 = 3;
        let lane = obj.wrapping_add(MODE_SLOT);
        let mode = (lane as *const u32).read_unaligned();
        let probe: u32 = lf_checker_rt::callee_thiscall!(PROBE_CALLEE, u32, lane, 1);
        let flag = if probe & 0xFF != 0 {
            true
        } else {
            let first: u32 = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, lane);
            if first == 0 {
                false
            } else {
                let second: u32 = lf_checker_rt::callee_thiscall!(LOOKUP_CALLEE, u32, lane);
                let arg = (second.wrapping_add(LOOKUP_FIELD) as *const u32).read_unaligned();
                let resolved: u32 = lf_checker_rt::callee_cdecl!(RESOLVE_CALLEE, u32, arg);
                ((resolved.wrapping_add(RESOLVE_FIELD) as *const u32).read_unaligned() >> 13) & 1 != 0
            }
        };
        let cur = (out as *const u32).read_unaligned();
        if cur == 0x100 || cur == 0x108 {
            if mode == 3 {
                (out as *mut u32).write_unaligned(0x112);
            } else if mode == 5 || flag {
                (out as *mut u32).write_unaligned(0x110);
            } else if mode == 4 {
                (out as *mut u32).write_unaligned(0x111);
            }
        }
        0
    }
});
