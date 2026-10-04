// original: 0x008d8360 slot_field_word
/// Read a 16-bit field from a slot record: `base[(entry + a1) * 3][0xE]`.
///
/// Indexes the slot table by `a2`, scales the combined row by the 24-byte
/// record stride (3 * 8 via addressing), and zero-extends the word at +0xE.
export!(cdecl, rw_008d8360(a1: u32, a2: u32) -> u32 {
    unsafe {
        let row = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1).wrapping_mul(3);
        let base = *global::<u32>(0x0103E8D0);
        let addr = base.wrapping_add(row.wrapping_mul(8)).wrapping_add(0x0E);
        (addr as *const u16).read_unaligned() as u32
    }
});
