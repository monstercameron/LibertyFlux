// original: 0x005f3230 GetStreamArrayEntry

/// Returns the selected stream-table entry from the pointer at `this + 0xb8`
/// and index at `this + 0xc4`. The all-ones index is a sentinel that returns
/// zero without reading the table.
lf_checker_rt::export!(thiscall, rw_005f3230(this: u32) -> u32 {
    const ENTRY_TABLE: u32 = 0xb8;
    const ENTRY_INDEX: u32 = 0xc4;
    let index = unsafe { (this.wrapping_add(ENTRY_INDEX) as *const u32).read_unaligned() };
    if index == u32::MAX {
        return 0;
    }
    let entries = unsafe { (this.wrapping_add(ENTRY_TABLE) as *const u32).read_unaligned() };
    let slot = entries.wrapping_add(index.wrapping_mul(4));
    unsafe { (slot as *const u32).read_unaligned() }
});
