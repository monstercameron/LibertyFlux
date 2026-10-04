// original: 0x00E510B0 find_matching_entry
/// Enumerate-and-match lookup: opens an enumeration on the manager object,
/// walks its entries, and reports whether any entry both passes validation
/// and matches the caller's key.
///
/// Behaviour: notify the subsystem (callee 1), fetch the manager (callee 2),
/// open the enumeration (slot 0x58; a handle of -1 fails at once). Each pass
/// fetches the current entry text into a stack buffer (callee 4), validates
/// it (callee 5), and skips entries whose text is 0x20 bytes or longer.
/// For a short valid entry it creates a helper (callee 6), initialises the
/// helper from the entry text (callee 7), tests the helper against `key`
/// (callee 8), releases the helper (callee 9) and, on a match, latches the
/// found flag and stops: a match closes the enumeration at once instead of
/// advancing. Otherwise advancing (slot 0x5c) repeats the pass until it
/// reports done, then the enumeration is closed (slot 0x60). Returns 1 when
/// a match latched.
///
/// The entry-text length feeds only the `< 0x20` test, so the scan stops at
/// 0x20: lengths below agree exactly and anything longer takes the same
/// branch on both sides. The stack-cookie prologue/epilogue of the original
/// is compiler scaffolding with no observable effect and is not mirrored.
lf_checker_rt::export!(stdcall, rb93_fn1(key: u32) -> u32 {
    const CAL_NOTIFY: u32 = 1; // subsystem notify (cdecl/2, result ignored)
    const CAL_MANAGER: u32 = 2; // manager fetch (cdecl/2)
    const CAL_OPEN: u32 = 3; // enumeration open, vtable slot 0x58 (thiscall/2)
    const CAL_FETCH: u32 = 4; // entry-text fetch into the buffer (cdecl/1)
    const CAL_VALIDATE: u32 = 5; // entry validation (cdecl/2)
    const CAL_CREATE: u32 = 6; // helper create (cdecl/0)
    const CAL_INIT: u32 = 7; // helper init from entry text (thiscall/1)
    const CAL_MATCH: u32 = 8; // helper match test against the key (thiscall/1)
    const CAL_RELEASE: u32 = 9; // helper release (cdecl/1)
    const CAL_NEXT: u32 = 10; // advance enumeration, vtable slot 0x5c (thiscall/2)
    const CAL_CLOSE: u32 = 11; // enumeration close, vtable slot 0x60 (thiscall/1)

    const NOTIFY_ARG_VA: u32 = 0x00F1A2C0;
    const QUERY_ARG_VA: u32 = 0x01168DD8;
    const VALIDATE_ARG_VA: u32 = 0x00F1A2CC;
    const MAX_TEXT: usize = 0x20;

    unsafe {
        lf_checker_rt::callee_cdecl!(
            CAL_NOTIFY,
            u32,
            lf_checker_rt::relocated(NOTIFY_ARG_VA),
            0
        );
        let mgr = lf_checker_rt::callee_cdecl!(
            CAL_MANAGER,
            u32,
            lf_checker_rt::relocated(QUERY_ARG_VA),
            1
        );
        // The original's buffer is uninitialized stack; the contract defines
        // the fill as zero, so an explicit zero block matches on both sides.
        let mut buf = [0u8; 64];
        let vt = *(mgr as *const u32);
        let open_addr = *((vt.wrapping_add(0x58)) as *const u32);
        let f_open: extern "thiscall" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(open_addr as usize);
        let handle = f_open(
            mgr,
            lf_checker_rt::relocated(QUERY_ARG_VA),
            buf.as_mut_ptr() as u32,
        );
        if handle == 0xFFFF_FFFF {
            return 0;
        }
        let mut found: u32 = 0;
        loop {
            let tok = lf_checker_rt::callee_cdecl!(CAL_FETCH, u32, buf.as_mut_ptr() as u32);
            let ok = lf_checker_rt::callee_cdecl!(
                CAL_VALIDATE,
                u32,
                tok,
                lf_checker_rt::relocated(VALIDATE_ARG_VA)
            );
            if ok != 0 {
                let mut len: usize = 0;
                while len < MAX_TEXT && buf[len] != 0 {
                    len += 1;
                }
                if len < MAX_TEXT {
                    let helper = lf_checker_rt::callee_cdecl!(CAL_CREATE, u32,);
                    lf_checker_rt::callee_thiscall!(CAL_INIT, u32, helper, buf.as_ptr() as u32);
                    let m = lf_checker_rt::callee_thiscall!(CAL_MATCH, u32, helper, key);
                    lf_checker_rt::callee_cdecl!(CAL_RELEASE, u32, helper);
                    if m != 0 {
                        found = 1;
                        break;
                    }
                }
            }
            let next_addr = *((vt.wrapping_add(0x5C)) as *const u32);
            let f_next: extern "thiscall" fn(u32, u32, u32) -> u32 =
                core::mem::transmute(next_addr as usize);
            // The original tests only AL; mask so high-byte leftovers agree.
            let more = f_next(mgr, handle, buf.as_mut_ptr() as u32) & 0xFF;
            if more == 0 {
                break;
            }
        }
        let close_addr = *((vt.wrapping_add(0x60)) as *const u32);
        let f_close: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(close_addr as usize);
        f_close(mgr, handle);
        found
    }
});
