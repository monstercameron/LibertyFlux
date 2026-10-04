// original: 0x009FF7F0 frag_buffer_release (proposed)

/// Release `obj`'s buffer at `+0x30` and clear the slot.
///
/// Passes the dword at `+0x30` to the shared free callee, then writes zero
/// back to the slot. Returns whatever the free callee returned.
///
/// Original: 0x009FF7F0 (thiscall, `obj` in `ecx`, no stack words).
lf_checker_rt::export!(thiscall, rw_009FF7F0(obj: u32) -> u32 {
    unsafe {
        const BUF_OFF: u32 = 0x30;
        let p = ((obj + BUF_OFF) as *const u32).read_unaligned();
        let r = lf_checker_rt::callee_cdecl!(1, u32, p);
        ((obj + BUF_OFF) as *mut u32).write_unaligned(0);
        r
    }
});
