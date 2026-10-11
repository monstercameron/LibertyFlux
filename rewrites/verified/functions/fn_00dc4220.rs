// original: 0x00DC4220 task_slot_add_reference

/// Add an item to the first vacant slot in a fixed task table.
///
/// The receiver has 64 pointer slots at offsets 4 through 0x100 and a
/// signed maximum-slot field at offset 0x108. When a slot is empty, the
/// item pointer is stored there, the reference-count helper is called with
/// the item and the slot address, byte 0xed4 of the item is set, and the
/// maximum-slot field becomes the signed greater of its previous value and
/// the slot index. A full table is left unchanged.
lf_checker_rt::export!(thiscall, rw_00DC4220(table: u32, item: u32) -> u32 {
    unsafe {
        const FIRST_SLOT: u32 = 0x04;
        const SLOT_COUNT: u32 = 64;
        const SLOT_BYTES: u32 = 4;
        const MAX_SLOT: u32 = 0x108;
        const ITEM_MARKER: u32 = 0xed4;
        const ADD_REFERENCE: u32 = 1;

        let mut index = 0u32;
        while index < SLOT_COUNT {
            let slot_address = table
                .wrapping_add(FIRST_SLOT)
                .wrapping_add(index.wrapping_mul(SLOT_BYTES));
            let current = (slot_address as *const u32).read_unaligned();
            if current == 0 {
                (slot_address as *mut u32).write_unaligned(item);
                let _ = lf_checker_rt::callee_thiscall!(
                    ADD_REFERENCE,
                    u32,
                    item,
                    slot_address,
                );
                (item.wrapping_add(ITEM_MARKER) as *mut u8).write(1);

                let previous = (table.wrapping_add(MAX_SLOT) as *const u32).read_unaligned();
                if (previous as i32) > (index as i32) {
                    index = previous;
                }
                (table.wrapping_add(MAX_SLOT) as *mut u32).write_unaligned(index);
                break;
            }
            index += 1;
        }
    }
    0
});
