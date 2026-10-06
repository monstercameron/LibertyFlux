// original: 0x008E68D0 sort_pass_copy_and_recurse (proposed)

/// One partition pass over 8-byte keyed elements, then tail recursion.
///
/// The setup routine (callee 1, cdecl) runs first as `(key, first, aux)`.
/// Then every element from `first` below `limit` (unsigned) is compared
/// against the key element: both are 8-byte pairs whose second word is an
/// IEEE float, and when the element's float is strictly above the key's
/// (an unordered/NaN comparison copies nothing) the key pair overwrites
/// the element (one way; the key is never written) and the copy callback
/// (callee 2, cdecl) runs as `(key, 0, (first - key) / 8, old_lo, old_hi,
/// aux)`, with the element count signed. Finally, while the leftover
/// `(cur - key)` length rounded down to 8 bytes is above 8 (signed), the
/// tail routine (callee 3, cdecl) runs as `(key, cur, aux)` and `cur`
/// retreats 8 bytes from `first`. Returns the final leftover length.
///
/// Original: 0x008E68D0 (cdecl, five stack arguments; the 4th is never
/// read).
lf_checker_rt::export!(
    cdecl,
    rw_008E68D0(key: u32, first: u32, limit: u32, _a3: u32, aux: u32) -> u32 {
        unsafe {
            /// Element size in bytes.
            const ELEM_SIZE: u32 = 8;
            /// Length mask: leftover lengths round down to 8 bytes.
            const LEN_MASK: u32 = 0xFFFF_FFF8;
            /// Tail loop stops at this leftover length (signed compare).
            const TAIL_MIN: u32 = 8;
            /// Setup routine callee id.
            const SETUP: u32 = 1;
            /// Copy callback callee id.
            const COPY: u32 = 2;
            /// Tail routine callee id.
            const TAIL: u32 = 3;

            #[inline(always)]
            unsafe fn rd32(a: u32) -> u32 {
                unsafe { (a as *const u32).read_unaligned() }
            }
            #[inline(always)]
            unsafe fn wr32(a: u32, v: u32) {
                unsafe { (a as *mut u32).write_unaligned(v) }
            }

            let _: u32 = lf_checker_rt::callee_cdecl!(SETUP, u32, key, first, aux);
            let n = (((first.wrapping_sub(key)) as i32) >> 3) as u32;
            let mut p = first;
            if first < limit {
                loop {
                    let k0 = rd32(key);
                    let k1 = rd32(key.wrapping_add(4));
                    let v0 = rd32(p);
                    let v1 = rd32(p.wrapping_add(4));
                    if f32::from_bits(v1) > f32::from_bits(k1) {
                        wr32(p, k0);
                        wr32(p.wrapping_add(4), k1);
                        let _: u32 =
                            lf_checker_rt::callee_cdecl!(COPY, u32, key, 0, n, v0, v1, aux);
                    }
                    p = p.wrapping_add(ELEM_SIZE);
                    if !(p < limit) {
                        break;
                    }
                }
            }
            let mut cur = first;
            loop {
                let left = (cur.wrapping_sub(key)) & LEN_MASK;
                if (left as i32) <= (TAIL_MIN as i32) {
                    return left;
                }
                let _: u32 = lf_checker_rt::callee_cdecl!(TAIL, u32, key, cur, aux);
                cur = cur.wrapping_sub(ELEM_SIZE);
            }
        }
    }
);
