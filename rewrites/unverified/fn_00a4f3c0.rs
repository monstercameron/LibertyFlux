// original: 0x00a4f3c0 vehicle_clear_bit1 (proposed)

/// Clear bit 1 of the byte at `this + offset`, returning the byte's old value.
///
/// Reads the byte, clears bit 0x02 only when set (otherwise no write), and
/// leaves the old byte in al. Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00a4f3c0(this: u32, offset: u32) -> u32 {
    unsafe {
        const FLAG: u8 = 0x02;
        let addr = this.wrapping_add(offset);
        let old = (addr as *const u8).read();
        if old & FLAG != 0 {
            (addr as *mut u8).write(old & !FLAG);
        }
        old as u32
    }
});
