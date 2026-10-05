// original: 0x00c6a4f0 stream_release_three_groups (proposed)

/// Release every handle in the three groups at +0x404, +0x504, +0x604.
///
/// Same top-down release loop as the four-bank variant, over the three
/// group offsets. A non-positive count skips its group.
///
/// Original: thiscall with no stack words, nine call sites, reads one
/// global.
lf_checker_rt::export!(thiscall, rw_00c6a4f0(this: u32) -> u32 {
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
        for off in [0x404u32, 0x504, 0x604] {
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
