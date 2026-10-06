// original: 0x008EAC60 quantize_pair_and_store (proposed)

/// Quantise a float pair and file the combined index.
///
/// Writes -1 to `*out`, quantises the two floats at `pair` with the indexer
/// (callee 1, thiscall on `obj`), combines the answers as the low 16 bits
/// of `r1 + r2 * 8`, and files the entry with the storer (callee 2,
/// thiscall on `obj`) as `(out, index, pair, &limit)`, where `&limit`
/// points at the incoming `limit` float's slot. Returns `out`.
///
/// The `&limit` argument is the original's own stack slot, so its address
/// differs between the sides: the contract skips that argument and compares
/// the pointed-to word (the `limit` bits) with a call-time snapshot.
///
/// Original: 0x008EAC60 (thiscall, three stack arguments).
lf_checker_rt::export!(thiscall, rw_008EAC60(obj: u32, out: u32, pair: u32, limit: u32) -> u32 {
    unsafe {
        /// Scale applied to the second quantised answer.
        const Q_MUL: u32 = 8;
        /// Indexer callee id.
        const INDEX: u32 = 1;
        /// Storer callee id.
        const STORE: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(out, 0xFFFF_FFFF);
        let r1: u32 = lf_checker_rt::callee_thiscall!(INDEX, u32, obj, rd32(pair));
        let r2: u32 =
            lf_checker_rt::callee_thiscall!(INDEX, u32, obj, rd32(pair.wrapping_add(4)));
        let idx = r1.wrapping_add(r2.wrapping_mul(Q_MUL)) & 0xFFFF;
        let slot = limit;
        let _: u32 = lf_checker_rt::callee_thiscall!(
            STORE,
            u32,
            obj,
            out,
            idx,
            pair,
            &slot as *const u32 as u32
        );
        out
    }
});
