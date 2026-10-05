// original: 0x00c6a420 stream_release_four_banks (proposed)

/// Release every handle in the four banks at +0, +0x100, +0x200, +0x300.
///
/// Each bank reports its handle count; handles are fetched from the
/// top down and each is released with the lock word from its global.
/// A non-positive count skips its bank.
///
/// Original: thiscall with no stack words, twelve call sites, reads
/// one global.
lf_checker_rt::export!(thiscall, rw_00c6a420(this: u32) -> u32 {
    unsafe {
        const LOCK: u32 = 0x012B_4138;
        const COUNT: u32 = 1;
        const FETCH: u32 = 2;
        const RELEASE: u32 = 3;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        let lock = rd32(lf_checker_rt::relocated(LOCK));
        for off in [0u32, 0x100, 0x200, 0x300] {
            let bank = this.wrapping_add(off);
            let n: u32 = lf_checker_rt::callee_thiscall!(COUNT, u32, bank);
            let mut i = (n as i32).wrapping_sub(1);
            while i >= 0 {
                let h: u32 =
                    lf_checker_rt::callee_thiscall!(FETCH, u32, bank, i as u32);
                lf_checker_rt::callee_cdecl!(RELEASE, u32, h, lock);
                i = i.wrapping_sub(1);
            }
        }
        0
    }
});
