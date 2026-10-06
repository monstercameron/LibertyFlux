// original: 0x008BEB10 menu_call_forward_4 (proposed)

/// Forward four arguments to the indexed menu handler.
///
/// The first stack word indexes the global handler table; the call target is
/// the table entry, invoked with (table[idx], second, third, fourth) where
/// the remaining three stack words pass through in order. Returns the
/// handler's answer (cdecl, four stack words; the index is a signed dword,
/// unbounded in the original).
lf_checker_rt::export!(cdecl, rw_008BEB10(idx: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        /// Global table of handler pointers, indexed by the first argument.
        const HANDLER_TABLE: u32 = 0x01160C0C;
        /// Id of the handler callee in the contract.
        const HANDLER: u32 = 1;
        let entry = (lf_checker_rt::relocated(
            HANDLER_TABLE.wrapping_add(idx.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        lf_checker_rt::callee_cdecl!(HANDLER, u32, entry, a, b, c)
    }
});
