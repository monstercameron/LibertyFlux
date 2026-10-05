// original: 0x00A8B1D0 pool_row_hook_dispatch (proposed)

/// Dispatch through a pool row's hook with an adjusted index.
///
/// The global table picks a byte by `3 * s`; `0xFF` means no row and
/// returns 0. Otherwise the row at `this + 100 * byte` supplies the hook
/// pointer (`+0x28`, called as cdecl with the adjusted index `s` minus the
/// row's `+0x58` first and `arg1` second) and its result is returned.
///
/// Original: thiscall, two stack words, returns u32 in EAX. One outgoing
/// call through a register (cdecl, two stack words), intercepted by a
/// planted stub address.
lf_checker_rt::export!(thiscall, rw_00A8B1D0(this: u32, s: u32, arg1: u32) -> u32 {
    unsafe {
        const GLOBAL_TABLE: u32 = 0x103e8d0;
        const BYTE_BASE: u32 = 0x17;
        const BYTE_STRIDE: u32 = 24;
        const ROW_STRIDE: u32 = 100;
        const HOOK_OFF: u32 = 0x28;
        const ADJ_OFF: u32 = 0x58;
        const ABSENT: u8 = 0xff;
        let table = (lf_checker_rt::global::<u32>(GLOBAL_TABLE) as *const u32)
            .read_unaligned();
        let b = (table.wrapping_add(s.wrapping_mul(3).wrapping_mul(8)).wrapping_add(BYTE_BASE)
            as *const u8)
            .read();
        if b == ABSENT {
            return 0;
        }
        let row = this.wrapping_add((b as u32).wrapping_mul(ROW_STRIDE));
        let d = s.wrapping_sub(((row + ADJ_OFF) as *const u32).read_unaligned());
        let hook = ((row + HOOK_OFF) as *const u32).read_unaligned();
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(hook as usize);
        f(d, arg1)
    }
});
