// original: 0x009B6510 script_get_position_action_for_token

/// Return the selected script position action when it matches a token. The
/// stdcall routine looks up selector 9 through the manager; a null record
/// returns null. A zero stack token accepts the record without a virtual call.
/// For a nonzero token, the record's vtable slot `+0x0c` is called with the
/// record in ECX, and the record pointer is returned only when the full EAX
/// result equals the token.
lf_checker_rt::export!(stdcall, rw_009b6510(expected_token: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e49c;
        const ENTRY_SELECTOR: u32 = 9;
        const SECONDARY_SELECTOR: u32 = 0;
        const RECORD_LOOKUP: u32 = 1;
        const ACTION_CODE: u32 = 2;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        let manager = read_u32(lf_checker_rt::relocated(MANAGER_GLOBAL));
        let record = lf_checker_rt::callee_thiscall!(
            RECORD_LOOKUP,
            u32,
            manager,
            ENTRY_SELECTOR,
            SECONDARY_SELECTOR
        );
        if record == 0 {
            0
        } else if expected_token == 0 {
            record
        } else {
            let action_code = lf_checker_rt::callee_thiscall!(ACTION_CODE, u32, record);
            if action_code == expected_token { record } else { 0 }
        }
    }
});
