// original: 0x00DC4600 task_slots_release_all

/// Clear all task references and reset the table's sentinel fields.
///
/// The thiscall receiver contains 64 pointer entries at offsets 4 through
/// 0x100. For each nonzero entry, the reference-release helper receives that
/// item as its this pointer and the address of the slot as its stack argument.
/// The slot is then cleared. Finally, the two sentinel words at offsets zero
/// and 0x108 are set to all ones.
lf_checker_rt::export!(thiscall, rw_00DC4600(table: u32) -> u32 {
    unsafe {
        const FIRST_SLOT: u32 = 0x04;
        const SLOT_COUNT: u32 = 64;
        const SLOT_BYTES: u32 = 4;
        const TABLE_SENTINEL: u32 = 0x00;
        const LAST_SLOT_SENTINEL: u32 = 0x108;
        const RELEASE_REFERENCE: u32 = 1;

        (table.wrapping_add(TABLE_SENTINEL) as *mut u32).write_unaligned(u32::MAX);
        (table.wrapping_add(LAST_SLOT_SENTINEL) as *mut u32).write_unaligned(u32::MAX);

        let mut index = 0u32;
        while index < SLOT_COUNT {
            let slot_address = table
                .wrapping_add(FIRST_SLOT)
                .wrapping_add(index.wrapping_mul(SLOT_BYTES));
            let item = (slot_address as *const u32).read_unaligned();
            if item != 0 {
                let _ = lf_checker_rt::callee_thiscall!(
                    RELEASE_REFERENCE,
                    u32,
                    item,
                    slot_address,
                );
                (slot_address as *mut u32).write_unaligned(0);
            }
            index += 1;
        }
    }
    0
});
