// original: 0x00cfde50 task_collect_slot_objects (proposed)

/// Collects up to `max` accepted neighbour objects into `out`, always
/// storing `obj` itself first; returns the stored count.
///
/// `out` is a dword array, `obj` the task-side record, `max` the capacity.
/// The head lookup (callee 1, thiscall, no stack args) runs with ECX = the
/// container pointer at `obj + 0x224`. `obj` is stored at `out[0]` and the
/// count starts at 1. Up to 16 slots at container `+0x168 + i * 4` are then
/// scanned while the count is below `max`: a slot is skipped when null, when
/// the byte at candidate `+0x211` is nonzero, or when the candidate equals
/// `obj` or the head result. Otherwise the accept test (callee 2, thiscall,
/// one stack arg = candidate) runs with ECX = container, and a nonzero low
/// byte stores the candidate at `out[count]` and bumps the count.
///
/// Original: 0x00cfde50 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_00cfde50(out: u32, obj: u32, max: u32) -> u32 {
    unsafe {
        const CONTAINER_SLOT: u32 = 0x224;
        const SLOTS: u32 = 0x168;
        const SLOT_COUNT: u32 = 16;
        const CAND_FLAG: u32 = 0x211;
        const HEAD_CALLEE: u32 = 1;
        const ACCEPT_CALLEE: u32 = 2;
        let container = (obj.wrapping_add(CONTAINER_SLOT) as *const u32).read_unaligned();
        let head: u32 = lf_checker_rt::callee_thiscall!(HEAD_CALLEE, u32, container);
        (out as *mut u32).write_unaligned(obj);
        let mut count = 1u32;
        let mut i = 0u32;
        loop {
            let cand = (container.wrapping_add(SLOTS + i * 4) as *const u32).read_unaligned();
            if cand != 0
                && (cand.wrapping_add(CAND_FLAG) as *const u8).read() == 0
                && cand != obj
                && cand != head
            {
                let ok: u32 = lf_checker_rt::callee_thiscall!(ACCEPT_CALLEE, u32, container, cand);
                if ok & 0xFF != 0 {
                    (out.wrapping_add(count * 4) as *mut u32).write_unaligned(cand);
                    count += 1;
                }
            }
            if count >= max {
                break;
            }
            i += 1;
            if i >= SLOT_COUNT {
                break;
            }
        }
        count
    }
});
