// original: 0x008d8380 slot_field_byte
/// Read an 8-bit field from a slot record: `base[(entry + a1) * 3][4]`.
export!(cdecl, rw_008d8380(a1: u32, a2: u32) -> u32 {
    unsafe {
        let row = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1).wrapping_mul(3);
        let base = *global::<u32>(0x0103E8D0);
        let addr = base.wrapping_add(row.wrapping_mul(8)).wrapping_add(0x04);
        (addr as *const u8).read() as u32
    }
});
