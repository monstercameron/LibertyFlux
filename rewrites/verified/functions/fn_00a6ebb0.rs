// original: 0x00a6ebb0 task_attach_by_tag_and_slot (proposed)

/// Attaches the task `arg2` to its slot: tag checks select the path, then a
/// vtable hook vetoes stale attachments and one of two submit routines runs.
///
/// The tag bytes at `arg1 + TAG0/TAG1` xored with the key byte at
/// `arg1 + TAGKEY` must clear `0x7f` and set it respectively (a scrambled
/// two-byte magic), else null. When the link word at `arg2 + LINK` is
/// non-null, the slot hook (the virtual at slot `HOOK_SLOT` of `arg2`'s
/// vtable, callee 1, `arg2` in ecx) runs and its answer is compared against
/// the sign-extended half-word at `link + HOOK_REF`: equality means the
/// attachment is stale and yields null.
///
/// Otherwise the cursor routine (callee 2) runs with `arg2 + CURSOR` in
/// ecx. When it returns non-null and the word at its result `+ SLOT_KIND`
/// equals `WANT_KIND`, the manager is fetched (callee 3) and the submit
/// routine (callee 4) runs with the manager in ecx and the words
/// `(link, SUBMIT_TAG)`; its result is returned, or null when the manager
/// is null.
///
/// Otherwise (no cursor or wrong kind): when the guard word at
/// `arg2 + GUARD` is non-zero, null. Else the build routine (callee 5,
/// cdecl, `arg2`) runs; a null answer yields null, else the manager is
/// fetched (callee 3 again) and the alternate submit (callee 6) runs with
/// the words `(built, SUBMIT_TAG)`; its result is returned, or null when
/// the manager is null.
///
/// Original: stdcall, two stack words (`arg1`, `arg2`), callee pops 8.
lf_checker_rt::export!(stdcall, rw_00a6ebb0(arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const TAGKEY: u32 = 0x280c;
        const TAG0: u32 = 0x280e;
        const TAG1: u32 = 0x280f;
        const TAG_LIM: u8 = 0x7f;
        const LINK: u32 = 0x2c4;
        const HOOK_REF: u32 = 0x2e;
        const HOOK_SLOT: u32 = 0x12c;
        const CURSOR: u32 = 0x2b0;
        const SLOT_KIND: u32 = 0x18;
        const WANT_KIND: u32 = 0x2e;
        const GUARD: u32 = 0x2c8;
        const SUBMIT_TAG: u32 = 0x48;
        const MANAGER_ANCHOR: u32 = 0x0167_e2a0;
        const CURSOR_OF: u32 = 2;
        const GET_MANAGER: u32 = 3;
        const SUBMIT: u32 = 4;
        const BUILD: u32 = 5;
        const SUBMIT_ALT: u32 = 6;

        let key = ((arg1 as *const u8).wrapping_byte_offset(TAGKEY as isize)).read();
        let tag0 = ((arg1 as *const u8).wrapping_byte_offset(TAG0 as isize)).read() ^ key;
        if tag0 <= TAG_LIM {
            return 0;
        }
        let tag1 = ((arg1 as *const u8).wrapping_byte_offset(TAG1 as isize)).read() ^ key;
        if tag1 > TAG_LIM {
            return 0;
        }
        let link = ((arg2 as *const u32).wrapping_byte_offset(LINK as isize)).read_unaligned();
        if link != 0 {
            let reference = ((link as *const u16).wrapping_byte_offset(HOOK_REF as isize))
                .read_unaligned() as i16 as i32;
            let vtable = (arg2 as *const u32).read_unaligned();
            let target = ((vtable as *const u32).wrapping_byte_offset(HOOK_SLOT as isize))
                .read_unaligned();
            let hook: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(target as usize);
            let seen = hook(arg2) as i32;
            if reference == seen {
                return 0;
            }
        }
        let cursor: u32 = lf_checker_rt::callee_thiscall!(CURSOR_OF, u32, arg2.wrapping_add(CURSOR));
        let direct = cursor != 0
            && ((cursor as *const u32).wrapping_byte_offset(SLOT_KIND as isize)).read_unaligned()
                == WANT_KIND;
        let anchor = (lf_checker_rt::relocated(MANAGER_ANCHOR) as *const u32).read_unaligned();
        if direct {
            let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
            if mgr == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(SUBMIT, u32, mgr, link, SUBMIT_TAG);
        }
        let guard = ((arg2 as *const u32).wrapping_byte_offset(GUARD as isize)).read_unaligned();
        if guard != 0 {
            return 0;
        }
        let built: u32 = lf_checker_rt::callee_cdecl!(BUILD, u32, arg2);
        if built == 0 {
            return 0;
        }
        let mgr: u32 = lf_checker_rt::callee_thiscall!(GET_MANAGER, u32, anchor);
        if mgr == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SUBMIT_ALT, u32, mgr, built, SUBMIT_TAG)
    }
});
