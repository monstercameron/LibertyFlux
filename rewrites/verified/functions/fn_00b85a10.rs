// original: 0x00B85A10 registry_release
/// Release `key` from the entity registry, then clear its flags.
///
/// A clear flag byte means there is nothing to do. Otherwise every live
/// registry row referencing `key` is visited: a primary reference is
/// released (callee 1), a secondary one detaches with mode `0xA`/`0xC`
/// (callee 2), tertiary and quaternary ones detach silently (callees
/// 3-4). Both flag bytes for `key` clear and this registry's use count
/// drops. Keys at or above `0x36` take a report path this proof never
/// exercises.
///
/// Original: 0x00B85A10 (thiscall, one stack argument, no return value).
lf_checker_rt::export!(thiscall, rw_00B85A10(this: u32, key: u32) -> u32 {
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
        const FLAGA: u32 = 0x0167E2D8;
        const FLAGB: u32 = 0x0167E37C;
        const REG: u32 = 0x018B6F1C;
        const BIG: u32 = 0x36;
        if unsafe { (lf_checker_rt::global::<u8>(FLAGA + key)).read() } == 0 {
            return 0;
        }
        if unsafe { (lf_checker_rt::global::<u8>(FLAGB + key)).read() } == 0 {
            let reg = (lf_checker_rt::global::<u32>(REG)).read_unaligned();
            let mut left = rd32(reg + 8);
            if left != 0 {
                let flagbase = rd32(reg + 4);
                let base = rd32(reg);
                let stride = rd32(reg + 0x0C);
                loop {
                    left -= 1;
                    if rd8(flagbase.wrapping_add(left)) & 0x80 == 0 {
                        let row = base.wrapping_add(stride.wrapping_mul(left));
                        if row != 0 {
                            let p = rd32(row + 0x224);
                            if rd32(p + 0xD0) == key {
                                lf_checker_rt::callee_thiscall!(1, u32, row);
                            }
                            let q = rd32(row + 0x224);
                            if rd32(q + 0xD8) == key {
                                let mode = if rd8(row + 0xA60) == 2 { 0x0A } else { 0x0C };
                                lf_checker_rt::callee_stdcall!(2, u32, mode);
                            }
                            let r = rd32(row + 0x224);
                            if rd32(r + 0xD4) == key {
                                lf_checker_rt::callee_stdcall!(3, u32, 0);
                            }
                            let t = rd32(row + 0x224);
                            if rd32(t + 0xDC) == key {
                                lf_checker_rt::callee_stdcall!(4, u32, 7);
                            }
                        }
                    }
                    if left == 0 {
                        break;
                    }
                }
            }
        }
        if key >= BIG {
            return 0;
        }
        unsafe { (lf_checker_rt::global::<u8>(FLAGA + key)).write(0) }
        unsafe { (lf_checker_rt::global::<u8>(FLAGB + key)).write(0) }
        wr32(this, rd32(this).wrapping_sub(1));
        0
    }
});
