// original: 0x00c72a70 ped_task_params_init (proposed)

/// Initialise a ped task parameter block to its default state.
///
/// `this` points to the block (at least 0x104 bytes). The routine writes a
/// magic word at `+0x10`, zeroes most fields, and sets four defaults: two
/// unit scales at `+0xD8`/`+0xDC` (1.0), a range at `+0xE0` (30.0) and a count
/// at `+0xE8` (10000). Untouched: the header at `+0x00..0x10`, the words at
/// `+0x18`, `+0x1C`, `+0x2C`, the bytes at `+0x41..0x43`, the words at `+0x88`,
/// `+0x98`, `+0xA0` and the bytes at `+0xF5..0xF7`; those keep whatever the
/// caller left there.
///
/// Original: 0x00C72A70 (thiscall, no stack arguments; returns `this`).
lf_checker_rt::export!(thiscall, rw_00C72A70(this: u32) -> u32 {
    unsafe {
        const MAGIC: u32 = 0xC479_FF5C;
        const ONE: u32 = 0x3F80_0000;
        const RANGE: u32 = 0x41F0_0000; // 30.0
        const COUNT: u32 = 0x2710; // 10000

        #[inline(always)]
        unsafe fn w32(base: u32, off: u32, val: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(val) }
        }
        #[inline(always)]
        unsafe fn w8(base: u32, off: u32, val: u8) {
            unsafe { ((base + off) as *mut u8).write(val) }
        }
        /// Zero `n` consecutive words starting at `off`.
        #[inline(always)]
        unsafe fn zero_run(base: u32, off: u32, n: u32) {
            unsafe {
                let mut i = 0u32;
                while i < n {
                    w32(base, off + i * 4, 0);
                    i += 1;
                }
            }
        }

        w32(this, 0x10, MAGIC);
        w32(this, 0x14, 0);
        zero_run(this, 0x20, 3);
        zero_run(this, 0x30, 4);
        w8(this, 0x40, 0);
        zero_run(this, 0x44, 8);
        zero_run(this, 0x60, 8);
        zero_run(this, 0x80, 2);
        w32(this, 0x8C, 0);
        zero_run(this, 0x90, 2);
        w32(this, 0x9C, 0);
        zero_run(this, 0xA4, 13);
        w32(this, 0xD8, ONE);
        w32(this, 0xDC, ONE);
        w32(this, 0xE0, RANGE);
        w32(this, 0xE4, 0);
        w32(this, 0xE8, COUNT);
        w32(this, 0xEC, 0);
        w32(this, 0xF0, 0);
        w8(this, 0xF4, 0);
        zero_run(this, 0xF8, 3);
        this
    }
});
