// original: 0x009B5950 script_state_refresh_flags

/// Refresh four flag bytes on the active script object.
///
/// `this` points to a script state. Its current row index is stored at byte
/// offset `0x30`; each row is `0x1a0` bytes and holds an object pointer at
/// `+0x1f4`. Row flags at `+0x1f8`, `+0x1fb`, `+0x1f9`, and `+0x1fa` set
/// object bytes `+0xb4a`, `+0xb4b`, `+0xb48`, and `+0xb49`, respectively.
/// The row index and object pointer are re-read for each flag group, so
/// writes that alias the state remain observable. If the row's signedness-
/// independent 32-bit value at `+0x8c` equals seven, a global dword is copied
/// to `this+0x54`. The function uses the 32-bit thiscall layout and returns no
/// meaningful value.
lf_checker_rt::export!(thiscall, rw_009b5950(this: u32) -> u32 {
    unsafe {
        const ROW_STRIDE: u32 = 0x1a0;
        const CURRENT_ROW: u32 = 0x30;
        const ROW_OBJECT: u32 = 0x1f4;
        const GLOBAL_VALUE: u32 = 0x0117_35b4;
        const STATE_VALUE: u32 = 0x54;
        const OBJECT_FLAGS: [u32; 4] = [0x1f8, 0x1fb, 0x1f9, 0x1fa];
        const OBJECT_FLAG_DESTS: [u32; 4] = [0xb4a, 0xb4b, 0xb48, 0xb49];

        #[inline(always)]
        unsafe fn read_u32(address: u32) -> u32 {
            unsafe { (address as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn read_u8(address: u32) -> u8 {
            unsafe { (address as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn write_u8(address: u32, value: u8) {
            unsafe { (address as *mut u8).write(value) }
        }

        for slot in 0..OBJECT_FLAGS.len() {
            let row_index = read_u32(this.wrapping_add(CURRENT_ROW));
            let row_offset = row_index.wrapping_mul(ROW_STRIDE);
            if slot == 0 {
                let object = read_u32(this.wrapping_add(row_offset).wrapping_add(ROW_OBJECT));
                if object == 0 {
                    continue;
                }
                if read_u8(this.wrapping_add(row_offset).wrapping_add(OBJECT_FLAGS[slot])) == 0 {
                    continue;
                }
                write_u8(object.wrapping_add(OBJECT_FLAG_DESTS[slot]), 1);
            } else if read_u8(this.wrapping_add(row_offset).wrapping_add(OBJECT_FLAGS[slot])) != 0 {
                let object = read_u32(this.wrapping_add(row_offset).wrapping_add(ROW_OBJECT));
                write_u8(object.wrapping_add(OBJECT_FLAG_DESTS[slot]), 1);
            }
        }

        let row_index = read_u32(this.wrapping_add(CURRENT_ROW));
        let row_offset = row_index.wrapping_mul(ROW_STRIDE);
        if read_u32(this.wrapping_add(row_offset).wrapping_add(0x8c)) == 7 {
            let value = lf_checker_rt::global::<u32>(GLOBAL_VALUE).read_unaligned();
            (this.wrapping_add(STATE_VALUE) as *mut u32).write_unaligned(value);
        }
    }
    0
});
