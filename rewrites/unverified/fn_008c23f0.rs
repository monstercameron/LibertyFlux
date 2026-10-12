// original: 0x008C23F0 audio_config_snapshot

/// Snapshot the live audio configuration bytes into their shadow copies.
///
/// Copies the enable bytes and the level float from the live words to the
/// shadow words: `SRC_EN_A` to `DST_EN_A`, `SRC_EN_B` to `DST_EN_B`,
/// `SRC_LEVEL` to `DST_LEVEL`, `SRC_FLAG_A` to `DST_FLAG_A`, `SRC_FLAG_B`
/// to `DST_FLAG_B` and `SRC_FLAG_C` to `DST_FLAG_C`. Returns the copied
/// `SRC_FLAG_C` byte zero-extended. Leaf, cdecl, no arguments.
lf_checker_rt::export!(cdecl, rw_008C23F0() -> u32 {
    unsafe {
        const SRC_EN_A: u32 = 0x0103_0C11;
        const SRC_LEVEL: u32 = 0x0103_0C14;
        const DST_EN_A: u32 = 0x0103_0C68;
        const SRC_EN_B: u32 = 0x0103_0C12;
        const DST_EN_B: u32 = 0x0103_0C69;
        const SRC_FLAG_A: u32 = 0x0116_2519;
        const DST_FLAG_A: u32 = 0x0116_2530;
        const SRC_FLAG_B: u32 = 0x0116_251A;
        const DST_FLAG_B: u32 = 0x0128_32B4;
        const SRC_FLAG_C: u32 = 0x0116_251B;
        const DST_LEVEL: u32 = 0x0103_0C6C;
        const DST_FLAG_C: u32 = 0x0116_252F;
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (lf_checker_rt::relocated(a) as *const u8).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (lf_checker_rt::relocated(a) as *mut u8).write_unaligned(v) }
        }
        let en_a = rd8(SRC_EN_A);
        let level = (lf_checker_rt::relocated(SRC_LEVEL) as *const u32).read_unaligned();
        wr8(DST_EN_A, en_a);
        wr8(DST_EN_B, rd8(SRC_EN_B));
        wr8(DST_FLAG_A, rd8(SRC_FLAG_A));
        wr8(DST_FLAG_B, rd8(SRC_FLAG_B));
        let flag_c = rd8(SRC_FLAG_C);
        (lf_checker_rt::relocated(DST_LEVEL) as *mut u32).write_unaligned(level);
        wr8(DST_FLAG_C, flag_c);
        u32::from(flag_c)
    }
});
