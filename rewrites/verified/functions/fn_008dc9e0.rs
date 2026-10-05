// original: 0x008DC9E0 draw_command_alloc_checked (proposed)

/// Checked command-block lookup (proposed name, cdecl).
///
/// Looks up the command slot for `size` and stores the handle at
/// `*out`, returning it. A -1 answer means the slot is missing: the
/// global default is noted and (for a non-zero size) registered through
/// the notify call, whose two pointer arguments address the function's
/// own incoming stack slots (compared by pointed-to value, addresses
/// skipped), then a fresh block is allocated and the payload copied.
/// The original reuses its incoming stack slots as scratch, so the
/// stack check is off; the scratched values are verified through the
/// call-time snapshots instead.
///
/// Original: 0x008DC9E0 (cdecl: `size`, `flags`, `out`).
lf_checker_rt::export!(cdecl, rw_008dc9e0(size: u32, flags: u32, out: u32) -> u32 {
    unsafe {
        const ALLOCATOR: u32 = 0x01175C58;
        const NOTIFY_OBJ: u32 = 0x01175C94;
        const DEFAULT_GLOBAL: u32 = 0x01175C74;
        let r = lf_checker_rt::callee_thiscall!(1, u32, lf_checker_rt::relocated(ALLOCATOR), size);
        (out as *mut u32).write(r);
        if r != 0xffffffffu32 {
            return r;
        }
        let g = lf_checker_rt::global::<u32>(DEFAULT_GLOBAL).read();
        if size != 0 {
            let mut slot_size = size;
            let mut slot_def = g;
            lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(NOTIFY_OBJ),
                &mut slot_size as *mut u32 as u32,
                &mut slot_def as *mut u32 as u32);
        }
        let p = lf_checker_rt::callee_thiscall!(3, u32, lf_checker_rt::relocated(ALLOCATOR), 0, flags);
        lf_checker_rt::callee_cdecl!(4, u32, p, size, flags)
    }
});
