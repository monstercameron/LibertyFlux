// original: 0x0088A540 stream_resource_mark_flag (proposed)

/// Set flag bit 2 on one byte of a resource group's slot map.
///
/// `this` points to the resource manager whose table base is at `+0xe8`.
/// `slot` selects a group (each group spans 0x6f40 bytes) and `index` a byte
/// within it; bit 2 of that byte is set, other bits kept. Returns `index`
/// (the original leaves it in eax).
///
/// Original: 0x0088A540 (thiscall: `this` in ecx, two stack words).
lf_checker_rt::export!(thiscall, rw_0088a540(this: u32, slot: u32, index: u32) -> u32 {
    unsafe {
        const GROUP_STRIDE: u32 = 0x6f40;
        const TABLE_BASE: u32 = 0xe8;
        const FLAG_BIT: u8 = 0x04;

        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let cell = base.wrapping_add(slot.wrapping_mul(GROUP_STRIDE)).wrapping_add(index);
        let old = (cell as *const u8).read();
        (cell as *mut u8).write(old | FLAG_BIT);
        index
    }
});
