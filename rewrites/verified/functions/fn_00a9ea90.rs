// original: 0x00a9ea90 stream_slot_construct (proposed)

/// Initialise a fresh slot object and stamp its type tag.
///
/// `obj` points to uninitialised storage and `mode` selects the
/// construction mode. The initialiser is called with the object pointer in
/// ECX and stack arguments `(0, mode, 0, 0)`; it fills the body. The header
/// word at `+0x0c` is then stamped with the slot type tag (file address
/// 0x0062e7a0, relocated at run time) and `obj` itself is returned.
///
/// Original: 0x00a9ea90 (cdecl, two stack words).
lf_checker_rt::export!(cdecl, rw_00a9ea90(obj: u32, mode: u32) -> u32 {
    unsafe {
        const TYPE_TAG: u32 = 0x0062e7a0;
        const TAG_OFF: u32 = 0x0c;
        const INIT: u32 = 1;
        lf_checker_rt::callee_thiscall!(INIT, u32, obj, 0, mode, 0, 0);
        ((obj + TAG_OFF) as *mut u32).write_unaligned(lf_checker_rt::relocated(TYPE_TAG));
        obj
    }
});
