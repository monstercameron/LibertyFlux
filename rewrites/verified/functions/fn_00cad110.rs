// original: 0x00cad110 task_strings_equal_2 (proposed)
/// Compare two pairs of NUL-terminated strings for equality.
///
/// Compares the bytes at `this` with those at `other`, then the bytes at
/// `this+0x20` with those at `other+0x20`, as unsigned bytes up to and
/// including the terminator. Returns 1 when both pairs match, else 0.
/// The original compares two bytes per iteration; the observable result is a
/// plain string equality. Original is thiscall(`this`, `other`).
lf_checker_rt::export!(thiscall, rw_00cad110(this: u32, other: u32) -> u32 {
    unsafe {
        const SECOND: u32 = 0x20;
        #[inline(always)]
        unsafe fn streq(a: u32, b: u32) -> bool {
            unsafe {
                let mut i = 0u32;
                loop {
                    let x = ((a.wrapping_add(i)) as *const u8).read();
                    let y = ((b.wrapping_add(i)) as *const u8).read();
                    if x != y {
                        return false;
                    }
                    if x == 0 {
                        return true;
                    }
                    i = i.wrapping_add(1);
                }
            }
        }
        if !streq(this, other) {
            return 0;
        }
        if !streq(this.wrapping_add(SECOND), other.wrapping_add(SECOND)) {
            return 0;
        }
        1
    }
});
