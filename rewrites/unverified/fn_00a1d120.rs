// original: 0x00a1d120 cam_lookup_publish_pair (proposed)

/// Looks a handle up twice and publishes it through two out-pointers.
///
/// `a1` selects the subject (null returns at once), `a0` points at a
/// record with a linked state pointer at `+LINK_OFF` (a record whose byte
/// at `+LINK_FLAG_OFF` is set vetoes the run), and `a2`/`a3` are
/// out-pointers. The predicate callee must accept `a1` and the lookup
/// callee on `a1` must return nonzero (it is called twice; the second
/// answer's word at `+HANDLE_OFF` is the handle). The handle is stored to
/// `a2`; when it is nonzero and the confirm callee accepts it, it is also
/// stored to `a3`. Returns nothing.
///
/// Original: 0x00a1d120 (stdcall, four stack words).
lf_checker_rt::export!(stdcall, rw_00a1d120(a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const C_PRED: u32 = 1;
        const C_LOOKUP: u32 = 2;
        const C_CONFIRM: u32 = 3;
        const LINK_OFF: u32 = 0x6c;
        const LINK_FLAG_OFF: u32 = 0xe;
        const HANDLE_OFF: u32 = 0x64;
        if a1 == 0 {
            return 0;
        }
        let pred: u32 = lf_checker_rt::callee_thiscall!(C_PRED, u32, a1);
        if pred as u8 == 0 {
            return 0;
        }
        let link = ((a0 + LINK_OFF) as *const u32).read_unaligned();
        if link != 0 && ((link + LINK_FLAG_OFF) as *const u8).read() != 0 {
            return 0;
        }
        let t = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, a1);
        if t == 0 {
            return 0;
        }
        let u = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, a1);
        let h = ((u + HANDLE_OFF) as *const u32).read_unaligned();
        (a2 as *mut u32).write_unaligned(h);
        if h == 0 {
            return 0;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(C_CONFIRM, u32, h);
        if ok as u8 == 0 {
            return 0;
        }
        (a3 as *mut u32).write_unaligned(h);
        0
    }
});
