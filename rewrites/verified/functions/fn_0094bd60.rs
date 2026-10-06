// original: 0x0094BD60 DELETING_PED_SEQUENCE (merged)

/// Tear down a ped sequence player, by one of three paths.
///
/// When `arg0` is null (path C) callee 13 runs with `this` =
/// `ROW_TABLE + [POOL_COUNT] * ROW_STRIDE` and argument `arg1`, and its
/// answer is returned. Otherwise callee 0 resolves `arg0` against
/// `SEQ_TABLE` to an entry, and callee 1's lookup object is consulted:
/// when its flag byte at `+LOOKUP_FLAG` is clear, a second lookup runs
/// and callee 14 resolves its key at `+LOOKUP_KEY` to a second entry in
/// `esi` (otherwise `esi` stays 0). When the entry's `ACTIVE_BIT` in
/// `+ENTRY_FLAGS` is clear, or the two entries are identical, path B
/// runs: callee 7 fills a six-word scratch block (its `this`; stack
/// arguments 3, `arg1`, 0), callees 8 and 16 poke the sub-object at
/// `[entry + ENTRY_SUB] + SUB_BIAS`, callee 9 consumes the scratch block,
/// and unless its result is zero callee 10 yields a row index that must
/// be non-negative (a SIGNED comparison: `js` skips) before callee 11
/// runs with `this` = `WEIGHT_TABLE + index * 20`; callee 12's answer is
/// returned. Otherwise (path A) a non-zero alternate at `+ENTRY_ALT`
/// triggers an inner notify block (callees 2, 3, 4 with its low byte
/// sign-extended, 5, 15), and finally the virtual slot at `[arg1]`
/// (thiscall, argument 1) runs when `arg1` is non-null; the last callee
/// answer seen is returned.
///
/// Original: 0x0094BD60 (cdecl, three stack words).
lf_checker_rt::export!(cdecl, rw_0094BD60(arg0: u32, arg1: u32, arg2: u32) -> u32 {
    unsafe {
        const SEQ_TABLE: u32 = 0x018b6f1c;
        const LOOKUP_FLAG: u32 = 0x9e;
        const LOOKUP_KEY: u32 = 0xa0;
        const ENTRY_FLAGS: u32 = 0x260;
        const ACTIVE_BIT: u32 = 0x20000;
        const ENTRY_ALT: u32 = 0x6c;
        const ENTRY_SUB: u32 = 0x224;
        const SUB_BIAS: u32 = 0x84;
        const FIXED_THIS: u32 = 0x01935fa8;
        const MSG_A: u32 = 0x00e88f30;
        const MSG_B: u32 = 0x00e88fb8;
        const MSG_C: u32 = 0x00e88fa0;
        const MODE_THIS: u32 = 0x018e51e8;
        const WEIGHT_TABLE: u32 = 0x016683a0;
        const ROW_STRIDE: u32 = 20;
        const POOL_COUNT: u32 = 0x010475c4;
        const POOL_STRIDE: u32 = 0x6c;
        const ROW_TABLE: u32 = 0x0167f780;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if arg0 == 0 {
            let base = (lf_checker_rt::global::<u32>(POOL_COUNT))
                .read()
                .wrapping_mul(POOL_STRIDE)
                .wrapping_add(lf_checker_rt::relocated(ROW_TABLE));
            return lf_checker_rt::callee_thiscall!(13, u32, base, arg1);
        }
        let seq = (lf_checker_rt::global::<u32>(SEQ_TABLE)).read();
        let edi: u32 = lf_checker_rt::callee_thiscall!(0, u32, seq, arg0);
        let mut esi: u32 = 0;
        let probe: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        let mut last: u32 = probe;
        if ((probe.wrapping_add(LOOKUP_FLAG)) as *const u8).read() == 0 {
            esi = (lf_checker_rt::global::<u32>(SEQ_TABLE)).read();
            let probe2: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
            let key = rd32(probe2.wrapping_add(LOOKUP_KEY));
            esi = lf_checker_rt::callee_thiscall!(14, u32, esi, key);
            last = esi;
        }
        if rd32(edi.wrapping_add(ENTRY_FLAGS)) & ACTIVE_BIT == 0 || esi == edi {
            let mut scratch = [0u32; 6];
            let this = scratch.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(7, u32, this, 3, arg1, 0);
            let sub = rd32(edi.wrapping_add(ENTRY_SUB)).wrapping_add(SUB_BIAS);
            let _: u32 = lf_checker_rt::callee_thiscall!(8, u32, sub, 0x20);
            let _: u32 = lf_checker_rt::callee_thiscall!(16, u32, sub);
            let r: u32 = lf_checker_rt::callee_thiscall!(9, u32, sub, this, 1, 1);
            if r != 0 {
                let k: u32 = lf_checker_rt::callee_cdecl!(10, u32,);
                if (k as i32) >= 0 {
                    let row = k
                        .wrapping_mul(ROW_STRIDE)
                        .wrapping_add(lf_checker_rt::relocated(WEIGHT_TABLE));
                    let _: u32 = lf_checker_rt::callee_thiscall!(11, u32, row, edi, arg2, r);
                }
            }
            return lf_checker_rt::callee_thiscall!(12, u32, this);
        }
        if rd32(edi.wrapping_add(ENTRY_ALT)) != 0 {
            let fixed = lf_checker_rt::relocated(FIXED_THIS);
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fixed, 0);
            let _: u32 = lf_checker_rt::callee_cdecl!(
                3,
                u32,
                fixed,
                2,
                lf_checker_rt::relocated(MSG_A)
            );
            let _: u32 = lf_checker_rt::callee_thiscall!(2, u32, fixed, 1);
            esi = rd32(edi.wrapping_add(ENTRY_ALT));
            let m: u32 = lf_checker_rt::callee_thiscall!(4, u32, lf_checker_rt::relocated(MODE_THIS));
            let sx = ((m as u8) as i8) as i32 as u32;
            let t: u32 = lf_checker_rt::callee_cdecl!(5, u32, sx);
            last = lf_checker_rt::callee_cdecl!(
                15,
                u32,
                fixed,
                2,
                lf_checker_rt::relocated(MSG_B),
                lf_checker_rt::relocated(MSG_C),
                t
            );
        }
        if arg1 == 0 {
            return last;
        }
