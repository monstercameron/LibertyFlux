// original: 0x00b77130 task_param_apply_indexed (proposed)

/// Apply one indexed task's parameters: push a constant, a summed float and
/// three words into per-kind setters, then copy a 40-byte block.
///
/// `this` points to the owner: entry pointers at `ENTRY_BASE + idx * 4` (at
/// most `ENTRY_COUNT` of them) and a float bias at `BIAS_OFF`. `idx`
/// selects the entry; out-of-range or null selects nothing and the function
/// returns. `farg` is added to the bias for the second setter; `w2`, `w3`
/// and `p4` go to the fourth, fifth and third setters; `buf` supplies the
/// 40 bytes copied at the end.
///
/// Each entry carries two tag bytes, `TAG_B` at `+0x04` and `TAG_C` at
/// `+0x40`. A `TAG_B` of `NO_TAG` (0xff) means "no object": the setters run
/// with a null object and the final copy is skipped. Otherwise the object is
/// `TABLE[TAG_C * ROW + HDR] + K * TAG_B` (wrapping) where `K` and `TABLE`
/// come from globals. The five setters all take (object, one word); the
/// third takes (entry, p4) instead. The final getter takes (object) and its
/// answer is the destination of the 40-byte copy from `buf`; the original
/// stages the block through its own frame, so a source that aliases the
/// destination still copies the pre-call bytes, which this rewrite matches
/// by snapshotting first.
///
/// The float add runs in the original's operand order (bias first).
/// The function returns nothing meaningful (early paths leave incoming
/// `eax` untouched), so the return channel is unchecked.
///
/// Original: 0x00b77130 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00b77130(
    this: u32,
    idx: u32,
    farg: u32,
    w2: u32,
    w3: u32,
    p4: u32,
    buf: u32,
) -> u32 {
    unsafe {
        const ENTRY_BASE: u32 = 0x968;
        const ENTRY_COUNT: u32 = 9;
        const BIAS_OFF: u32 = 0xbb8;
        const TAG_B: u32 = 0x04;
        const TAG_C: u32 = 0x40;
        const NO_TAG: u32 = 0xff;
        const ROW: u32 = 0x6f40;
        const HDR: u32 = 0x6f14;
        const G_K: u32 = 0x0115d968;
        const G_TABLE: u32 = 0x0115d988;
        const COPY_WORDS: u32 = 10;
        const SET_CONST: u32 = 1;
        const SET_FLOAT: u32 = 2;
        const SET_ENTRY: u32 = 3;
        const SET_W2: u32 = 4;
        const SET_W3: u32 = 5;
        const GET_DEST: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u32 {
            unsafe { (a as *const u8).read() as u32 }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        /// Object for an entry: 0 when untagged, else the table lookup.
        #[inline(always)]
        unsafe fn object(entry: u32) -> u32 {
            unsafe {
                let b = rd8(entry.wrapping_add(TAG_B));
                if b == NO_TAG {
                    return 0;
                }
                let c = rd8(entry.wrapping_add(TAG_C));
                let k = rd32(lf_checker_rt::relocated(G_K));
                let t = rd32(lf_checker_rt::relocated(G_TABLE));
                let cell = t
                    .wrapping_add(c.wrapping_mul(ROW))
                    .wrapping_add(HDR);
                k.wrapping_mul(b).wrapping_add(rd32(cell))
            }
        }

        if idx >= ENTRY_COUNT {
            return 0;
        }
        let entry = rd32(
            this
                .wrapping_add(ENTRY_BASE)
                .wrapping_add(idx.wrapping_mul(4)),
        );
        if entry == 0 {
            return 0;
        }

        let _: u32 = lf_checker_rt::callee_thiscall!(SET_CONST, u32, object(entry), 0);
        let f = fadd(
            f32::from_bits(rd32(this.wrapping_add(BIAS_OFF))),
            f32::from_bits(farg),
        );
        let _: u32 =
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, object(entry), f.to_bits());
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_ENTRY, u32, entry, p4);
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_W2, u32, object(entry), w2);
        let _: u32 = lf_checker_rt::callee_thiscall!(SET_W3, u32, object(entry), w3);

        if rd8(entry.wrapping_add(TAG_B)) == NO_TAG {
            return 0;
        }
        let obj = object(entry);
        if obj == 0 {
            return 0;
        }
        // Snapshot the block before the call, like the original's staging.
        let mut blk = [0u32; 10];
        let mut i = 0u32;
        while i < COPY_WORDS {
            blk[i as usize] = rd32(buf.wrapping_add(i.wrapping_mul(4)));
            i += 1;
        }
        let dst: u32 = lf_checker_rt::callee_thiscall!(GET_DEST, u32, obj);
        let mut j = 0u32;
        while j < COPY_WORDS {
            wr32(dst.wrapping_add(j.wrapping_mul(4)), blk[j as usize]);
            j += 1;
        }
        0
    }
});
