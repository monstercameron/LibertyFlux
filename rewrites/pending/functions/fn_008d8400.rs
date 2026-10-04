// original: 0x008d8400 Streaming_IsSlotLoaded
/// Streaming `IsSlotLoaded`: true when the slot state nibble reads 1.
///
/// Reads the byte at record offset +8, keeps bits 0..1, and reports whether
/// the slot is in state 1 (loaded).
export!(cdecl, rw_008d8400(a1: u32, a2: u32) -> u32 {
    unsafe {
        let row = (*global::<u32>(0x013053A8)
            .byte_add(a2.wrapping_mul(100) as usize)).wrapping_add(a1).wrapping_mul(3);
        let base = *global::<u32>(0x0103E8D0);
        let addr = base.wrapping_add(row.wrapping_mul(8)).wrapping_add(0x08);
        (((addr as *const u8).read() & 3) == 1) as u32
    }
});
