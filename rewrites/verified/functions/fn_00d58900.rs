// original: 0x00d58900 ccam_view_seq_child_check
/// Walk the child list at `[this+0x124]`; every child whose kind query
/// answers `WANT_KIND` must also pass the readiness probe.
///
/// Follows the `NEXT` (+0x11c) links from the head child. For each child it
/// calls the kind query (the virtual slot at `+0x28`, intercepted callee 1)
/// and, only when the answer equals `WANT_KIND` (0x22), the readiness probe
/// (intercepted callee 2); a probe answering zero fails the whole walk.
/// Returns 1 when every probed child passed (an empty list passes), else 0.
///
/// Original: thiscall, no stack arguments, returns `al`.
lf_checker_rt::export!(thiscall, rw_00d58900 (this: u32) -> u32 {
    unsafe {
        const HEAD: u32 = 0x124;
        const NEXT: u32 = 0x11c;
        const KIND_SLOT: u32 = 0x28;
        const WANT_KIND: u32 = 0x22;
        const PROBE: u32 = 2;
        let mut child = ((this + HEAD) as *const u32).read_unaligned();
        while child != 0 {
            let vtable = (child as *const u32).read_unaligned();
            let kind_of: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vtable + KIND_SLOT) as *const u32).read_unaligned() as usize);
            if kind_of(child) == WANT_KIND {
                let ok: u32 = lf_checker_rt::callee_thiscall!(PROBE, u32, child);
                if ok & 0xFF == 0 {
                    return 0;
                }
            }
            child = ((child + NEXT) as *const u32).read_unaligned();
        }
        1
    }
});
