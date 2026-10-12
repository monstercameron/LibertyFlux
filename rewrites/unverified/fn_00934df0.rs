// original: 0x00934DF0 content_live_slots_teardown_payload

/// Refresh the slot owner and shared worker, then visit the counted slot
/// records in increasing index order. Call the supplied teardown method only
/// when both record flags are nonzero. The count is a zero-extended 16-bit
/// field and is reloaded after each iteration; the final count is returned in
/// EAX. The receiver follows the thiscall convention.
lf_checker_rt::export!(thiscall, rw_00934DF0(this: *mut u8) -> u32 {
    const SLOT_ARRAY_POINTER_OFFSET: usize = 0x144;
    const SLOT_COUNT_OFFSET: usize = 0x148;
    const SLOT_STRIDE: u32 = 0x22C;
    const PRESENT_FLAG_OFFSET: u32 = 0x225;
    const ACTIVE_FLAG_OFFSET: u32 = 0x226;
    const REFRESH_CALLEE: u32 = 1;
    const SET_OWNER_CALLEE: u32 = 2;
    const TEARDOWN_CALLEE: u32 = 3;

    unsafe {
        let receiver = this as u32;
        lf_checker_rt::callee_thiscall!(REFRESH_CALLEE, u32, receiver);
        lf_checker_rt::callee_thiscall!(SET_OWNER_CALLEE, u32, receiver);

        let mut count = u32::from(
            this.add(SLOT_COUNT_OFFSET).cast::<u16>().read_unaligned(),
        );
        let mut index = 0u32;
        while index < count {
            let slots = this.add(SLOT_ARRAY_POINTER_OFFSET).cast::<u32>().read_unaligned();
            let record = slots.wrapping_add(index.wrapping_mul(SLOT_STRIDE));
            let active = (record.wrapping_add(ACTIVE_FLAG_OFFSET) as *const u8).read() != 0;
            if active {
                let present = (record.wrapping_add(PRESENT_FLAG_OFFSET) as *const u8).read() != 0;
                if present {
                    lf_checker_rt::callee_thiscall!(TEARDOWN_CALLEE, u32, receiver, index);
                }
            }
            index = index.wrapping_add(1);
            count = u32::from(
                this.add(SLOT_COUNT_OFFSET).cast::<u16>().read_unaligned(),
            );
        }
        count
    }
});
