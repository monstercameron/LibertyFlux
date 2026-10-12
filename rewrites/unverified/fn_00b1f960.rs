// original: 0x00B1F960 cell_grid_a_clear
/// Clear a 32-by-32 grid of 48-byte cell records beginning at byte offset
/// 0x250 from the ECX base pointer. Every byte in the contiguous grid is
/// written to zero. The original leaves EAX pointing two bytes beyond the
/// cleared range, which is returned; it has no stack arguments or callees.
lf_checker_rt::export!(thiscall, rw_00b1f960(object: u32) -> u32 {
    const GRID_START: u32 = 0x250;
    const GRID_BYTES: usize = 0xC000;
    const RETURN_OFFSET: u32 = 0xC252;

    unsafe {
        core::ptr::write_bytes(object.wrapping_add(GRID_START) as *mut u8, 0, GRID_BYTES);
    }
    object.wrapping_add(RETURN_OFFSET)
});
