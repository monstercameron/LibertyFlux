// original: 0x00B835D0 row_insert_6
/// Insert a row into the first free slot of the six-row table.
///
/// A slot is free when its key word is -1 with four zero bytes at `+0x18`
/// and two zero bytes at `+0x30`. The insert stores `key`, converts four
/// floats from `vals` with truncation (out-of-range and NaN give
/// `0x80000000`, as `cvttss2si`), stores two presence bytes from `flags`,
/// and merges twelve mask bits from `mask` (a null mask sets them all).
/// Full table: nothing happens.
///
/// Original: 0x00B835D0 (thiscall, four stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_00B835D0(this: u32, key: u32, vals: u32, flags: u32, mask: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits((a as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn cvtt(x: f32) -> i32 {
            if x.is_nan() {
                return i32::MIN;
            }
            if x >= 2147483648.0 || x < -2147483648.0 {
                return i32::MIN;
            }
            x as i32
        }
        let mut row = 0u32;
        loop {
            if row >= 6 {
                return 0;
            }
            let base = this.wrapping_add(row * 4);
            if rd32(base) == 0xFFFF_FFFF
                && rd8(base + 0x18) == 0 && rd8(base + 0x19) == 0
                && rd8(base + 0x1A) == 0 && rd8(base + 0x1B) == 0
                && rd8(this + row * 2 + 0x30) == 0
                && rd8(this + row * 2 + 0x31) == 0
            {
                break;
            }
            row += 1;
        }
        if row == 0xFFFF_FFFF {
            return 0;
        }
        let base = this.wrapping_add(row * 4);
        wr32(base, key);
        let mut i = 0u32;
        while i < 4 {
            wr8(base + 0x18 + i, cvtt(rdf(vals + i * 4)) as u8);
            i += 1;
        }
        wr8(this + row * 2 + 0x30, (rd32(flags) != 0) as u8);
        wr8(this + row * 2 + 0x31, (rd32(flags + 4) != 0) as u8);
        let mut bit = 1u32;
        let mut i = 0u32;
        while i < 12 {
            let set = mask == 0 || rd32(mask + i * 4) != 0;
            let cur = rd32(base + 0x3C);
            wr32(base + 0x3C, if set { cur | bit } else { cur & !bit });
            bit = bit.rotate_left(1);
            i += 1;
        }
        0
    }
});
