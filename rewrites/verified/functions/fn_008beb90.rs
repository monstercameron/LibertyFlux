// original: 0x008BEB90 menu_call_forward_2 (proposed)

/// Forward two arguments to the indexed menu handler.
///
/// The first stack word indexes the global handler table; the call target is
/// the table entry, invoked with (table[idx], second). Returns the handler's
/// answer (cdecl, two stack words; the index is a signed dword, unbounded in
/// the original).
lf_checker_rt::export!(cdecl, rw_008BEB90(idx: u32, a: u32) -> u32 {
    unsafe {
        /// Global table of handler pointers, indexed by the first argument.
        const HANDLER_TABLE: u32 = 0x01160C0C;
        /// Id of the handler callee in the contract.
        const HANDLER: u32 = 1;
        let entry = (lf_checker_rt::relocated(
            HANDLER_TABLE.wrapping_add(idx.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        lf_checker_rt::callee_cdecl!(HANDLER, u32, entry, a)
    }
});
