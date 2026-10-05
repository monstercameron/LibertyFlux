// original: 0x00AC6810 stream_ensure_state_applied (proposed)

/// Copy the pending vectors into place and lazily apply both state blocks.
///
/// The original copies the pending vector and its mirror into the live
/// slots, then for each of the two blocks checks its done flag bit: when
/// clear it copies the block's constant vector into place, sets the bit and
/// applies the block through its applier callee (cdecl, no arguments). It
/// then refreshes the low and middle parameter slots with 1.0 through their
/// setters and clears the dirty byte. No value is returned.
lf_checker_rt::export!(cdecl, rw_00AC6810() -> u32 {
    unsafe {
        const PENDING: u32 = 0x0154E0F0;
        const LIVE0: u32 = 0x0154E070;
        const MIRROR: u32 = 0x0103F2B0;
        const LIVE1: u32 = 0x0154E080;
        const FLAGS: u32 = 0x0154E110;
        const CONST0: u32 = 0x00EA6060;
        const BLOCK0: u32 = 0x0154E100;
        const CONST1: u32 = 0x00EA6070;
        const BLOCK1: u32 = 0x0154E120;
        const DIRTY: u32 = 0x0103F254;
        const ONE: u32 = 0x3f80_0000;
        const APPLY0: u32 = 1;
        const APPLY1: u32 = 2;
        const SET_LO: u32 = 3;
        const SET_MID: u32 = 4;
        #[inline(always)]
        unsafe fn copy16(dst: u32, src: u32) {
            unsafe {
                for i in 0..4u32 {
                    let v = (src.wrapping_add(i * 4) as *const u32).read_unaligned();
                    (dst.wrapping_add(i * 4) as *mut u32).write_unaligned(v);
                }
            }
        }
        unsafe {
            copy16(lf_checker_rt::relocated(LIVE0), lf_checker_rt::relocated(PENDING));
            copy16(lf_checker_rt::relocated(LIVE1), lf_checker_rt::relocated(MIRROR));
            let mut fl = (lf_checker_rt::relocated(FLAGS) as *const u32).read_unaligned();
            if fl & 1 == 0 {
                copy16(lf_checker_rt::relocated(BLOCK0), lf_checker_rt::relocated(CONST0));
                fl |= 1;
                (lf_checker_rt::relocated(FLAGS) as *mut u32).write_unaligned(fl);
            }
            lf_checker_rt::callee_cdecl!(APPLY0, u32, lf_checker_rt::relocated(BLOCK0));
            fl = (lf_checker_rt::relocated(FLAGS) as *const u32).read_unaligned();
            if fl & 2 == 0 {
                copy16(lf_checker_rt::relocated(BLOCK1), lf_checker_rt::relocated(CONST1));
                fl |= 2;
                (lf_checker_rt::relocated(FLAGS) as *mut u32).write_unaligned(fl);
            }
            lf_checker_rt::callee_cdecl!(APPLY1, u32, lf_checker_rt::relocated(BLOCK1));
            lf_checker_rt::callee_cdecl!(SET_LO, u32, ONE);
            lf_checker_rt::callee_cdecl!(SET_MID, u32, ONE);
            (lf_checker_rt::relocated(DIRTY) as *mut u8).write(0);
            0
        }
    }
});
