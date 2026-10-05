// original: 0x008bb780 files_memory_indexed_dispatch_stub (proposed)

/// Look one entry up in a global slot table, hand it to a notifier, then
/// tail-jump to a shared worker routine.
///
/// `slot` (the single stack word, read unsigned) selects a dword from the
/// global `SLOT_TABLE`: the original scales it by four and loads one word,
/// with no bounds check and no comparison, so every 32-bit index simply
/// addresses `table + 4 * slot` with wrapping arithmetic. The loaded entry
/// is passed as the one argument of the cdecl notifier at callee 1 (whose
/// return value is ignored), and control then jumps to the shared routine
/// at callee 2 with the stack untouched. The shared routine takes no stack
/// arguments of its own (its body never reads past its frame, and it ends
/// in a plain return that pops nothing), so the caller's word stays on the
/// stack for the caller to clean, exactly as the direct callers do.
///
/// The only memory this function touches itself is the one table word it
/// loads; it writes nothing. Real callers pass small indices (0 and 5 were
/// observed). There are no comparisons, so no signedness question; the only
/// edge is a wild index, which reads whatever dword sits at the scaled
/// address on both sides alike.
///
/// Original: 0x008bb780 (cdecl, one stack word; 24 bytes: load, call, jump).
lf_checker_rt::export!(cdecl, rw_008bb780(slot: u32) -> u32 {
    unsafe {
        const SLOT_TABLE: u32 = 0x0116_0C0C;
        const NOTIFIER: u32 = 1;
        const TAIL_ROUTINE: u32 = 2;
        let entry = ((lf_checker_rt::relocated(SLOT_TABLE).wrapping_add(slot.wrapping_mul(4)))
            as *const u32)
            .read_unaligned();
        let _: u32 = lf_checker_rt::callee_cdecl!(NOTIFIER, u32, entry);
        lf_checker_rt::callee_cdecl!(TAIL_ROUTINE, u32,)
    }
});
