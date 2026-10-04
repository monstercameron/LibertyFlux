// original: 0x008d84e0 slot_state_is_3
/// Report whether a slot's state nibble reads 3.
///
/// Same record addressing as `streaming_is_slot_loaded`, testing for the
/// value 3 instead of 1.
export!(cdecl, rw_008d84e0(a1: u32, a2: u32) -> u32 {
    unsafe {
        let row = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1).wrapping_mul(3);
        let base = *global::<u32>(0x0103E8D0);
        let addr = base.wrapping_add(row.wrapping_mul(8)).wrapping_add(0x08);
        (((addr as *const u8).read() & 3) == 3) as u32
    }
});
