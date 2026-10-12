// original: 0x009E2DD0 tbd

/// Insert the record into the first free word of the row selected by its
/// index, clear its state byte, then store the signed division remainder
/// and increment that row's counter. Return the signed quotient. When all
/// 120 row entries are occupied, return the address just past the row and
/// leave the record and counter unchanged.
lf_checker_rt::export!(cdecl, rw_009e2dd0(record: u32) -> u32 {
    unsafe {
        const TABLE_VA: u32 = 0x012B_41D8;
        const DIVISOR_POINTERS_VA: u32 = 0x012B_60A0;
        const COUNTERS_VA: u32 = 0x012B_5858;
        const ROW_BYTES: u32 = 0x1E0;
        const ROW_WORDS: u32 = 120;

        let index = (record.wrapping_add(4) as *const u32).read_unaligned();
        let row_offset = index.wrapping_mul(ROW_BYTES);
        let mut free_slot = 0u32;
        while free_slot < ROW_WORDS {
            let cell_va = TABLE_VA
                .wrapping_add(row_offset)
                .wrapping_add(free_slot.wrapping_mul(4));
            if (lf_checker_rt::global::<u32>(cell_va) as *const u32).read_unaligned() == 0 {
                break;
            }
            free_slot = free_slot.wrapping_add(1);
        }

        if free_slot == ROW_WORDS {
            return lf_checker_rt::global::<u32>(
                TABLE_VA
                    .wrapping_add(row_offset)
                    .wrapping_add(ROW_WORDS.wrapping_mul(4)),
            ) as u32;
        }

        let table_index = index
            .wrapping_mul(ROW_WORDS)
            .wrapping_add(free_slot);
        let selected_cell = TABLE_VA.wrapping_add(table_index.wrapping_mul(4));
        (lf_checker_rt::global::<u32>(selected_cell) as *mut u32)
            .write_unaligned(record);

        (record.wrapping_add(8) as *mut u8).write(0);

        let divisor_object = (lf_checker_rt::global::<u32>(
            DIVISOR_POINTERS_VA.wrapping_add(index.wrapping_mul(4)),
        ) as *const u32)
            .read_unaligned();
        let divisor = (divisor_object.wrapping_add(4) as *const i32).read_unaligned();
        let signed_slot = free_slot as i32;
        let quotient = signed_slot / divisor;
        let remainder = signed_slot % divisor;
        (record as *mut u32).write_unaligned(remainder as u32);

        let counter_va = COUNTERS_VA.wrapping_add(index.wrapping_mul(4));
        let counter_ptr = lf_checker_rt::global::<u32>(counter_va);
        let counter = counter_ptr.read_unaligned();
        counter_ptr.write_unaligned(counter.wrapping_add(1));

        quotient as u32
    }
});
