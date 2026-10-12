// original: 0x008B9590 menu_slot_get_state_byte

/// Fetch the state byte of one menu slot through its record pointer.
///
/// Loads the record pointer at `TABLE + index * 4` and tail-calls the
/// record reader with it, returning its result. Cdecl, index on the stack.
lf_checker_rt::export!(cdecl, rw_008B9590(index: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0116_0C0C;
        let record = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
            as *const u32)
            .read_unaligned();
        lf_checker_rt::callee_cdecl!(1, u32, record)
    }
});
