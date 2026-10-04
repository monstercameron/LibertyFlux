// original: 0x00663370 sn_migrate_collect (proposed)

/// Collect the live migration rows into the task's row array.
///
/// `this` is the task. Thirty-two candidate rows are initialised and then
/// either copied from the task's row array (when it already holds rows) or
/// filled through the fill hook; each candidate whose key lookup finds a
/// live entry is converted through the copy hook into a result row. With no
/// live candidate the manager's current row is used instead. The result rows
/// are then looked up once more and the surviving ones are stored back into
/// the task's row array with cleared flags, the manager's current row is
/// appended unless one of the stored rows already carries its keys, and the
/// row count is updated. Under a zeroed scratch fill the three padding word
/// pairs of every row read as zero on both sides.
/// Original: 0x00663370 (thiscall, no stack arguments, no result).
lf_checker_rt::export!(thiscall, rw_00663370(this: u32) -> u32 {
    unsafe { sn_migrate_collect(this, 0) }
});

lf_checker_rt::export!(thiscall, mut_00663370(this: u32) -> u32 {
    unsafe { sn_migrate_collect(this, 1) }
});

lf_checker_rt::export!(thiscall, mutstruct_00663370(this: u32) -> u32 {
    unsafe { sn_migrate_collect(this, 2) }
});

unsafe fn sn_migrate_collect(this: u32, mode: u32) -> u32 {
    unsafe {
        const ROWS: usize = 32;
        const SRC_ROWS: usize = 40;
        const WORDS: usize = 16;
        /// Initialised row: zero payload, -1 markers, zero tags. The upper
        /// halves of words 6, 8 and 10 are padding the original never
        /// writes; under a zeroed scratch fill they read as zero.
        const INIT: [u32; WORDS] =
            [0, 0, 0, 0, 0, 0xffff_ffff, 0, 0xffff_ffff, 0, 0xffff_ffff, 0, 0, 0, 0, 0xffff_ffff, 0xffff_ffff];
        const C_FILL: u32 = 2;
        const C_LOOKUP: u32 = 3;
        const C_COPY: u32 = 4;
        const C_COOKIE: u32 = 5;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        /// Copy a row while leaving the padding halves of words 6, 8 and 10
        /// untouched at the destination.
        #[inline(always)]
        unsafe fn copy_row(dst: *mut u32, src: *const u32) {
            unsafe {
                for w in [0usize, 1, 2, 3, 4, 5, 7, 9, 11, 12, 13, 14, 15] {
                    dst.add(w).write_unaligned(src.add(w).read_unaligned());
                }
                for w in [6usize, 8, 10] {
                    (dst.add(w) as *mut u16)
                        .write_unaligned((src.add(w) as *const u16).read_unaligned());
                }
            }
        }

        let mgr = rd32(this + 0x60);
        // The stack-probe helper runs natively on the original side (its
        // stub hangs the worker); it is transparent, so the rewrite omits it.
        let mut src = [[0u32; WORDS]; SRC_ROWS];
        let mut dst = [[0u32; WORDS]; ROWS];
        for r in src.iter_mut().take(ROWS) {
            *r = INIT;
        }
        for r in dst.iter_mut() {
            *r = INIT;
        }
        let count: u32;
        if rd32(this + 0x904) as i32 > 0 {
            let from = this + 0xA0;
            for i in 0..0x200usize {
                wr32(src.as_mut_ptr() as u32 + (i * 4) as u32, rd32(from + (i * 4) as u32));
            }
            count = 0;
        } else {
            count = lf_checker_rt::callee_thiscall!(C_FILL, u32, mgr, src.as_mut_ptr() as u32);
        }
        if mode == 2 {
            src[0][10] ^= 0x9E37_79B9;
        }
        let mut dcnt = 0u32;
        if (count as i32) > 0 {
            let mut di = 0usize;
            let mut si = 0u32;
            while si < count {
                if dcnt >= ROWS as u32 {
                    break;
                }
                let (k0, k1) = (src[si as usize][14], src[si as usize][15]);
                let found: u32 =
                    lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, mgr, k0, k1);
                let mut live = false;
                if found != 0 {
                    if rd32(found) as i32 >= 0 {
                        live = true;
                    } else if ((found + 0x78) as *const u8).read() & 1 != 0 {
                        live = true;
                    }
                }
                if live {
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        C_COPY, u32, dst[di].as_mut_ptr() as u32, src[si as usize].as_ptr() as u32
                    );
                    dst[di][14] = k0;
                    dst[di][15] = k1;
                    dcnt += 1;
                    di += 1;
                }
                si += 1;
            }
        }
        if dcnt == 0 {
            copy_row(dst[0].as_mut_ptr(), (mgr + 0xBB8) as *const u32);
            dcnt = 1;
        }
        let mut found_flag = false;
        wr32(this + 0x904, 0);
        if mode != 1 {
            wr32(this + 0x1660, 0);
        }
        wr32(this + 0x165C, 0);
        wr32(this + 0x1658, 0);
        if dcnt != 0 {
            let mut idx = 0u32;
            while idx < dcnt {
                if idx >= ROWS as u32 {
                    break;
                }
                let (k0, k1) = (dst[idx as usize][14], dst[idx as usize][15]);
                let found: u32 =
                    lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, mgr, k0, k1);
                let skip = found != 0 && ((found + 0x78) as *const u8).read() & 3 == 0;
                if !skip {
                    let at = rd32(this + 0x904);
                    copy_row(
                        (this + 0xA0 + at * 64) as *mut u32,
                        dst[idx as usize].as_ptr(),
                    );
                    wr32(this + 0xA0 + at * 64 + 0x38, k0);
                    wr32(this + 0xA0 + at * 64 + 0x3C, k1);
                    wr8_at(this + 0x8E0 + at, 0);
                    if !found_flag
                        && k0 == rd32(mgr + 0xBB8 + 0x38)
                        && k1 == rd32(mgr + 0xBB8 + 0x3C)
                    {
                        found_flag = true;
                    }
                    wr32(this + 0x904, at + 1);
                }
                idx += 1;
            }
        }
        if dcnt == 0 || !found_flag {
            let at = rd32(this + 0x904);
            copy_row((this + 0xA0 + at * 64) as *mut u32, (mgr + 0xBB8) as *const u32);
            wr32(this + 0x904, at + 1);
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        0
    }
}

/// Byte store helper for the row-flag array.
#[inline(always)]
unsafe fn wr8_at(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}
