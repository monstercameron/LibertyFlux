// original: 0x00935C20 apply_network_state_transition

/// Apply the transition for one state-table entry. The receiver stores the
/// table pointer at `+0x14c`; entries use a `0x168` stride. Only entries with
/// a nonzero byte at `+0x164` are processed. The byte at `+0x165` selects
/// either the fixed transition data or the entry's pointer at `+0x144` for
/// the first helper. The entry is deactivated before the helper result is
/// tested; only its low return byte is meaningful. A nonzero result dispatches
/// status `2` through the pointer at `+0x154` or status `4` through `+0x15c`;
/// status `3` and all other values stop after deactivation. This is a
/// thiscall routine with one stack index and no defined return value.
lf_checker_rt::export!(thiscall, rw_00935c20(this: u32, state_index: u32) -> () {
    unsafe {
        const STATE_TABLE_POINTER: u32 = 0x14c;
        const STATE_STRIDE: u32 = 0x168;
        const ACTIVE_FLAG: u32 = 0x164;
        const ROUTE_FLAG: u32 = 0x165;
        const ENTRY_DATA: u32 = 0x144;
        const STATUS: u32 = 0x158;
        const STATUS_TWO_OBJECT: u32 = 0x154;
        const STATUS_FOUR_OBJECT: u32 = 0x15c;
        const FIXED_TRANSITION_DATA: u32 = 0x00e8_77fc;

        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }

        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        let state_table = read_u32(this.wrapping_add(STATE_TABLE_POINTER));
        let entry = state_table.wrapping_add(state_index.wrapping_mul(STATE_STRIDE));
        if read_u8(entry.wrapping_add(ACTIVE_FLAG)) == 0 {
            return;
        }

        let transition_data = if read_u8(entry.wrapping_add(ROUTE_FLAG)) != 0 {
            entry.wrapping_add(ENTRY_DATA)
        } else {
            lf_checker_rt::relocated(FIXED_TRANSITION_DATA)
        };
        write_u8(entry.wrapping_add(ACTIVE_FLAG), 0);
        let accepted = lf_checker_rt::callee_cdecl!(1, u32, transition_data);
        if (accepted as u8) == 0 {
            return;
        }

        match read_u32(entry.wrapping_add(STATUS)) {
            2 => {
                let object = read_u32(entry.wrapping_add(STATUS_TWO_OBJECT));
                let _ = lf_checker_rt::callee_thiscall!(2, u32, object);
            }
            4 => {
                let object = read_u32(entry.wrapping_add(STATUS_FOUR_OBJECT));
                let _ = lf_checker_rt::callee_thiscall!(2, u32, object);
            }
            _ => {}
        }
    }
});
