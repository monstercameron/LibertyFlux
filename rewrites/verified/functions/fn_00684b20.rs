// original: 0x00684B20 crFrame::release_frames

/// Release each pointer in the frame record's item array through virtual slot
/// two, clearing each array entry after its release call. The unsigned 16-bit
/// item count is at byte 0x10, the array pointer at byte 0x0c, and the final
/// status word at byte 8 is reset even when the count is zero. With a
/// positive count, the return value is the item-array pointer. With a zero
/// count, the original preserves entry EAX; the contract fixes entry EAX
/// to zero so that path is compared exactly. This method has no stack args.
lf_checker_rt::export!(thiscall, rw_00684b20(this: u32) -> u32 {
    unsafe {
        const ITEMS_OFFSET: u32 = 0x0C;
        const ITEM_COUNT_OFFSET: u32 = 0x10;
        const STATUS_OFFSET: u32 = 8;
        const ITEM_BYTES: u32 = 4;

        let count = crate::read_halfword(this.wrapping_add(ITEM_COUNT_OFFSET)) as u32;
        let items = crate::read_word(this.wrapping_add(ITEMS_OFFSET));
        if count == 0 {
            crate::write_word(this.wrapping_add(STATUS_OFFSET), 0);
            return 0;
        }
        for index in 0..count {
            let slot = items.wrapping_add(index.wrapping_mul(ITEM_BYTES));
            let item = crate::read_word(slot);
            let _ = lf_checker_rt::callee_thiscall!(1, u32, this, item);
            crate::write_word(slot, 0);
        }
        crate::write_word(this.wrapping_add(STATUS_OFFSET), 0);
        items
    }
});
