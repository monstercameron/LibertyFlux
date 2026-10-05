// original: 0x008deab0 arena_alloc_aligned (proposed)

/// Advance a bump arena and return the aligned block address for `base`.
///
/// `base` is rounded up to 16 bytes and one 16-byte row is added. When
/// `size` is `0xffffffff` the arena cursor at `this + 0x18` is used as the
/// size instead, and the cursor is grown by the aligned block plus its own
/// alignment padding while the allocation counter global is bumped. When
/// `base + size` reaches `LIMIT` the request is rejected with 0; otherwise
/// the word at slot index `[this + 0x10]` is returned plus one row and the
/// size. Thiscall, two stack arguments, no calls.
lf_checker_rt::export!(thiscall, rw_008deab0(this: u32, base: u32, size: u32) -> u32 {
    unsafe {
        const CURSOR_OFF: u32 = 0x18;
        const SLOT_INDEX_OFF: u32 = 0x10;
        const LIMIT: u32 = 0x20_0000;
        const ROW: u32 = 0x10;
        const ALIGN_MASK: u32 = 0x0f;
        const USE_CURSOR: u32 = 0xffff_ffff;
        const ALLOC_COUNTER: u32 = 0x0117_5c48;
        let aligned = base
            .wrapping_add(ROW)
            .wrapping_add(base.wrapping_neg() & ALIGN_MASK);
        let mut size = size;
        if size == USE_CURSOR {
            let cursor = ((this + CURSOR_OFF) as *const u32).read_unaligned();
            let grown = aligned
                .wrapping_add(aligned.wrapping_neg() & ALIGN_MASK)
                .wrapping_add(cursor);
            ((this + CURSOR_OFF) as *mut u32).write_unaligned(grown);
            let ctr = lf_checker_rt::global::<u32>(ALLOC_COUNTER);
            ctr.write_unaligned(ctr.read_unaligned().wrapping_add(1));
            size = cursor;
        }
        if base.wrapping_add(size) >= LIMIT {
            return 0;
        }
        let slot = ((this + SLOT_INDEX_OFF) as *const u32).read_unaligned();
        let slot_ptr =
            ((this + slot.wrapping_mul(4)) as *const u32).read_unaligned();
        slot_ptr.wrapping_add(ROW).wrapping_add(size)
    }
});
