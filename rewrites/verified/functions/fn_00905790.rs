// original: 0x00905790 input_handle_live (proposed)
/// Report whether a handle resolves to a live table slot.
///
/// Validates `handle` through the lookup callee; a negative answer (signed
/// `jns` test) means invalid and returns 0. Otherwise returns 1 when the
/// table slot for the returned index is non-null, else 0. Cdecl with one
/// stack word; only the low byte of the return is set.
export!(cdecl, rw_00905790(handle: u32) -> u32 {
    unsafe {
        /// Handle table base (file VA).
        const TABLE: u32 = 0x0118F6F8;
        const LOOKUP_ID: u32 = 1;
        let ans: u32 = callee_cdecl!(LOOKUP_ID, u32, handle);
        // Signed: the original branches on the sign flag (jns).
        if (ans as i32) < 0 {
            return 0;
        }
        let slot = ((relocated(TABLE).wrapping_add(ans.wrapping_mul(4))) as *const u32)
            .read_unaligned();
        if slot != 0 {
            1
        } else {
            0
        }
    }
});
