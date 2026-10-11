// original: 0x00C9D800 task_event_gate_flags

/// Set task event flags from three table driven category checks.
///
/// `this` is a task object with a controller pointer at `+0x40` and a flag
/// word at `+0x30`. The controller has a signed table index at `+0x2e`, a
/// query vtable at `+0`, and a fallback result at `+0x100`. Each category
/// lookup selects a controller from the relocated dispatch table, calls its
/// `+0x38` method with one of the category keys, then resolves the selected
/// record through either the controller query method at `+0xa0` (and its
/// result's `+0xe0` method) or the fallback pointer. A selected record matches
/// when its 16-bit field at `+0x16` equals the category's expected code.
/// Matches for the first two categories set bit `0x40`; a match for the third
/// sets bit `0x80`. The operation only sets bits and preserves all other flag
/// bits. The function is a thiscall with no stack arguments; its return
/// register is not part of the observable contract.
lf_checker_rt::export!(thiscall, rw_00c9d800(this: u32) -> u32 {
    unsafe {
        const CONTROLLER_PTR: u32 = 0x40;
        const CONTROLLER_INDEX: u32 = 0x2e;
        const CONTROLLER_VTABLE: u32 = 0x00;
        const CONTROLLER_FALLBACK: u32 = 0x100;
        const TASK_FLAGS: u32 = 0x30;
        const DISPATCH_TABLE_VA: u32 = 0x1295_cd8;
        const DISPATCH_METHOD: u32 = 0x38;
        const QUERY_METHOD: u32 = 0xa0;
        const FOLLOW_METHOD: u32 = 0xe0;
        const RECORD_STRIDE: u32 = 0xe0;
        const RECORDS_PTR: u32 = 0x10;
        const RECORD_CODE: u32 = 0x16;
        const NO_RECORD: u32 = 0xff;
        const CATEGORY_KEYS: [u32; 3] = [0x21, 0x22, 0x23];
        const EXPECTED_CODES: [u16; 3] = [0x36a1, 0x36a1, 0x04b3];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }

        let controller = read_u32(this.wrapping_add(CONTROLLER_PTR));
        let signed_table_index = read_u16(controller.wrapping_add(CONTROLLER_INDEX)) as i16 as i32;
        let dispatch_table = lf_checker_rt::relocated(DISPATCH_TABLE_VA);
        let dispatch_entry = read_u32(
            dispatch_table.wrapping_add((signed_table_index as u32).wrapping_mul(4)),
        );
        let dispatch_vtable = read_u32(dispatch_entry.wrapping_add(CONTROLLER_VTABLE));
        let dispatch_address = read_u32(dispatch_vtable.wrapping_add(DISPATCH_METHOD));
        let dispatch: unsafe extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(dispatch_address as usize);

        let controller_vtable = read_u32(controller.wrapping_add(CONTROLLER_VTABLE));
        let query_address = read_u32(controller_vtable.wrapping_add(QUERY_METHOD));
        let query: unsafe extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(query_address as usize);

        let mut matches = [false; 3];
        for index in 0..CATEGORY_KEYS.len() {
            let record_index = dispatch(dispatch_entry, CATEGORY_KEYS[index]);
            let mut category_matches = false;
            if record_index != NO_RECORD {
                let first_query = query(controller);
                let tail = if first_query != 0 {
                    let query_result = query(controller);
                    let query_vtable = read_u32(query_result.wrapping_add(CONTROLLER_VTABLE));
                    let follow_address = read_u32(query_vtable.wrapping_add(FOLLOW_METHOD));
                    let follow: unsafe extern "thiscall" fn(u32) -> u32 =
                        core::mem::transmute(follow_address as usize);
                    follow(query_result)
                } else {
                    read_u32(controller.wrapping_add(CONTROLLER_FALLBACK))
                };

                let record_table_holder = read_u32(tail.wrapping_add(4));
                let record_array = read_u32(record_table_holder);
                let record_address = read_u32(
                    record_array
                        .wrapping_add(record_index.wrapping_mul(RECORD_STRIDE))
                        .wrapping_add(RECORDS_PTR),
                );
                category_matches = read_u16(record_address.wrapping_add(RECORD_CODE))
                    == EXPECTED_CODES[index];
            }

            matches[index] = category_matches;
        }

        let flags_address = this.wrapping_add(TASK_FLAGS);
        if matches[0] && matches[1] {
            let flags = read_u32(flags_address);
            (flags_address as *mut u32).write_unaligned(flags | 0x40);
        }
        if matches[2] {
            let flags = read_u32(flags_address);
            (flags_address as *mut u32).write_unaligned(flags | 0x80);
        }
        0
    }
});
