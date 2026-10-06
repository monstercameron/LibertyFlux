// original: 0x00890BB0 audsound_init_submix_table
/// Forwards six arguments to the mixer init, then builds the submix table.
///
/// Calls the mixer initializer (thiscall on the global mixer object, six
/// stack words: the six arguments in order) and clears the global started
/// byte. Then for each of the 24 submix indices `b` (1-based value `b + 1`
/// pushed as a full word, whose upper bytes repeat the caller's entry ECX
/// and are therefore compared low-byte-only): calls the nine loader callees
/// (cdecl, one stack word each) in fixed order and stores each answer into
/// its own global array at index `b` (loader `k` feeds array `k`). The count
/// compares unsigned. Returns the ninth loader's last answer.
/// Original: 0x00890BB0 (cdecl, six stack words; true size 198 bytes, the
/// batch list says 196).
export!(cdecl, rw_00890BB0(a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    unsafe {
        const MIXER_INIT: u32 = 1;
        const MIXER_G: u32 = 0x115d8a0;
        const STARTED_G: u32 = 0x1030393;
        const ARRAYS: [u32; 9] = [
            0x115d538, 0x115d598, 0x115d658, 0x115d5f8, 0x115d6b8, 0x115d718, 0x115d778,
            0x115d7d8, 0x115d838,
        ];
        const N: u32 = 24;
        let mixer = lf_checker_rt::relocated(MIXER_G);
        let _: u32 = callee_thiscall!(MIXER_INIT, u32, mixer, a1, a2, a3, a4, a5, a6);
        *lf_checker_rt::global::<u8>(STARTED_G) = 0;
        let mut ans = 0u32;
        let mut b = 0u32;
        while b < N {
            let v = b.wrapping_add(1);
            ans = callee_cdecl!(2, u32, v);
            store_at(ARRAYS[0], b, ans);
            ans = callee_cdecl!(3, u32, v);
            store_at(ARRAYS[1], b, ans);
            ans = callee_cdecl!(4, u32, v);
            store_at(ARRAYS[2], b, ans);
            ans = callee_cdecl!(5, u32, v);
            store_at(ARRAYS[3], b, ans);
            ans = callee_cdecl!(6, u32, v);
            store_at(ARRAYS[4], b, ans);
            ans = callee_cdecl!(7, u32, v);
            store_at(ARRAYS[5], b, ans);
            ans = callee_cdecl!(8, u32, v);
            store_at(ARRAYS[6], b, ans);
            ans = callee_cdecl!(9, u32, v);
            store_at(ARRAYS[7], b, ans);
            ans = callee_cdecl!(10, u32, v);
            store_at(ARRAYS[8], b, ans);
            b = b.wrapping_add(1);
        }
        ans
    }
});

/// Store `v` into global dword array `base` at index `i`.
#[inline(always)]
unsafe fn store_at(base: u32, i: u32, v: u32) {
    unsafe {
        let at = lf_checker_rt::relocated(base).wrapping_add(i.wrapping_mul(4));
        (at as *mut u32).write_unaligned(v);
    }
}
