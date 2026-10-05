// original: 0x00a35650 vehicle_tag_dispatch (proposed)

/// Dispatch through a tagged pointer, passing -1 tags straight through.
///
/// Loads `p = *pp`; a `p` of -1 is returned unchanged with no call.
/// Otherwise tail-calls the handler (id 1, one argument) with `p` and
/// returns its answer. Cdecl/1, returns EAX.
lf_checker_rt::export!(cdecl, rw_00a35650(pp: u32) -> u32 {
    unsafe {
        const HANDLER: u32 = 1;
        const ABSENT: u32 = 0xFFFF_FFFF;
        let p = core::ptr::read_unaligned(pp as *const u32);
        if p == ABSENT {
            p
        } else {
            lf_checker_rt::callee_cdecl!(HANDLER, u32, p)
        }
    }
});
