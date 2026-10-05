// original: 0x00CBFBF0 task_pair_table_build (proposed)

/// Build entries in a global task-pair table from two input arrays.
///
/// `table_a` and `table_b` are lookup tables, `keys_a` and `keys_b` hold key
/// dwords followed by floats. For each of the four (a-key, b-key) pairs the
/// keys form an index `((a * 7 + b) * 24)` into the tables; a pair whose key
/// is -1, whose looked-up code falls outside 15..=81, or which arrives after
/// the table's nine slots are full contributes nothing. Otherwise one
/// 32-byte entry (code, -1 sentinel, three blended floats, the table-a row
/// pointer, a global float) is appended and the table count grows.
///
/// Original: 0x00CBFBF0 (stdcall, four stack arguments, no return value).
lf_checker_rt::export!(stdcall, rw_00cbfbf0(table_a: u32, table_b: u32, keys_a: u32, keys_b: u32) -> u32 {
    unsafe {
        const GAIN: u32 = 0x0117_35bc;
        const TAG_FLOAT: u32 = 0x0171_bf8c;
        const COUNT: u32 = 0x0171_bfb0;
        const ENTRIES: u32 = 0x0171_bfb4;
        const CODE_LO: u32 = 15;
        const CODE_HI: u32 = 81;
        const MAX_ENTRIES: u32 = 9;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let gain = f32::from_bits(lf_checker_rt::global::<u32>(GAIN).read());
        let tag = lf_checker_rt::global::<u32>(TAG_FLOAT).read();
        let mut count = lf_checker_rt::global::<u32>(COUNT).read();
        let mut pb = keys_b.wrapping_add(0x10);
        let mut outer = 2u32;
        while outer != 0 {
            outer -= 1;
            let mut pa = keys_a.wrapping_add(0x10);
            let mut inner = 2u32;
            while inner != 0 {
                inner -= 1;
                let ka = rd32(pa.wrapping_sub(0x10));
                let kb = rd32(pb.wrapping_sub(0x10));
                if ka != 0xffff_ffff && kb != 0xffff_ffff {
                    let idx = ka.wrapping_mul(7).wrapping_add(kb).wrapping_mul(24);
                    let row = table_a.wrapping_add(8).wrapping_add(idx);
                    let mut code = rd32(row);
                    if table_b != 0 && (code < CODE_LO || code > CODE_HI) {
                        code = rd32(table_b.wrapping_add(idx).wrapping_add(8));
                    }
                    if code.wrapping_sub(CODE_LO) <= CODE_HI - CODE_LO {
                        // The original performs these loads before testing the
                        // count; keep them first so a faulting load faults on
                        // both sides. The fence pins them above the branch.
                        let fa0 = rdf(pa.wrapping_sub(8));
                        let m0 = rdf(table_a.wrapping_add(idx).wrapping_add(0x18));
                        let f5 = rdf(table_a.wrapping_add(idx).wrapping_add(0x1c));
                        let fa1 = rdf(pa);
                        let fb0 = rdf(pb.wrapping_sub(8));
                        let fb1 = rdf(pb);
                        core::hint::black_box((fa0, m0, f5, fa1, fb0, fb1));
                        if (count as i32) < MAX_ENTRIES as i32 {
                            let f0 = mul(m0, gain);
                            let f1 = mul(fb0, fa0);
                            let f2 = mul(fb1, fa1);
                            let slot = lf_checker_rt::relocated(ENTRIES)
                                .wrapping_add(count.wrapping_shl(5));
                            wr32(slot, code);
                            wr32(slot.wrapping_add(4), 0xffff_ffff);
                            wr32(slot.wrapping_add(8), f1.to_bits());
                            wr32(slot.wrapping_add(0x0c), f2.to_bits());
                            wr32(slot.wrapping_add(0x10), f0.to_bits());
                            wr32(slot.wrapping_add(0x14), f5.to_bits());
                            wr32(slot.wrapping_add(0x18), row);
                            wr32(slot.wrapping_add(0x1c), tag);
                            count = count.wrapping_add(1);
                            lf_checker_rt::global::<u32>(COUNT).write(count);
                        }
                    }
                }
                pa = pa.wrapping_add(4);
            }
            pb = pb.wrapping_add(4);
        }
    }
    0
});
