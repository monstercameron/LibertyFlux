// original: 0x00a959d0 stream_drop_handle (proposed)

/// Release the handle stored in an object and clear the slot.
///
/// The dword at `this+0x00` is passed to the release routine (callee 1,
/// cdecl/1) and the slot is then set to 0.
///
/// Returns the release routine's answer. Thiscall: object in ecx, no stack
/// words.
lf_checker_rt::export!(thiscall, rw_00a959d0(this: u32) -> u32 {
    unsafe {
        const SLOT: u32 = 0x00;
        const RELEASE_CALLEE: u32 = 1;
        let r = lf_checker_rt::callee_cdecl!(
            RELEASE_CALLEE,
            u32,
            ((this + SLOT) as *const u32).read_unaligned()
        );
        ((this + SLOT) as *mut u32).write_unaligned(0);
        r
    }
});
