// original: 0x0088A920 stream_resource_handle_is_valid (proposed)

/// Whether a voice handle currently resolves to a live slot.
///
/// `this` points to the handle: slot index byte at `+0x04` (0xff means none)
/// and group byte at `+0x40`. The global at 0x0115D988 is the group table
/// base, the one at 0x0115D968 the slot stride. Each half of the function
/// resolves the same slot pointer (`stride * slot + group_table[group]`)
/// through a helper that maps index 0xff to null; that null branch is dead
/// (the outer 0xff check returns first) and is kept only to mirror the
/// original's shape. Returns 1 when flag bit 6 of the slot's byte at `+0xe8`
/// is set, else 1 when the word at slot `+0xd8` is nonzero, else 0.
///
/// Original: 0x0088A920 (thiscall, `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_0088a920(this: u32) -> u32 {
    unsafe {
        const SLOT_IDX: u32 = 0x04;
        const GROUP_IDX: u32 = 0x40;
        const STRIDE_SLOT: u32 = 0x0115_d968;
        const GROUPS_SLOT: u32 = 0x0115_d988;
        const GROUP_STRIDE: u32 = 0x6f40;
        const GROUP_TABLE_OFF: u32 = 0x6f14;
        const FLAG_OFF: u32 = 0xe8;
        const LIVE_OFF: u32 = 0xd8;
        const NO_SLOT: u32 = 0xff;
        const LIVE_BIT: u8 = 0x40;

        unsafe fn slot_ptr(this: u32, idx: u32, stride: u32, groups: u32) -> u32 {
            if idx == 0xff {
                return 0;
            }
            let group = ((this + 0x40) as *const u8).read() as u32;
            stride
                .wrapping_mul(idx)
                .wrapping_add(
                    ((group
                        .wrapping_mul(0x6f40)
                        .wrapping_add(groups)
                        .wrapping_add(0x6f14))
                        as *const u32)
                        .read_unaligned(),
                )
        }

        let stride = lf_checker_rt::global::<u32>(STRIDE_SLOT).read();
        let groups = lf_checker_rt::global::<u32>(GROUPS_SLOT).read();
        let slot = ((this + SLOT_IDX) as *const u8).read() as u32;
        if slot == NO_SLOT {
            return 0;
        }
        let ptr = slot_ptr(this, slot, stride, groups);
        if ((ptr.wrapping_add(FLAG_OFF)) as *const u8).read() & LIVE_BIT != 0 {
            return 1;
        }
        // Second resolution of the same slot; its 0xff checks are dead too.
        if slot == NO_SLOT {
            return 0;
        }
        let ptr2 = slot_ptr(this, slot, stride, groups);
        if ((ptr2.wrapping_add(LIVE_OFF)) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        1
    }
});
