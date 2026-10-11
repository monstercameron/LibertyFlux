// original: 0x009B5CE0 script_read_selected_record_values

/// Read the selected script record's two stored values into caller-provided
/// output slots. The manager is loaded from its relocated global pointer and
/// queried with selector 9 and secondary selector 0. If the lookup returns a
/// record, its dwords at `+0x244` and `+0x248` are copied to the two output
/// pointers and the function returns true in AL. A null record leaves both
/// output slots untouched and returns false. The function is stdcall and
/// consumes its two pointer arguments.
lf_checker_rt::export!(stdcall, rw_009b5ce0(first_output: u32, second_output: u32) -> u32 {
    unsafe {
        const MANAGER_GLOBAL: u32 = 0x0103_e49c;
        const SELECTED_RECORD: u32 = 9;
        const SECONDARY_SELECTOR: u32 = 0;
        const FIRST_VALUE: u32 = 0x244;
        const SECOND_VALUE: u32 = 0x248;
        const LOOKUP: u32 = 1;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let manager_slot = lf_checker_rt::relocated(MANAGER_GLOBAL);
        let manager = read_u32(manager_slot);
        let record = lf_checker_rt::callee_thiscall!(
            LOOKUP,
            u32,
            manager,
            SELECTED_RECORD,
            SECONDARY_SELECTOR
        );
        if record == 0 {
            0
        } else {
            let first_value = read_u32(record.wrapping_add(FIRST_VALUE));
            let second_value = read_u32(record.wrapping_add(SECOND_VALUE));
            write_u32(first_output, first_value);
            write_u32(second_output, second_value);
            1
        }
    }
});
