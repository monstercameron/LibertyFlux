// original: 0x009bbaf0 reset_input_dispatch (proposed)

/// Reset the input dispatch state and tail into the slot-table initialiser.
///
/// Copies the global word at 0x0103ABB0 into the object pointed to by the
/// global at 0x011A28F4 (offset `+0x260`), clears the two state words
/// through helper id 1 (cdecl, no arguments), re-initialises the dispatch
/// record at 0x0128E94C through helper id 2 (thiscall, no stack arguments),
/// then tail-jumps to the slot-table initialiser (id 3, no arguments) and
/// returns its answer.
///
/// Edge cases: none take another path; the pointer is dereferenced
/// unconditionally, so a bad one faults on both sides alike.
///
/// Original: no arguments, two direct callees plus a tail call.
lf_checker_rt::export!(cdecl, rw_009bbaf0() -> u32 {
    unsafe {
        const OWNER_PTR: u32 = 0x011a28f4;
        const VALUE: u32 = 0x0103abb0;
        const CELL_OFF: u32 = 0x260;
        const DISPATCH: u32 = 0x0128e94c;
        let owner = (lf_checker_rt::global::<u32>(OWNER_PTR) as *const u32).read_unaligned();
        let v = (lf_checker_rt::global::<u32>(VALUE) as *const u32).read_unaligned();
        ((owner + CELL_OFF) as *mut u32).write_unaligned(v);
        let _: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, lf_checker_rt::relocated(DISPATCH),);
        lf_checker_rt::callee_cdecl!(3, u32,)
    }
});
