// original: 0x00a4f3c0 vehicle_clear_bit1 (proposed)

/// Clear bit 1 of the byte at `this + offset`, returning the resulting byte.
///
/// Reads the byte; when bit 0x02 is set it is cleared in al and only then
/// stored back, otherwise no write happens. al holds the new value either
/// way (the and runs on al itself). Thiscall, one stack word.
lf_checker_rt::export!(thiscall, rw_00a4f3c0(this: u32, offset: u32) -> u32 {
    unsafe {
        const FLAG: u8 = 0x02;
        let addr = this.wrapping_add(offset);
        let old = (addr as *const u8).read();
        let new = old & !FLAG;
        if old & FLAG != 0 {
            (addr as *mut u8).write(new);
        }
        new as u32
    }
});
