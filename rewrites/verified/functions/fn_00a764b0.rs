// original: 0x00A764B0 vehicle_model_helper_lookup

/// When the record's field at `+0x1C` has bit one set, ask the helper to look
/// up the requested value and write one dword through a temporary pointer.
/// Return the temporary's low word when the helper's AL result is nonzero, or
/// `u32::MAX` when the guard or helper fails. The function is cdecl with two
/// stack arguments; the temporary's address is only used during the call.
lf_checker_rt::export!(cdecl, rw_00a764b0(record: u32, requested_value: u32) -> u32 {
    const FLAGS_OFFSET: u32 = 0x1C;
    const LOOKUP_CALLEE: u32 = 1;

    unsafe {
        let flags = (record.wrapping_add(FLAGS_OFFSET) as *const u32).read_unaligned();
        if flags & 0x02 == 0 {
            return u32::MAX;
        }

        let mut helper_output = 0u32;
        let helper_ok = lf_checker_rt::callee_stdcall!(
            LOOKUP_CALLEE,
            u8,
            requested_value,
            &mut helper_output as *mut u32 as u32,
        );
        if helper_ok == 0 {
            return u32::MAX;
        }
        u32::from(helper_output as u16)
    }
});
