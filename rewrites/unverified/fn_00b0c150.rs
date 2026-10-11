// original: 0x00b0c150 net_pool_slot_alloc
/// Allocate and publish one row in the network pickup pool.
///
/// Chooses a free row from the mode's pool window, or reclaims a row in a
/// reusable state when the free scan finds none. It advances the appropriate
/// cursor, resets the selected row through its helper, copies two caller
/// records into the row, updates the row's mode and flag bytes, and attaches
/// the optional owner object. Special mode obtains and stores a callback
/// token; the event-enabled path creates and fills the notification object.
/// It returns the slot sequence value, or `u32::MAX` when no eligible row
/// exists or the initial gate declines the request.
///
/// The exported ABI consumes thirteen cdecl stack words. Pointers are kept as
/// 32-bit guest addresses so the rewrite does not depend on host pointer width.
lf_checker_rt::export!(cdecl, rw_00b0c150(
    first_record: u32,
    second_record: u32,
    requested_key: u32,
    requested_mode: u32,
    owner_gate: u32,
    row_state: u32,
    event_enabled: u32,
    _unused_8: u32,
    _unused_9: u32,
    _unused_10: u32,
    owner: u32,
    owner_flag_a: u32,
    owner_flag_b: u32,
) -> u32 {
    unsafe {
        const POOL_BASE: u32 = 0x1615660;
        const POOL_FLAGS: u32 = 0x44;
        const ROW_STRIDE: u32 = 0x50;
        const ROW_LIMIT: u32 = 0x5DC;
        const SPECIAL_LIMIT: u32 = 0x578;
        const SPECIAL_MODE: u8 = 0x17;
        const FLAG_SECONDARY: u32 = 0x10400DC;
        const CURSOR: u32 = 0x1615620;
        const KEY_SOURCE: u32 = 0x12FA2CC;
        const KEY_REPLACEMENT: u32 = 0x12F9DA4;
        const MODE_KEY: u32 = 0x12FA3E0;
        const CALLBACK_CONTEXT: u32 = 0x18ECFB0;
        const EXTERNAL_CONTEXT: u32 = 0x18B600C;
        const EXTERNAL_ENABLED: u32 = 0x11E6450;
        const CALLBACK_TABLE: u32 = 0x00EAA8D8;
        const EVENT_COUNTER: u32 = 0x1615634;
        const EVENT_VALUE_A: u32 = 0x161562C;
        const EVENT_VALUE_B: u32 = 0x1615638;
        const EVENT_VALUE_C: u32 = 0x1615630;
        const EVENT_FLOAT: u32 = 0x10400CC;
        const OWNER_SPECIAL: u32 = 0x08000000;
        const OWNER_NOTIFY: u32 = 0x210;
        const TOKEN_OFFSET: u32 = 0x10;
        const PROPERTY_OFFSET: u32 = 0x0C;
        const POINTER_OFFSET: u32 = 0x04;
        const MODE_OFFSET: u32 = 0x44;
        const MODE_FLAGS_OFFSET: u32 = 0x45;
        const OWNER_FLAGS_OFFSET: u32 = 0x46;
        const KEY_OFFSET: u32 = 0x40;
        const VALUE_A_OFFSET: u32 = 0x18;
        const VALUE_B_OFFSET: u32 = 0x1C;
        const VALUE_C_OFFSET: u32 = 0x20;
        const RECORD_A_OFFSET: u32 = 0x30;
        const RECORD_B_OFFSET: u32 = 0x34;
        const RECORD_C_OFFSET: u32 = 0x38;
        const RECORD_D_OFFSET: u32 = 0x3C;
        const CALL_GATE: u32 = 1;
        const CALL_MODE_GATE: u32 = 2;
        const CALL_REFRESH_GATE: u32 = 3;
        const CALL_EARLY_DISPATCH: u32 = 4;
        const CALL_POOL_RECLAIM: u32 = 5;
        const CALL_POOL_RESET: u32 = 6;
        const CALL_POOL_SET_STATE: u32 = 7;
        const CALL_POOL_FINALIZE: u32 = 8;
        const CALL_POOL_QUERY_FLAGS: u32 = 9;
        const CALL_POOL_BUILD_TOKEN: u32 = 10;
        const CALL_TOKEN_FINALIZE: u32 = 11;
        const CALL_OWNER_PROPERTY: u32 = 12;
        const CALL_OWNER_NOTIFY: u32 = 13;
        const CALL_SLOT_PREPARE: u32 = 14;
        const CALL_SLOT_SEQUENCE: u32 = 15;
        const CALL_EXTERNAL_LOOKUP: u32 = 16;
        const CALL_EXTERNAL_NOTIFY: u32 = 17;
        const CALL_EXTERNAL_COMMIT: u32 = 18;
        const CALL_EVENT_CREATE: u32 = 19;
        const CALL_EVENT_SET: u32 = 20;
        const CALL_EVENT_SET_FLOAT: u32 = 21;

        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn read_u16(address: u32) -> u16 {
            unsafe { (address as *const u16).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }
        #[inline(always)]
        unsafe fn write_u16(address: u32, value: u16) {
            unsafe { (address as *mut u16).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn write_u32(address: u32, value: u32) {
            unsafe { (address as *mut u32).write_unaligned(value) }
        }
        #[inline(always)]
        unsafe fn global_u8(file_address: u32) -> u8 {
            unsafe { read_u8(lf_checker_rt::relocated(file_address)) }
        }
        #[inline(always)]
        unsafe fn global_u32(file_address: u32) -> u32 {
            unsafe { read_u32(lf_checker_rt::relocated(file_address)) }
        }
        #[inline(always)]
        unsafe fn write_global_u32(file_address: u32, value: u32) {
            unsafe { write_u32(lf_checker_rt::relocated(file_address), value) }
        }
        #[inline(always)]
        unsafe fn row_address(file_address: u32, index: u32) -> u32 {
            lf_checker_rt::relocated(file_address.wrapping_add(index.wrapping_mul(ROW_STRIDE)))
        }

        let original_mode = requested_mode as u8;
        if original_mode != SPECIAL_MODE {
            let initial_gate: u8 = lf_checker_rt::callee_cdecl!(CALL_GATE, u8,);
            if initial_gate != 0 {
                let mode_gate: u8 = lf_checker_rt::callee_cdecl!(CALL_MODE_GATE, u8,);
                if mode_gate == 0 {
                    let callback_context = lf_checker_rt::relocated(CALLBACK_CONTEXT);
                    let _ = lf_checker_rt::callee_thiscall!(
                        CALL_EARLY_DISPATCH,
                        u32,
                        callback_context,
                        0u32,
                        first_record,
                        requested_key,
                        requested_mode,
                        row_state,
                        0u32,
                        event_enabled,
                    );
                    return u32::MAX;
                }
            }
        }

        let mut key = requested_key;
        if key == global_u32(KEY_SOURCE) {
            key = global_u32(KEY_REPLACEMENT);
        }
        let mut mode = original_mode;
        if key == global_u32(MODE_KEY) && mode >= 0x18 {
            mode = 2;
        }

        let mut limit = ROW_LIMIT;
        let mut start_cursor = global_u32(CURSOR);
        let mut wrap_cursor = 0;
        if mode == SPECIAL_MODE {
            limit = SPECIAL_LIMIT;
            if start_cursor >= limit {
                start_cursor = 0;
            }
        } else {
            let gate = lf_checker_rt::callee_cdecl!(CALL_GATE, u8,);
            let mut use_secondary = gate != 0;
            if !use_secondary {
                use_secondary = lf_checker_rt::callee_cdecl!(CALL_REFRESH_GATE, u8,) != 0;
            }
            if use_secondary {
                start_cursor = global_u32(FLAG_SECONDARY);
                wrap_cursor = SPECIAL_LIMIT;
            }
        }

        let pool = lf_checker_rt::relocated(POOL_BASE);
        let mut free_row = None;
        let mut scan_index = start_cursor;
        if scan_index < limit {
            while scan_index < limit {
                if read_u8(pool.wrapping_add(scan_index.wrapping_mul(ROW_STRIDE)).wrapping_add(POOL_FLAGS)) == 0 {
                    free_row = Some(scan_index);
                    break;
                }
                scan_index = scan_index.wrapping_add(1);
            }
            if free_row.is_none() {
                scan_index = limit;
            }
        }
        if free_row.is_none() && wrap_cursor < start_cursor {
            scan_index = wrap_cursor;
            while scan_index < start_cursor {
                if read_u8(pool.wrapping_add(scan_index.wrapping_mul(ROW_STRIDE)).wrapping_add(POOL_FLAGS)) == 0 {
                    free_row = Some(scan_index);
                    break;
                }
                scan_index = scan_index.wrapping_add(1);
            }
            if free_row.is_none() {
                scan_index = start_cursor;
            }
        }

        let gate = lf_checker_rt::callee_cdecl!(CALL_GATE, u8,);
        let mut cursor_blocked = gate != 0;
        if !cursor_blocked {
            cursor_blocked = lf_checker_rt::callee_cdecl!(CALL_REFRESH_GATE, u8,) != 0;
        }
        let next_cursor = scan_index.wrapping_add(1);
        if !cursor_blocked {
            write_global_u32(CURSOR, if next_cursor == ROW_LIMIT { 0 } else { next_cursor });
        } else if mode == SPECIAL_MODE {
            write_global_u32(CURSOR, if next_cursor == SPECIAL_LIMIT { 0 } else { next_cursor });
        } else {
            let secondary = if next_cursor == ROW_LIMIT { SPECIAL_LIMIT } else { next_cursor };
            write_global_u32(FLAG_SECONDARY, secondary);
        }

        let recycled = free_row.is_none();
        let row_index = if let Some(index) = free_row {
            index
        } else {
            let mut reusable = None;
            let mut index = start_cursor;
            while index < limit {
                if read_u8(pool.wrapping_add(index.wrapping_mul(ROW_STRIDE)).wrapping_add(POOL_FLAGS)) == 8 {
                    reusable = Some(index);
                    break;
                }
                index = index.wrapping_add(1);
            }
            if reusable.is_none() {
                index = 0;
                while index < limit {
                    let state = read_u8(pool.wrapping_add(index.wrapping_mul(ROW_STRIDE)).wrapping_add(POOL_FLAGS));
                    if state == 4 || state == 5 {
                        reusable = Some(index);
                        break;
                    }
                    index = index.wrapping_add(1);
                }
            }
            match reusable {
                Some(index) => index,
                None => return u32::MAX,
            }
        };

        if row_index >= limit {
            return u32::MAX;
        }
        let row_file_address = POOL_BASE.wrapping_add(row_index.wrapping_mul(ROW_STRIDE));
        let row = row_address(POOL_BASE, row_index);
        if recycled {
            let _ = lf_checker_rt::callee_thiscall!(CALL_POOL_RECLAIM, u32, row);
        }
        let _ = lf_checker_rt::callee_thiscall!(CALL_POOL_RESET, u32, row, 0u32);

        write_u8(row.wrapping_add(MODE_FLAGS_OFFSET), read_u8(row.wrapping_add(MODE_FLAGS_OFFSET)) & 0xFE);
        write_u8(row.wrapping_add(MODE_OFFSET), mode);
        let _ = lf_checker_rt::callee_thiscall!(CALL_POOL_SET_STATE, u32, row, row_state);

        let bit_a = (owner_flag_a as u8).wrapping_shl(6);
        let old_mode_flags = read_u8(row.wrapping_add(MODE_FLAGS_OFFSET));
        let changed_mode_flags = (bit_a ^ old_mode_flags) & 0x40;
        write_u8(row.wrapping_add(MODE_FLAGS_OFFSET), old_mode_flags ^ changed_mode_flags);

        let old_owner_flags = read_u8(row.wrapping_add(OWNER_FLAGS_OFFSET));
        let changed_owner_flags = (old_owner_flags ^ owner_flag_b as u8) & 1;
        write_u8(row.wrapping_add(OWNER_FLAGS_OFFSET), old_owner_flags ^ changed_owner_flags);
        let _ = lf_checker_rt::callee_thiscall!(CALL_POOL_FINALIZE, u32, row);

        write_u16(row.wrapping_add(KEY_OFFSET), key as u16);
        write_u32(row.wrapping_add(VALUE_A_OFFSET), read_u32(first_record));
        write_u32(row.wrapping_add(VALUE_B_OFFSET), read_u32(first_record.wrapping_add(4)));
        write_u32(row.wrapping_add(VALUE_C_OFFSET), read_u32(first_record.wrapping_add(8)));
        write_u32(row.wrapping_add(RECORD_A_OFFSET), read_u32(second_record));
        write_u32(row.wrapping_add(RECORD_B_OFFSET), read_u32(second_record.wrapping_add(4)));
        write_u32(row.wrapping_add(RECORD_C_OFFSET), read_u32(second_record.wrapping_add(8)));
        write_u32(row.wrapping_add(RECORD_D_OFFSET), read_u32(second_record.wrapping_add(12)));

        let helper_flags: u8 = lf_checker_rt::callee_thiscall!(CALL_POOL_QUERY_FLAGS, u8, row);
        let old_mode_flags = read_u8(row.wrapping_add(MODE_FLAGS_OFFSET));
        let changed_mode_flags = (helper_flags.wrapping_shl(3) ^ old_mode_flags) & 8;
        write_u8(row.wrapping_add(MODE_FLAGS_OFFSET), old_mode_flags ^ changed_mode_flags);
        write_u32(row.wrapping_add(POINTER_OFFSET), owner);

        if mode == SPECIAL_MODE {
            let mut callback_word = 0u32;
            let callback_out = &mut callback_word as *mut u32 as u32;
            let callback_value: u32 = lf_checker_rt::callee_thiscall!(
                CALL_POOL_BUILD_TOKEN,
                u32,
                row,
                callback_out,
                CALLBACK_TABLE,
                row_index,
            );
            let token: u32 = lf_checker_rt::callee_cdecl!(CALL_TOKEN_FINALIZE, u32, callback_value);
            write_u32(row.wrapping_add(TOKEN_OFFSET), token);
        }

        if owner != 0 {
            if read_u32(owner.wrapping_add(0x24)) & OWNER_SPECIAL != 0 {
                let property: u32 = lf_checker_rt::callee_thiscall!(
                    CALL_OWNER_PROPERTY,
                    u32,
                    owner.wrapping_add(0x80),
                );
                write_u32(row.wrapping_add(PROPERTY_OFFSET), property);
            }
            let owner_notify = read_u32(owner.wrapping_add(OWNER_NOTIFY));
            write_u32(owner.wrapping_add(OWNER_NOTIFY), owner_notify | 1);
            let row_owner_slot = row.wrapping_add(POINTER_OFFSET);
            let _ = lf_checker_rt::callee_thiscall!(CALL_OWNER_NOTIFY, u32, owner, row_owner_slot);
        }

        let _ = lf_checker_rt::callee_cdecl!(CALL_SLOT_PREPARE, u32, row_index);
        let sequence: u32 = lf_checker_rt::callee_cdecl!(CALL_SLOT_SEQUENCE, u32, row_index);

        if mode != SPECIAL_MODE {
            let refresh: u8 = lf_checker_rt::callee_cdecl!(CALL_REFRESH_GATE, u8,);
            let mut notify = refresh != 0;
            if !notify {
                notify = lf_checker_rt::callee_cdecl!(CALL_GATE, u8,) != 0;
            }
            if notify {
                let context = global_u32(EXTERNAL_CONTEXT);
                let first_object: u32 = lf_checker_rt::callee_thiscall!(CALL_EXTERNAL_LOOKUP, u32, context);
                if first_object != 0 {
                    let object: u32 = lf_checker_rt::callee_thiscall!(CALL_EXTERNAL_LOOKUP, u32, context);
                    let vtable = read_u32(object);
                    let callback = read_u32(vtable.wrapping_add(0x34));
                    let callback_fn: extern "thiscall" fn(u32, u32) -> u32 =
                        core::mem::transmute(callback as usize);
                    let _ = callback_fn(object, row_index.wrapping_sub(SPECIAL_LIMIT));

                    if global_u8(EXTERNAL_ENABLED) != 0 && owner_gate as u8 != 0 {
                        let object: u32 = lf_checker_rt::callee_thiscall!(CALL_EXTERNAL_LOOKUP, u32, context);
                        let _ = lf_checker_rt::callee_thiscall!(
                            CALL_EXTERNAL_COMMIT,
                            u32,
                            object,
                            row_index.wrapping_sub(SPECIAL_LIMIT),
                        );
                    }
                }
            }
        }

        if event_enabled != 0 {
            let event: u32 = lf_checker_rt::callee_cdecl!(
                CALL_EVENT_CREATE,
                u32,
                1u32,
                6u32,
                row_index,
                4u32,
                0u32,
            );
            write_u32(row.wrapping_add(TOKEN_OFFSET), event);
            let _ = lf_checker_rt::callee_cdecl!(
                CALL_EVENT_SET,
                u32,
                3u32,
                event,
                global_u32(EVENT_COUNTER),
            );
            let _ = lf_checker_rt::callee_cdecl!(
                CALL_EVENT_SET_FLOAT,
                u32,
                0u32,
                row.wrapping_add(0x70),
                global_u32(EVENT_FLOAT),
            );
            let _ = lf_checker_rt::callee_cdecl!(
                CALL_EVENT_SET,
                u32,
                1u32,
                row.wrapping_add(0x70),
                global_u32(EVENT_VALUE_A),
            );
            let _ = lf_checker_rt::callee_cdecl!(
                CALL_EVENT_SET,
                u32,
                5u32,
                row.wrapping_add(0x70),
                global_u32(EVENT_VALUE_B),
            );
            let _ = lf_checker_rt::callee_cdecl!(
                CALL_EVENT_SET,
                u32,
                15u32,
                row.wrapping_add(0x70),
                global_u32(EVENT_VALUE_C),
            );
            write_u8(row.wrapping_add(OWNER_FLAGS_OFFSET), read_u8(row.wrapping_add(OWNER_FLAGS_OFFSET)) | 4);
        }

        let _ = row_file_address;
        sequence
    }
});
