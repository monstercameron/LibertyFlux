// original: 0x009B64D0 script_match_position_action

/// Check whether the selected script position action matches a stack token.
/// The stdcall routine looks up selector 27 through the manager. A null record
/// returns false. A zero token accepts any non-null record without a virtual
/// call; otherwise, virtual slot `+0x0c` is called with the record in ECX and
/// its full EAX result is compared for equality with the token. Equality
/// returns one in AL, and a mismatch returns zero.
lf_checker_rt::export!(stdcall, rw_009b64d0(expected_token: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e49c;
        const ENTRY_SELECTOR: u32 = 27;
        const SECONDARY_SELECTOR: u32 = 0;
        const RECORD_LOOKUP: u32 = 1;
        const MATCH_ACTION: u32 = 2;

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
            1
        } else {
            let action_code = lf_checker_rt::callee_thiscall!(MATCH_ACTION, u32, record);
            u32::from(action_code == expected_token)
        }
    }
});
