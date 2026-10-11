// original: 0x00C59B50 CEntityScanner::vf1

/// Update one entity-scanner slot from its indexed table entry. The scanner
/// stores a signed comparison key at `+0x04`, a table index at `+0x08`, a
/// rolling count at `+0x10`, and its signed count limit at `+0x14`. A key of
/// `-1` uses the rolling-count path; otherwise the signed remainder of a
/// global numerator by the indexed entry's divisor must equal the key.
///
/// On the update path the function clears the scanner's sixteen recent slots,
/// initializes the selected table entry when the key is not `-1`, chooses the
/// greater of two object floats (or a fixed radius for mode 2), advances a
/// global 16-bit sequence counter, and calls the candidate scanner. The
/// thiscall method takes the scanner in ECX and two stack words: mode and a
/// context pointer. It returns no meaningful value to its known callers.
lf_checker_rt::export!(thiscall, rw_entity_scanner_vf1(
    scanner: u32,
    mode: u32,
    context: u32,
) -> () {
    unsafe {
        const KEY_OFFSET: u32 = 0x04;
        const TABLE_INDEX_OFFSET: u32 = 0x08;
        const SCANNER_ACTIVE_FLAG_OFFSET: u32 = 0x0c;
        const ROLLING_COUNT_OFFSET: u32 = 0x10;
        const ROLLING_LIMIT_OFFSET: u32 = 0x14;
        const CONTEXT_ENTITY_OFFSET: u32 = 0x224;
        const ENTITY_RADIUS_A_OFFSET: u32 = 0xe0;
        const ENTITY_RADIUS_B_OFFSET: u32 = 0xe4;
        const ENTITY_OUTPUT_OFFSET: u32 = 0x20;
        const OUTPUT_POINT_OFFSET: u32 = 0x30;
        const TABLE_ENTRY_DIVISOR_OFFSET: u32 = 0x04;
        const TABLE_ENTRY_METHODS_OFFSET: u32 = 0x08;
        const TABLE_BASE_VA: u32 = 0x012b60a0;
        const NUMERATOR_VA: u32 = 0x012b41d4;
        const SEQUENCE_COUNTER_VA: u32 = 0x011a8908;
        const FIXED_RADIUS_VA: u32 = 0x00fe8b08;
        const MODE_FIXED_RADIUS: u32 = 2;
        const SCANNER_RECENT_SLOTS: u32 = 0x18;
        const SCANNER_RECENT_COUNT: u32 = 16;
        const CALLEE_CLEAR_SCANNER: u32 = 1;
        const CALLEE_TABLE_INIT_A: u32 = 2;
        const CALLEE_TABLE_INIT_B: u32 = 3;
        const CALLEE_SEQUENCE_WRAP: u32 = 4;
        const CALLEE_SCAN_CANDIDATES: u32 = 5;
        const CALLEE_REFRESH_SCANNER: u32 = 6;

        unsafe fn read_u32(address: u32) -> u32 {
            (address as *const u32).read_volatile()
        }

        unsafe fn read_u16(address: u32) -> u16 {
            (address as *const u16).read_volatile()
        }

        unsafe fn write_u32(address: u32, value: u32) {
            (address as *mut u32).write_volatile(value);
        }

        unsafe fn write_u8(address: u32, value: u8) {
            (address as *mut u8).write_volatile(value);
        }

        unsafe fn write_u16(address: u32, value: u16) {
            (address as *mut u16).write_volatile(value);
        }

        let key = read_u32(scanner.wrapping_add(KEY_OFFSET));
        let regular_path = key != u32::MAX;
        let table_entry = if regular_path {
            let index = read_u32(scanner.wrapping_add(TABLE_INDEX_OFFSET));
            let table_slot = lf_checker_rt::relocated(TABLE_BASE_VA)
                .wrapping_add(index.wrapping_mul(4));
            let entry = read_u32(table_slot);
            let numerator = read_u32(lf_checker_rt::relocated(NUMERATOR_VA)) as i32;
            let divisor = read_u32(entry.wrapping_add(TABLE_ENTRY_DIVISOR_OFFSET)) as i32;
            if numerator % divisor != key as i32 {
                return;
            }
            entry
        } else {
            let previous = read_u32(scanner.wrapping_add(ROLLING_COUNT_OFFSET));
            let limit = read_u32(scanner.wrapping_add(ROLLING_LIMIT_OFFSET)) as i32;
            let mut next = previous.wrapping_add(1);
            if (next as i32) > limit {
                next = 0;
            }
            write_u32(scanner.wrapping_add(ROLLING_COUNT_OFFSET), next);
            if previous != 0 {
                return;
            }
            0
        };

        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_CLEAR_SCANNER,
            u32,
            scanner,
        );

        if regular_path {
            write_u8(scanner.wrapping_add(SCANNER_ACTIVE_FLAG_OFFSET), 1);
            let methods = table_entry.wrapping_add(TABLE_ENTRY_METHODS_OFFSET);
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_TABLE_INIT_A,
                u32,
                methods,
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_TABLE_INIT_B,
                u32,
                methods,
            );
        }

        let entity = read_u32(context.wrapping_add(CONTEXT_ENTITY_OFFSET));
        let radius_a = f32::from_bits(read_u32(entity.wrapping_add(ENTITY_RADIUS_A_OFFSET)));
        let radius_b = f32::from_bits(read_u32(entity.wrapping_add(ENTITY_RADIUS_B_OFFSET)));
        let mut selected_radius = if radius_a > radius_b { radius_a } else { radius_b };
        if mode == MODE_FIXED_RADIUS {
            selected_radius = f32::from_bits(read_u32(
                lf_checker_rt::relocated(FIXED_RADIUS_VA),
            ));
        }

        let output_owner = read_u32(context.wrapping_add(ENTITY_OUTPUT_OFFSET));
        let output_point = output_owner.wrapping_add(OUTPUT_POINT_OFFSET);
        let counter_ptr = lf_checker_rt::relocated(SEQUENCE_COUNTER_VA);
        let mut sequence = read_u16(counter_ptr);
        if sequence == u16::MAX {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_SEQUENCE_WRAP,
                u32,
                context,
            );
            sequence = 1;
        } else {
            sequence = sequence.wrapping_add(1);
        }
        write_u16(counter_ptr, sequence);

        let _: u32 = lf_checker_rt::callee_thiscall!(
            CALLEE_SCAN_CANDIDATES,
            u32,
            scanner,
            mode,
            output_point,
            selected_radius.to_bits(),
            context,
        );

        if regular_path {
            let _: u32 = lf_checker_rt::callee_thiscall!(
                CALLEE_REFRESH_SCANNER,
                u32,
                scanner.wrapping_add(KEY_OFFSET),
            );
        }
    }
});
