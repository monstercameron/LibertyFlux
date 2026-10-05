// original: 0x00a4f830 vehicle_slot_index_addr (proposed)

/// Address of the `index`-th fixed-size slot in this object's slot table.
///
/// The table of 256 entries starts at `this + 0x100`; each entry is 0xe0
/// bytes. Pure address computation, no memory access. Thiscall, one stack
/// word, returns the address in eax.
lf_checker_rt::export!(thiscall, rw_00a4f830(this: u32, index: u32) -> u32 {
    const SLOTS_BASE: u32 = 0x100;
    const SLOT_STRIDE: u32 = 0xe0;
    this.wrapping_add(SLOTS_BASE)
        .wrapping_add(index.wrapping_mul(SLOT_STRIDE))
});
