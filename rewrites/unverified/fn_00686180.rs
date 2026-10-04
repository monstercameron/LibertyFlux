// original: 0x00686180 dof_pair_copy_quat (proposed)

/// Resolve two frames by key and refresh their quaternions from a matrix
/// object.
///
/// `r1 = lookup(key0, key2)` and `r2 = lookup(key1, key2)`. `mtx` carries a
/// source quaternion at `+0x30`..`+0x3c`. When the low byte of `flag` is
/// non-zero, each non-null result is refreshed on its own: `r1` gets the
/// source quaternion copied to `+0x10`..`+0x1c`, `r2` gets the
/// matrix-to-quaternion conversion of `mtx`; both have flag bit `0x10` at
/// `+0x04` cleared. The result is 1 unless both lookups missed. When the
/// flag byte is zero, both lookups must hit or the result is 0 with no
/// writes; otherwise both refreshes happen and the result is 1. `this` is
/// unused.
///
/// Original: 0x00686180 (thiscall, five stack words, full-word 0/1 result).
lf_checker_rt::export!(thiscall, rw_00686180(_this: u32, key0: u32, key1: u32, key2: u32, mtx: u32, flag: u32) -> u32 {
    unsafe {
        const LOOKUP1: u32 = 1;
        const LOOKUP2: u32 = 2;
        const MAT2QUAT: u32 = 3;
        const FLAGS_OFF: u32 = 0x04;
        const KEEP_MASK: u8 = 0xef;
        const DST_QUAT: u32 = 0x10;
        const SRC_QUAT: u32 = 0x30;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn clear_skip(frame: u32) {
            unsafe {
                let f = ((frame + FLAGS_OFF) as *mut u8).read();
                ((frame + FLAGS_OFF) as *mut u8).write(f & KEEP_MASK);
            }
        }
        #[inline(always)]
        unsafe fn copy_src_quat(mtx: u32, dst: u32) {
            unsafe {
                for i in 0..4u32 {
                    wr32(dst + DST_QUAT + i * 4, rd32(mtx + SRC_QUAT + i * 4));
                }
            }
        }
        #[inline(always)]
        unsafe fn convert_quat(mtx: u32, dst: u32) {
            unsafe {
                let mut buf = [0u32; 4];
                let _ans: u32 = lf_checker_rt::callee_thiscall!(
                    MAT2QUAT,
                    u32,
                    buf.as_mut_ptr() as u32,
                    mtx
                );
                for i in 0..4u32 {
                    wr32(dst + DST_QUAT + i * 4, buf[i as usize]);
                }
            }
        }

        let r1: u32 = lf_checker_rt::callee_stdcall!(LOOKUP1, u32, key0, key2);
        let r2: u32 = lf_checker_rt::callee_stdcall!(LOOKUP2, u32, key1, key2);
        if (flag as u8) != 0 {
            if r1 != 0 {
                unsafe {
                    copy_src_quat(mtx, r1);
                    clear_skip(r1);
                }
            }
            if r2 != 0 {
                unsafe {
                    convert_quat(mtx, r2);
                    clear_skip(r2);
                }
            }
            if r1 != 0 || r2 != 0 {
                1
            } else {
                0
            }
        } else {
            if r1 == 0 || r2 == 0 {
                return 0;
            }
            unsafe {
                copy_src_quat(mtx, r1);
                clear_skip(r1);
                convert_quat(mtx, r2);
                clear_skip(r2);
            }
            1
        }
    }
});
