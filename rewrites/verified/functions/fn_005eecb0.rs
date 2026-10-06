// original: 0x005eecb0 input_ui_factory_0c

/// Allocate and initialise a small callback holder, then hand it to callee 2.
///
/// Calls callee 1 (`new`-like, cdecl) for a `0xc`-byte block. When it returns
/// null, callee 2 is called with null. Otherwise the block receives the holder's
/// vtable pointer at `+0x00`, a masked link word at `+0x04` (the old word xored
/// with the low 14 bits of itself xored with the global callback counter, which
/// is then incremented) and the word `arg0` at `+0x08`, and callee 2 is called with the block.
/// Returns callee 2's answer.
///
/// Original: cdecl, 1 stack argument, two direct callees, caller cleans up.
lf_checker_rt::export!(cdecl, rw_005eecb0(arg0: u32) -> u32 {
    unsafe {
        const NEW_SIZE: u32 = 0xC;
        const VTABLE: u32 = 0xE7E080;
        const LINK: u32 = 0x04;
        const A0: u32 = 0x08;
        const COUNTER: u32 = 0x010327A0;
        const LINK_MASK: u32 = 0x3FFF;
        let obj = lf_checker_rt::callee_cdecl!(1, u32, NEW_SIZE, 0);
        if obj == 0 {
            return lf_checker_rt::callee_cdecl!(2, u32, 0);
        }
        let counter = (lf_checker_rt::global::<u32>(COUNTER)).read_unaligned();
        let link = ((obj + LINK) as *const u32).read_unaligned();
        ((obj + A0) as *mut u32).write_unaligned(arg0);
        ((obj + LINK) as *mut u32).write_unaligned(link ^ ((link ^ counter) & LINK_MASK));
        (obj as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));

        (lf_checker_rt::global::<u32>(COUNTER)).write_unaligned(counter.wrapping_add(1));
        lf_checker_rt::callee_cdecl!(2, u32, obj)
    }
});
