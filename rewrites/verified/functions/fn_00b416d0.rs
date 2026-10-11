// original: 0x00B416D0 task_global_tables_init

/// Rebuild the task lookup tables using the configured row count.
///
/// The routine releases two previous allocations, constructs a 0xA0-byte
/// header, allocates four row buffers and two index buffers, and initializes
/// each row from its prior heap contents. The count is a 32-bit global; row
/// and byte-size arithmetic follows the original wrapping/overflow behavior.
/// Calls to the destructor, allocator, constructor, release helper and clear
/// helper are intercepted by the checker contract. The code uses 32-bit game
/// pointers only at the boundary; the table calculations are ordinary Rust.
#[unsafe(no_mangle)]
pub extern "C" fn rw_00B416D0() -> u32 {
    const GLOBAL_PRIMARY: u32 = 0x016B8F8C;
    const GLOBAL_SECONDARY: u32 = 0x016B8F90;
    const GLOBAL_ACCOUNTING: u32 = 0x016B6B0C;
    const GLOBAL_BITS_PTR: u32 = 0x016B9DB8;
    const GLOBAL_BITS_COUNTS: u32 = 0x016B9DBC;
    const GLOBAL_ZEROED: u32 = 0x016B8FA0;
    const GLOBAL_ROW_COUNT: u32 = 0x00EEDE18;
    const HEADER_BYTES: u32 = 0xA0;
    const HEADER_TRIPLES: u32 = 0x78;
    const HEADER_ROWS: u32 = 0x7C;
    const HEADER_ROW_FLAGS: u32 = 0x5C;
    const HEADER_ROW_IDS: u32 = 0x60;
    const HEADER_ROW_DATA: u32 = 0x64;
    const HEADER_RECORDS: u32 = 0x6C;
    const HEADER_BITS_A: u32 = 0x80;
    const HEADER_BITS_B: u32 = 0x84;
    const RELEASE_OLD_OBJECT: u32 = 1;
    const RELEASE_MEMORY: u32 = 2;
    const ALLOCATE: u32 = 3;
    const CONSTRUCT: u32 = 4;
    const CLEAR_0: u32 = 5;
    const CLEAR_1: u32 = 6;
    const CLEAR_2: u32 = 7;
    const CLEAR_3: u32 = 8;
    const CLEAR_4: u32 = 9;
    const CLEAR_5: u32 = 10;
    const INITIAL_ROW_WORD: u32 = 0xFFFF_0FFF;
    const ROW_FLAGS_MASK: u32 = 0xEFFF_FFFF;
    const ROW_FLAGS_VALUE: u32 = 0x0FFF_FFFF;
    const RECORD_FLAGS_MASK: u32 = 0xFCE8_0000;
    const RECORD_FLAGS_VALUE: u32 = 0x1CE8_0000;
    const RECORD_LOW_MASK: u32 = 0x1FFFF;
    const RECORD_LOW_INITIAL_MASK: u32 = 0xFE60_0000;
    const RECORD_LOW_INITIAL_VALUE: u32 = 0x0060_0000;
    const ROW_STRIDE: u32 = 0x28;

    let read = |base: u32, offset: u32| -> u32 {
        // SAFETY: globals and allocated segments are declared by the contract.
        unsafe { (base.wrapping_add(offset) as *const u32).read_unaligned() }
    };
    let write = |base: u32, offset: u32, value: u32| {
        // SAFETY: every output address is a declared global or allocated row.
        unsafe { (base.wrapping_add(offset) as *mut u32).write_unaligned(value) }
    };
    let global_ptr = |address: u32| -> u32 {
        lf_checker_rt::global::<u32>(address) as u32
    };
    let global_value = |address: u32| -> u32 { read(global_ptr(address), 0) };
    let set_global = |address: u32, value: u32| { write(global_ptr(address), 0, value) };
    let allocation_size = |count: u32, stride: u32| -> u32 {
        count.checked_mul(stride).unwrap_or(u32::MAX)
    };

    let previous_primary = global_value(GLOBAL_PRIMARY);
    if previous_primary != 0 {
        let _ = lf_checker_rt::callee_thiscall!(RELEASE_OLD_OBJECT, u32, previous_primary);
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_MEMORY, u32, previous_primary);
        set_global(GLOBAL_PRIMARY, 0);
    }
    let previous_secondary = global_value(GLOBAL_SECONDARY);
    if previous_secondary != 0 {
        let _ = lf_checker_rt::callee_cdecl!(RELEASE_MEMORY, u32, previous_secondary);
        set_global(GLOBAL_SECONDARY, 0);
    }

    let row_count = global_value(GLOBAL_ROW_COUNT);
    let triple_count = row_count.wrapping_mul(3);
    let header = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, HEADER_BYTES);
    let header = if header != 0 {
        let _ = lf_checker_rt::callee_thiscall!(CONSTRUCT, u32, header);
        header
    } else {
        0
    };
    let accounting = global_value(GLOBAL_ACCOUNTING).wrapping_add(HEADER_BYTES);
    set_global(GLOBAL_ACCOUNTING, accounting);
    set_global(GLOBAL_PRIMARY, header);

    if header != 0 {
        write(header, 0x50, 0);
        write(header, HEADER_ROWS, row_count);
        write(header, HEADER_TRIPLES, triple_count);
        write(header, 0x58, 0);

        let row_flags = lf_checker_rt::callee_cdecl!(
            ALLOCATE, u32, allocation_size(triple_count, 16)
        );
        write(header, HEADER_ROW_FLAGS, row_flags);
        let row_ids = lf_checker_rt::callee_cdecl!(
            ALLOCATE, u32, allocation_size(triple_count, 2)
        );
        write(header, HEADER_ROW_IDS, row_ids);
        let row_data = lf_checker_rt::callee_cdecl!(
            ALLOCATE, u32, allocation_size(triple_count, 8)
        );
        write(header, HEADER_ROW_DATA, row_data);

        if row_data != 0 && (triple_count.wrapping_sub(1) as i32) >= 0 {
            for row in 0..triple_count {
                let entry = row_data.wrapping_add(row.wrapping_mul(8));
                let prior_flags = read(entry, 4);
                write(entry, 0, INITIAL_ROW_WORD);
                write(
                    entry,
                    4,
                    (prior_flags & ROW_FLAGS_MASK) | ROW_FLAGS_VALUE,
                );
            }
        }
        set_global(
            GLOBAL_ACCOUNTING,
            global_value(GLOBAL_ACCOUNTING)
                .wrapping_add(triple_count.wrapping_mul(0x1A)),
        );
        write(header, 0x68, triple_count);

        let records = lf_checker_rt::callee_cdecl!(
            ALLOCATE, u32, allocation_size(row_count, ROW_STRIDE)
        );
        write(header, HEADER_RECORDS, records);
        if records != 0 && (row_count.wrapping_sub(1) as i32) >= 0 {
            for row in 0..row_count {
                let record = records.wrapping_add(row.wrapping_mul(ROW_STRIDE));
                write(record, 0x0C, 0);
                write(record, 0x24, 0);
            }
        }

        // The source reads a stack slot it never initialized; this models the
        // contract's explicit zero stack fill.
        write(header, 0x70, 0);
        write(header, 0x40, 0);
        write(header, 0x44, 0);
        write(header, 0x48, 0);
        write(header, 0x4C, 0);
        write(header, HEADER_BITS_A, 0xE74);
        write(header, HEADER_BITS_B, 0);
        let header_accounting = row_count
            .wrapping_mul(5)
            .wrapping_shl(3);
        set_global(
            GLOBAL_ACCOUNTING,
            global_value(GLOBAL_ACCOUNTING).wrapping_add(header_accounting),
        );

        let _ = lf_checker_rt::callee_cdecl!(
            CLEAR_0, u32, row_flags, 0, triple_count.wrapping_mul(16)
        );
        let _ = lf_checker_rt::callee_cdecl!(
            CLEAR_1, u32, row_ids, 0, triple_count.wrapping_mul(2)
        );
        let _ = lf_checker_rt::callee_cdecl!(
            CLEAR_2, u32, row_data, 0, triple_count.wrapping_mul(8)
        );
        let _ = lf_checker_rt::callee_cdecl!(
            CLEAR_3, u32, records, 0, header_accounting
        );

        if triple_count as i32 > 0 {
            for row in 0..triple_count {
                let id = row_ids.wrapping_add(row.wrapping_mul(2));
                unsafe { (id as *mut u16).write_unaligned(row as u16) };
            }
        }

        let row_count_again = global_value(GLOBAL_ROW_COUNT);
        let ids = lf_checker_rt::callee_cdecl!(
            ALLOCATE, u32, allocation_size(row_count_again, 4)
        );
        if ids != 0 {
            if (row_count_again.wrapping_sub(1) as i32) >= 0 {
                for row in 0..row_count_again {
                    write(ids, row.wrapping_mul(4), INITIAL_ROW_WORD);
                }
            }
            set_global(GLOBAL_SECONDARY, ids);
        } else {
            set_global(GLOBAL_SECONDARY, 0);
        }
        set_global(
            GLOBAL_ACCOUNTING,
            global_value(GLOBAL_ACCOUNTING).wrapping_add(row_count_again.wrapping_mul(4)),
        );

        if row_count as i32 > 0 {
            for row in 0..row_count {
                let record = records.wrapping_add(row.wrapping_mul(ROW_STRIDE));
                let old_low = read(record, 0);
                let old_flags = read(record, 4);
                write(
                    record,
                    0,
                    (old_low & RECORD_LOW_INITIAL_MASK) | RECORD_LOW_INITIAL_VALUE,
                );
                let low_bits = row.wrapping_mul(3) & RECORD_LOW_MASK;
                write(
                    record,
                    4,
                    (old_flags & RECORD_FLAGS_MASK) | low_bits | RECORD_FLAGS_VALUE,
                );
                write(ids, row.wrapping_mul(4), INITIAL_ROW_WORD);
            }
        }
    }

    let old_bits = global_value(GLOBAL_BITS_PTR);
    let _ = lf_checker_rt::callee_cdecl!(RELEASE_MEMORY, u32, old_bits);
    let word_count = row_count.wrapping_add(0x1F).wrapping_shr(5) as u16;
    let count_word = row_count as u16;
    let counts_ptr = global_ptr(GLOBAL_BITS_COUNTS);
    unsafe {
        (counts_ptr as *mut u16).write_unaligned(word_count);
        (counts_ptr.wrapping_add(2) as *mut u16).write_unaligned(count_word);
    }
    let bits_size = u32::from(word_count).wrapping_mul(4);
    let bit_table = lf_checker_rt::callee_cdecl!(ALLOCATE, u32, bits_size);
    let _ = lf_checker_rt::callee_cdecl!(CLEAR_4, u32, bit_table, 0, bits_size);
    set_global(GLOBAL_BITS_PTR, bit_table);
    let _ = lf_checker_rt::callee_cdecl!(CLEAR_5, u32, bit_table, 0, bits_size);
    set_global(GLOBAL_ZEROED, 0);
    1
}
