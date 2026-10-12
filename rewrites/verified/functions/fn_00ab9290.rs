// original: 0x00AB9290 slot_table_reset_all

/// Reset the slot sentinels and clear the table's byte flags.
///
/// The thiscall receiver contains eleven 32-bit slots, each reset to the
/// invalid sentinel. The byte flags from offsets 152 through 167 and the
/// 16-bit count at offset 143 are cleared. The receiver is returned in EAX.
lf_checker_rt::export!(thiscall, rw_00ab9290(this: u32) -> u32 {
    const SLOT_COUNT: usize = 11;
    const INVALID_SLOT: u32 = u32::MAX;
    const BYTE_FLAGS_START: usize = 0x98;
    const BYTE_FLAGS_LENGTH: usize = 16;
    const ITEM_COUNT: usize = 0x8f;

    unsafe {
        let receiver = this as *mut u8;
        let slots = receiver.cast::<u32>();
        for index in 0..SLOT_COUNT {
            slots.add(index).write_unaligned(INVALID_SLOT);
        }
        for offset in BYTE_FLAGS_START..BYTE_FLAGS_START + BYTE_FLAGS_LENGTH {
            receiver.add(offset).write(0);
        }
        receiver.add(ITEM_COUNT).cast::<u16>().write_unaligned(0);
    }

    this
});
