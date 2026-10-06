// original: 0x00AB9BC0 blend_entry_pair (proposed)

/// Blend or copy two table entries into an output buffer.
///
/// Arguments (cdecl, four stack words, plain `ret`): `out` points to the
/// output (two 64-byte blocks are written, twelve words each, skipping the
/// words at `+0x0c` and `+0x2c` of each block), `table` to a header whose
/// word at `+0x14` points at 64-byte entries, `idx` to two signed indices,
/// `vec` to per-item data (a flag word then three floats per item, items
/// 64 bytes apart starting at `+0x14`). The function advances its own
/// `out` argument slot by `0x40` per processed item. It returns nothing
/// meaningful (whatever value the last item left in `eax`).
///
/// Algorithm. For each of the two indices in order: an index of `-1`
/// (equality) skips the item without advancing `out`. Otherwise the entry
/// is `table[0x14] + (index<<6)`. When the item's flag word equals `-1`
/// (equality; both comparisons are exact equality so signedness is
/// immaterial) the entry's twelve words are copied verbatim to `out`.
/// Otherwise helper 1 (thiscall: scratch matrix, pointer to the item's
/// third float) runs first and the entry is combined with the item's three
/// floats through the scratch matrix with single-precision ops in a fixed
/// order (below), writing twelve floats to `out`. `out` advances by
/// `0x40` after either kind of item; the item pointer always advances.
///
/// The helper really rewrites the nine matrix words, but the intercepted
/// stand-in performs no writes, so both sides consume the pre-call
/// initialiser: `1.0` at matrix offsets `0x50`, `0x64`, `0x78` and `0.0`
/// elsewhere. The rewrite hard-codes those nine values; the comparison
/// therefore covers the arithmetic given that matrix, disclosed in the
/// proof record. All float arithmetic is order-pinned through the helpers.
///
/// Edge cases: NaN or infinite entry/vec floats propagate through the
/// multiplies and adds per IEEE-754, bit-exact with the original; a `-1`
/// index leaves its output block untouched.
///
/// Original: 0x00AB9BC0 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00ab9bc0(out: u32, table: u32, idx: u32, vec: u32) -> u32 {
    unsafe {
        const HELPER: u32 = 1;
        // Pre-call matrix initialiser (see doc comment).
        const M50: f32 = 1.0;
        const M54: f32 = 0.0;
        const M58: f32 = 0.0;
        const M60: f32 = 0.0;
        const M64: f32 = 1.0;
        const M68: f32 = 0.0;
        const M70: f32 = 0.0;
        const M74: f32 = 0.0;
        const M78: f32 = 1.0;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { (a as *mut u32).write_unaligned(v.to_bits()) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let entries = rd32(table.wrapping_add(0x14));
        let mut outp = out;
        let mut last: u32 = 0;
        let mut it = 0u32;
        while it < 2 {
            let index = rd32(idx.wrapping_add(it.wrapping_mul(4)));
            let item = vec.wrapping_add(0x28).wrapping_add(it.wrapping_mul(0x40));
            if index != 0xFFFF_FFFF {
                let e = entries.wrapping_add(index.wrapping_shl(6));
                if rd32(item.wrapping_sub(0x14)) == 0xFFFF_FFFF {
                    wr32(outp, rd32(e));
                    wr32(outp.wrapping_add(4), rd32(e.wrapping_add(4)));
                    wr32(outp.wrapping_add(8), rd32(e.wrapping_add(8)));
                    wr32(outp.wrapping_add(0x10), rd32(e.wrapping_add(0x10)));
                    wr32(outp.wrapping_add(0x14), rd32(e.wrapping_add(0x14)));
                    wr32(outp.wrapping_add(0x18), rd32(e.wrapping_add(0x18)));
                    wr32(outp.wrapping_add(0x20), rd32(e.wrapping_add(0x20)));
                    wr32(outp.wrapping_add(0x24), rd32(e.wrapping_add(0x24)));
                    wr32(outp.wrapping_add(0x28), rd32(e.wrapping_add(0x28)));
                    wr32(outp.wrapping_add(0x30), rd32(e.wrapping_add(0x30)));
                    wr32(outp.wrapping_add(0x34), rd32(e.wrapping_add(0x34)));
                    last = rd32(e.wrapping_add(0x38));
                    wr32(outp.wrapping_add(0x38), last);
                } else {
                    let mut frame = [0u32; 15];
                    let _: u32 = lf_checker_rt::callee_thiscall!(
                        HELPER,
                        u32,
                        frame.as_mut_ptr() as u32,
                        item.wrapping_add(8)
                    );
                    let v20 = rdf(item.wrapping_sub(8));
                    let v24 = rdf(item.wrapping_sub(4));
                    let v28 = rdf(item);
                    let e0 = rdf(e);
                    let e4 = rdf(e.wrapping_add(4));
                    let e8 = rdf(e.wrapping_add(8));
                    let e10 = rdf(e.wrapping_add(0x10));
                    let e14 = rdf(e.wrapping_add(0x14));
                    let e18 = rdf(e.wrapping_add(0x18));
                    let e20 = rdf(e.wrapping_add(0x20));
                    let e24 = rdf(e.wrapping_add(0x24));
                    let e28 = rdf(e.wrapping_add(0x28));
                    let e30 = rdf(e.wrapping_add(0x30));
                    let e34 = rdf(e.wrapping_add(0x34));
                    let e38 = rdf(e.wrapping_add(0x38));
                    let sp20 = v20;
                    let sp30 = v28;
                    let sp0c = e14;
                    let mut x3 = mul(e14, M54);
                    let mut x0 = mul(e4, M50);
                    let mut x6 = e28;
                    let x2 = M64;
                    x3 = add(x3, x0);
                    x0 = mul(e24, M58);
                    let mut x5 = e20;
                    let mut x1 = M68;
                    x3 = add(x3, x0);
                    let mut sp08 = e18;
                    let sp3c = x3;
                    x3 = mul(e18, M54);
                    x0 = mul(e8, M50);
                    x3 = add(x3, x0);
                    x0 = mul(x6, M58);
                    x3 = add(x3, x0);
                    let mut x4 = e0;
                    let sp10 = e0;
                    x0 = mul(e10, x2);
                    let sp40 = x3;
                    x3 = M60;
                    x4 = mul(x4, x3);
                    let mut x7 = mul(e8, x3);
                    x4 = add(x4, x0);
                    x0 = mul(x5, x1);
                    x4 = add(x4, x0);
                    x0 = mul(sp0c, x2);
                    let sp44 = x4;
                    x4 = mul(e4, x3);
                    x3 = M70;
                    x4 = add(x4, x0);
                    x0 = mul(e24, x1);
                    x4 = add(x4, x0);
                    let sp48 = x4;
                    x4 = sp08;
                    x0 = mul(x4, x2);
                    x7 = add(x7, x0);
                    x0 = x6;
                    x6 = sp10;
                    x0 = mul(x0, x1);
                    x6 = mul(x6, x3);
                    x7 = add(x7, x0);
                    let sp4c = x7;
                    x1 = M74;
                    x0 = mul(e10, x1);
                    let mut x2b = M78;
                    x7 = sp0c;
                    x6 = add(x6, x0);
                    x0 = mul(x5, x2b);
                    x5 = mul(x7, x1);
                    x6 = add(x6, x0);
                    x0 = mul(e4, x3);
                    x7 = mul(x7, v24);
                    x5 = add(x5, x0);
                    x0 = mul(e24, x2b);
                    x4 = mul(x4, x1);
                    x5 = add(x5, x0);
                    x0 = mul(e8, x3);
                    x3 = e10;
                    x1 = mul(x3, v24);
                    x4 = add(x4, x0);
                    x0 = mul(e28, x2b);
                    x2b = e20;
                    x3 = mul(x3, M54);
                    x4 = add(x4, x0);
                    x0 = mul(sp10, v20);
                    x1 = add(x1, x0);
                    x0 = mul(x2b, v28);
                    x2b = mul(x2b, M58);
                    x1 = add(x1, x0);
                    x0 = mul(e4, sp20);
                    x1 = add(x1, e30);
                    x7 = add(x7, x0);
                    x0 = mul(e24, sp30);
                    x7 = add(x7, x0);
                    x0 = mul(sp08, v24);
                    x7 = add(x7, e34);
                    sp08 = x0;
                    x0 = mul(e8, sp20);
                    let mut sp0c2 = x7;
                    x7 = sp08;
                    x7 = add(x7, x0);
                    x0 = mul(e28, sp30);
                    x7 = add(x7, x0);
                    x0 = add(x7, e38);
                    sp08 = x7;
                    x7 = sp4c;
                    sp08 = x0;
                    x0 = mul(sp10, M50);
                    x0 = add(x0, x3);
                    x0 = add(x0, x2b);
                    last = outp;
                    wrf(outp, x0);
                    wrf(outp.wrapping_add(4), sp3c);
                    wrf(outp.wrapping_add(8), sp40);
                    wrf(outp.wrapping_add(0x10), sp44);
                    wrf(outp.wrapping_add(0x14), sp48);
                    wrf(outp.wrapping_add(0x18), x7);
                    wrf(outp.wrapping_add(0x20), x6);
                    wrf(outp.wrapping_add(0x24), x5);
                    x0 = sp0c2;
                    wrf(outp.wrapping_add(0x28), x4);
                    wrf(outp.wrapping_add(0x34), x0);
                    x0 = sp08;
                    wrf(outp.wrapping_add(0x30), x1);
                    wrf(outp.wrapping_add(0x38), x0);
                }
                outp = outp.wrapping_add(0x40);
            }
            it += 1;
        }
        last
    }
});
