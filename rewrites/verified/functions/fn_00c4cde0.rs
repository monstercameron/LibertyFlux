// original: 0x00C4CDE0 task_pool_entry_at_index (proposed)
/// Return the address of the indexed 0x68-byte task-pool entry, starting at
/// `this + 0x0c`. Index multiplication and pointer addition wrap as 32-bit
/// arithmetic. The method takes the pool base in ECX and one stack index,
/// cleans that word, and returns the computed address in EAX.
lf_checker_rt::export!(thiscall, rw_c4cde0(this: u32, index: u32) -> u32 {
    const ENTRY_STRIDE: u32 = 0x68;
    const ENTRIES: u32 = 0x0c;
    this.wrapping_add(ENTRIES).wrapping_add(index.wrapping_mul(ENTRY_STRIDE))
});
