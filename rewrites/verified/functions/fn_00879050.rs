// original: 0x00879050 composer_slot_alloc

/// Allocate one composer slot from the block at `this + 0x0c`.
///
/// `this` points to a composer object with a slot cursor at `+0x94` and a
/// slot block pointer at `+0x0c`. While the cursor is below 64 (SIGNED
/// comparison: a negative cursor still allocates), the cursor is bumped
/// and the slot at `block + 0xE1C + cursor * 0x34` is handed out with its
/// words at `+0x2C` and `+0x30` cleared. A cursor of 64 or more (signed)
/// means the block is exhausted and 0 is returned.
///
/// Original: 0x00879050 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00879050(this: u32) -> u32 {
    unsafe {
        const CURSOR_OFF: u32 = 0x94;
        const BLOCK_OFF: u32 = 0x0c;
        const SLOTS_BASE: u32 = 0xe1c;
        const SLOT_STRIDE: u32 = 0x34;
        const MAX_SLOTS: i32 = 0x40;
        const CLEAR0_OFF: u32 = 0x2c;
        const CLEAR1_OFF: u32 = 0x30;
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        let cursor = rd32(this.wrapping_add(CURSOR_OFF)) as i32;
        if cursor >= MAX_SLOTS {
            return 0;
        }
        wr32(this.wrapping_add(CURSOR_OFF), (cursor.wrapping_add(1)) as u32);
        let block = rd32(this.wrapping_add(BLOCK_OFF));
        let slot = block
            .wrapping_add(SLOTS_BASE)
            .wrapping_add((cursor as u32).wrapping_mul(SLOT_STRIDE));
        wr32(slot.wrapping_add(CLEAR0_OFF), 0);
        wr32(slot.wrapping_add(CLEAR1_OFF), 0);
        slot
    }
});
