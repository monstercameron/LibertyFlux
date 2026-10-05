// original: 0x00884ce0 stream_handle_notify (proposed)
/// Notify a listener about one manager handle.
///
/// Reads the pool index from the half-word table at `this+0x208` (`handle`
/// selects the entry), resolves it to an object through the pool lookup
/// (intercepted callee 1, cdecl, one argument), then reports the object's
/// payload pointer (at `+0x08`), the `detail` argument and four times the
/// object's kind nibble (low 4 bits of the word at `+0x10`) to the notify
/// routine (intercepted callee 2, cdecl, three arguments).
///
/// Original: thiscall, two stack arguments, callee cleans 8, no return value.
lf_checker_rt::export!(thiscall, rw_00884ce0(this: u32, handle: u32, detail: u32) -> u32 {
    unsafe {
        const HANDLE_TABLE: u32 = 0x208;
        const PAYLOAD: u32 = 0x08;
        const KIND_WORD: u32 = 0x10;
        const KIND_MASK: u32 = 0x0f;
        const LOOKUP_CALLEE: u32 = 1;
        const NOTIFY_CALLEE: u32 = 2;
        let index =
            ((this + HANDLE_TABLE + handle.wrapping_mul(2)) as *const u16).read_unaligned() as u32;
        let obj: u32 = lf_checker_rt::callee_cdecl!(LOOKUP_CALLEE, u32, index);
        let kind = ((obj + KIND_WORD) as *const u32).read_unaligned() & KIND_MASK;
        let payload = ((obj + PAYLOAD) as *const u32).read_unaligned();
        let _notified: u32 =
            lf_checker_rt::callee_cdecl!(NOTIFY_CALLEE, u32, payload, detail, kind.wrapping_mul(4));
        0
    }
});
