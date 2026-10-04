// original: 0x00ae43a0 bitmask_table_dispatch (proposed)

/// Dispatch one table entry per set bit of `mask`, from the highest bit down.
///
/// `TABLE` holds one `ENTRY`-byte (0x54) entry per bit position; the entry for
/// bit `i` starts at `TABLE + i * ENTRY` and carries a float threshold at
/// offset `THRESH_OFF` (0x20). `obj` points to an object whose virtual slot at
/// `VT_SLOT` (0x58) answers a float (returned on the x87 stack); `f` is a
/// float passed by bits; `flag` selects the dispatch mode.
///
/// For each set bit, highest first: when `flag` is 0 or 1 and the entry's
/// threshold is not below +0.0 (a NaN threshold skips, like a negative one),
/// the object slot is called, its answer `v` forms `t = v * MULT + f` with
/// the constant at `MULT`, and `t` is compared against the threshold. If `t`
/// is strictly greater, the node builder (intercepted callee 2, thiscall with
/// the entry address and the constant 0x10) runs and the returned node gets
/// `obj` and the bits of `f` in its first two words. In every other case
/// (any other `flag`, a negative or NaN threshold, `t` not above it) the node
/// builder runs instead with `TABLE_DIRECT_BASE + flag * 8 + i * ENTRY` and
/// the node is filled the same way. The bit is then cleared and the next
/// highest set bit is handled.
///
/// A zero mask returns at once; the original then leaks its entry `eax`, so
/// the return value is not compared and the rewrite returns 0.
///
/// Original: 0x00ae43a0 (cdecl, four stack words). Bit scans use the
/// highest-set-bit idiom (`31 - leading_zeros`); float order is the
/// original's, pinned through `black_box`.
lf_checker_rt::export!(cdecl, rw_00ae43a0(mask: u32, obj: u32, fbits: u32, flag: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x01614cc0;
        const TABLE_DIRECT_BASE: u32 = 0x01614ca0;
        const ENTRY: u32 = 0x54;
        const THRESH_OFF: u32 = 0x20;
        const VT_SLOT: u32 = 0x58;
        const MULT: u32 = 0x00fe8a24;
        const NODE_ARG: u32 = 0x10;
        const NODE_CALLEE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd_glob_f(va: u32) -> f32 {
            unsafe { f32::from_bits((lf_checker_rt::global::<u32>(va) as *const u32).read_unaligned()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        if mask == 0 {
            return 0;
        }
        let f = f32::from_bits(fbits);
        let mult = rd_glob_f(MULT);
        let mut m = mask;
        while m != 0 {
            let i = 31 - m.leading_zeros();
            let entry = TABLE.wrapping_add(i.wrapping_mul(ENTRY));
            let mut done = false;
            if flag == 1 || flag == 0 {
                let th = rd_glob_f(entry.wrapping_add(THRESH_OFF));
                if th >= 0.0 {
                    // Object slot: thiscall through the fabricated vtable,
                    // loaded fresh each iteration like the original.
                    let slot: extern "thiscall" fn(u32) -> f32 =
                        unsafe { core::mem::transmute(rd32(rd32(obj) + VT_SLOT) as usize) };
                    let v = slot(obj);
                    let t = add(mul(v, mult), f);
                    if t > th {
                        let p: u32 = lf_checker_rt::callee_thiscall!(
                            NODE_CALLEE, u32, lf_checker_rt::relocated(entry), NODE_ARG);
                        wr32(p, obj);
                        wr32(p.wrapping_add(4), fbits);
                        done = true;
                    }
                }
            }
            if !done {
                let alt = TABLE_DIRECT_BASE
                    .wrapping_add(flag.wrapping_mul(8))
                    .wrapping_add(i.wrapping_mul(ENTRY));
                let p: u32 = lf_checker_rt::callee_thiscall!(
                    NODE_CALLEE, u32, lf_checker_rt::relocated(alt), NODE_ARG);
                wr32(p, obj);
                wr32(p.wrapping_add(4), fbits);
            }
            m &= !(1u32 << i);
        }
        0
    }
});
