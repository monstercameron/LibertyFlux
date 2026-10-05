// original: 0x005ddba0 CTaskComplexMedicTreatInjuredPed::vf0

/// Delete a medic-treat task when the flag's low bit is set.
///
/// Runs the full destructor (no vtable stamp here), then returns
/// `this` unless bit 0 of `flag` is set, in which case the object is
/// returned to the task pool at `POOL`.
///
/// Original: 0x005ddba0 (thiscall).
lf_checker_rt::export!(thiscall, rw_005ddba0(this: u32, flag: u32) -> u32 {
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        const POOL: u32 = 0x167e2a0;
        const POOL_FLAGS: u32 = 0x04;
        const POOL_STRIDE: u32 = 0x0c;
        const POOL_LO: u32 = 0x10;
        const POOL_COUNT: u32 = 0x14;
        const FREE_BIT: u8 = 0x80;
        const BASE_DTOR: u32 = 1;
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this);
        if flag & 1 == 0 {
            return this;
        }
        // Return the object to the task pool: index = (this - base) / stride.
        let pool = rd32(lf_checker_rt::relocated(POOL));
        let base = rd32(pool);
        let flags = rd32(pool + POOL_FLAGS);
        let stride = rd32(pool + POOL_STRIDE);
        let num = (this.wrapping_sub(base) as i32) as i64;
        let idx = num.wrapping_div((stride as i32) as i64) as u32;
        let cell = flags.wrapping_add(idx) as *mut u8;
        cell.write(cell.read() | FREE_BIT);
        let lo = rd32(pool + POOL_LO);
        if (idx as i32) < (lo as i32) {
            wr32(pool + POOL_LO, idx);
        }
        let cnt = rd32(pool + POOL_COUNT);
        wr32(pool + POOL_COUNT, cnt.wrapping_sub(1));
        this
    }
});
