// original: 0x009FD1C0 frag_typed_dispatch (proposed)

/// Dispatch a frag record by its kind bits, table or matcher path.
///
/// Reads the kind field (bits 6..9) of the dword at `arg + 0x28`. For kinds
/// other than 1 and 8, offers a pointer to the argument slot to the matcher
/// callee on a global object; a zero answer ends the call, otherwise the
/// match callee runs on (`arg`, answer) and its answer is returned. For
/// kinds 1 and 8, indexes a global table by the signed word at `arg + 0x2e`,
/// reads the signed word at `+0x52` of that entry, spills it to the incoming
/// argument slot (as the original does; the stack check is off for this
/// contract because a rewrite cannot address its caller's slot), sets bit 1
/// of the byte that far past a global array base, and returns the base.
///
/// Original: 0x009FD1C0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_009FD1C0(arg: u32) -> u32 {
    unsafe {
        const KIND_OFF: u32 = 0x28;
        const INDEX_OFF: u32 = 0x2E;
        const MATCHER_OBJ: u32 = 0x012BCC94;
        const TABLE: u32 = 0x01295CD8;
        const ENTRY_OFF: u32 = 0x52;
        const ARRAY_BASE: u32 = 0x016EC7B4;
        const SET_BIT: u8 = 2;
        let kind = (((arg + KIND_OFF) as *const u32).read_unaligned() >> 6) & 0xF;
        if kind != 1 && kind != 8 {
            let mut slot = arg;
            let r = lf_checker_rt::callee_thiscall!(1, u32,
                lf_checker_rt::relocated(MATCHER_OBJ), &mut slot as *mut u32 as u32);
            if r == 0 {
                return 0;
            }
            return lf_checker_rt::callee_thiscall!(2, u32, arg, r);
        }
        let idx = ((arg + INDEX_OFF) as *const i16).read_unaligned() as i32;
        let entry = ((lf_checker_rt::relocated(TABLE) as *const u32)
            .offset(idx as isize))
        .read_unaligned();
        let w = ((entry + ENTRY_OFF) as *const i16).read_unaligned() as i32;
        let base = (lf_checker_rt::relocated(ARRAY_BASE) as *const u32).read_unaligned();
        let cell = (base.wrapping_add(w as u32)) as *mut u8;
        cell.write(cell.read() | SET_BIT);
        base
    }
});
