// original: 0x00D11EB0 ped_task_select_status_and_dispatch

/// Select and publish a task code from a task object and its kind.
///
/// The cdecl entry takes nine stack words. The first is a task object with a
/// function table at offset 0; slot `0x128` is queried with the object in ECX.
/// The object also has a task subobject at `0x2b0`. Arguments 2 and 5 are
/// writable dword pointers: argument 5 receives the helper's resolved code,
/// and argument 2 receives the kind/variant mapping. Argument 3 is a task-kind
/// selector, argument 6 is forwarded to the resolver, argument 7 is its state
/// word, and argument 8 supplies a low-byte mode flag. The source reuses
/// incoming argument slots as scratch, so the proof observes their effects
/// through calls and outputs rather than comparing those stack writes.
///
/// The resolver result is translated for kinds 8, 9, 10, and 21 when the
/// resolved-code pointer's low byte is nonzero (`0x22` to `0x23`, `0x26` to
/// `0x27`). A mode value is selected from the object's word at offset 0,
/// the task subobject's query/flag results, and the resolver status. The
/// resulting table index is `kind * 5 + mode`; the table is a relocated
/// read-only game datum. Kinds 11 through 13 also issue a secondary update
/// when their queried object exists. The return is zero unless both output
/// codes are not `u32::MAX`; otherwise it is the boolean result of the code
/// comparison helper. Pointers are assumed readable/writable, as in the
/// original; invalid pointers fault rather than being validated.
lf_checker_rt::export!(cdecl, rw_ped_task_select_status_and_dispatch(
    task_object: u32,
    _arg1: u32,
    mapped_type_out: u32,
    task_kind: u32,
    subtype: u32,
    resolved_code_out: u32,
    option_index: u32,
    state_flags: u32,
    mode_flags: u32
) -> u32 {
    unsafe {
        const VTABLE_TASK_QUERY: u32 = 0x128;
        const TASK_SUBOBJECT: u32 = 0x2B0;
        const TASK_CODE_TABLE: u32 = 0x01053A84;
        const KIND_GROUP_A: [u32; 3] = [0x0B, 0x0C, 0x0D];
        const KIND_GROUP_B: [u32; 3] = [0x08, 0x09, 0x0A];
        const CODE_A: u32 = 0x22;
        const CODE_A_REPLACEMENT: u32 = 0x23;
        const CODE_B: u32 = 0x26;
        const CODE_B_REPLACEMENT: u32 = 0x27;
        const FLAG_WORD_OFFSET: u32 = 0x20;
        const FLAG_BIT: u32 = 1 << 13;

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }

        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }

        let is_group_a = KIND_GROUP_A.contains(&task_kind);
        let is_group_b = KIND_GROUP_B.contains(&task_kind);
        let is_special_kind = task_kind == 0x15;
        let object_vtable = read_u32(task_object);
        let query_address = read_u32(object_vtable.wrapping_add(VTABLE_TASK_QUERY));
        let query: extern "thiscall" fn(u32) -> u8 =
            core::mem::transmute(query_address as usize);
        let query_succeeded = query(task_object) != 0;

        let must_clear_state_byte =
            !(query_succeeded && mode_flags as u8 == 0)
                && (resolved_code_out as u8 == 0)
                && (is_group_a
                    || is_group_b
                    || (is_special_kind != is_group_b));
        let effective_state_flags = if must_clear_state_byte {
            state_flags & !0xFF
        } else {
            state_flags
        };

        let resolved_code = lf_checker_rt::callee_cdecl!(
            2,
            u32,
            task_object,
            option_index,
            resolved_code_out,
            effective_state_flags
        );
        write_u32(resolved_code_out, resolved_code);
        let helper_code = resolved_code;
        if (task_kind == 0x08 || task_kind == 0x09 || task_kind == 0x0A || is_special_kind)
            && resolved_code_out as u8 != 0
        {
            let translated = if helper_code == CODE_A {
                CODE_A_REPLACEMENT
            } else if helper_code == CODE_B {
                CODE_B_REPLACEMENT
            } else {
                helper_code
            };
            write_u32(resolved_code_out, translated);
        }

        let subobject = task_object.wrapping_add(TASK_SUBOBJECT);
        let query_enabled = lf_checker_rt::callee_thiscall!(3, u8, subobject, 1u32) != 0;
        let local_flag = if query_enabled {
            true
        } else {
            let first_related = lf_checker_rt::callee_thiscall!(4, u32, subobject);
            if first_related == 0 {
                false
            } else {
                let related = lf_checker_rt::callee_thiscall!(4, u32, subobject);
                let flags_address = lf_checker_rt::callee_cdecl!(
                    5,
                    u32,
                    read_u32(related.wrapping_add(0x18))
                );
                (read_u32(flags_address.wrapping_add(FLAG_WORD_OFFSET)) & FLAG_BIT) != 0
            }
        };

        let update_status = lf_checker_rt::callee_thiscall!(6, u8, subobject, 1u32);
        let mode = if read_u32(task_object) == 7 {
            3
        } else if local_flag {
            2
        } else if update_status != 0 {
            1
        } else {
            0
        };
        let table_index = task_kind.wrapping_mul(5).wrapping_add(mode);
        let table_entry_address = TASK_CODE_TABLE.wrapping_add(table_index.wrapping_mul(4));
        let mapped_code = lf_checker_rt::global::<u32>(table_entry_address).read_unaligned();
        write_u32(mapped_type_out, mapped_code);

        if KIND_GROUP_A.contains(&task_kind) {
            let third_related = lf_checker_rt::callee_thiscall!(4, u32, subobject);
            if third_related != 0 {
                let _ = lf_checker_rt::callee_cdecl!(
                    7,
                    u32,
                    task_object,
                    mapped_type_out,
                    subtype,
                    effective_state_flags
                );
            }
        }

        let resolved_value = read_u32(resolved_code_out);
        let mapped_value = read_u32(mapped_type_out);
        if resolved_value != u32::MAX && mapped_value != u32::MAX {
            u32::from(lf_checker_rt::callee_cdecl!(8, u32, resolved_value, mapped_value) != 0)
        } else {
            0
        }
    }
});
