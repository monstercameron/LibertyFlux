// original: 0x00AE8510 sweep_timing_entries (proposed)

/// Sweep a list of timed entries, keeping or retiring each one.
///
/// Arguments (cdecl): `aux` is passed through as the probe callee's second
/// argument; `start`/`end` delimit an array of 8-byte entries (object
/// pointer, float tag); `ctx` is the timing context whose dword at `+0x900`
/// selects the bit cleared from retired entries and which is passed as the
/// probe callee's third argument.
///
/// Returns the last callee result when the last entry reported, else the
/// original's leftover EAX for the last entry processed (the kept-bit mask
/// after a quiet retire, the shifted class flags or the inverted `+0xC`
/// word after a fold-path entry), or the `+0x900` field when the list is
/// empty. When the global enable flag is set the original returns whatever
/// was in EAX on entry; the contract pins the flag to zero because entry
/// EAX cannot be observed by a rewrite (see `narrowed`).
///
/// Per entry with a null object pointer nothing happens. Otherwise the entry
/// kind byte at `+0x61` selects the path:
/// - Zero: probe the object through callee 1 `(object, end, aux-or-ctx)`.
///   A zero low byte means retired: clear the context bit from the mask
///   dwords at `+0x54`/`+0x58`, null the entry, and when the counter at
///   `+0x34` is nonzero report through callee 2 `(mask, object, tag, flag)`,
///   where `mask` is the newly-uncovered bits of `+0x8` (narrowed by the
///   global mask when byte `+0x26` bit 0 is set) and `flag` is 7 when the
///   class row's `+0x40` has bit 8 set, else 0.
/// - Nonzero: when the class row's `+0x40` has bit 3 clear, fold the
///   object's low 24 mask bits into its parent's `+0x58` (the object at
///   `+0x4C`, when nonzero) and null the entry. When bit 3 is set the
///   entry is always kept, and the fold happens only when the parent is
///   nonzero and the guards pass (`+0x24` bit 6 set, `+0x63` below 0xEF,
///   `+0x28` bit 19 clear).
///
/// The class row for an object is `CLASS_TABLE[word at +0x2E]`, indexed by
/// the sign-extended 16-bit kind. The entry cursor advances 8 bytes per
/// entry until `end`. Original: 0x00AE8510 (cdecl, four stack words).
lf_checker_rt::export!(cdecl, rw_00AE8510(aux: u32, start: u32, end: u32, ctx: u32) -> u32 {
    unsafe {
        const CLASS_TABLE: u32 = 0x01295CD8;
        const ENABLE_FLAG: u32 = 0x01593311;
        const GLOBAL_MASK: u32 = 0x0159B760;
        const CTX_BITSEL: u32 = 0x900;
        const KIND: u32 = 0x2E;
        const PATH_SEL: u32 = 0x61;
        const PARENT: u32 = 0x4C;
        const MASK_A: u32 = 0x54;
        const MASK_B: u32 = 0x58;
        const COUNTER: u32 = 0x34;
        const SEEN: u32 = 0x0C;
        const BITS: u32 = 0x08;
        const NARROW: u32 = 0x26;
        const GUARD: u32 = 0x24;
        const LEVEL: u32 = 0x63;
        const STATE: u32 = 0x28;
        const CLASS_FLAGS: u32 = 0x40;
        const LOW24: u32 = 0x00FF_FFFF;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }

        /// Fold `obj`'s low 24 mask bits into its parent's `+0x58` word,
        /// keeping the parent's high byte. Written as the original's
        /// instruction sequence over named values.
        #[inline(always)]
        unsafe fn fold_into_parent(obj: u32, parent: u32) {
            unsafe {
                let m58 = rd32(obj + MASK_B);
                let m54 = rd32(obj + MASK_A);
                let seen = rd32(obj + SEEN);
                let bits = rd32(obj + BITS);
                let old = rd32(parent + MASK_B);
                let mut fold = !m58 & m54;
                fold = !fold & !seen & bits;
                fold |= old;
                fold ^= old;
                fold &= LOW24;
                fold ^= old;
                wr32(parent + MASK_B, fold);
            }
        }

        if lf_checker_rt::global::<u8>(ENABLE_FLAG).read() != 0 {
            // The original returns entry EAX here, which no rewrite can
            // observe; the contract pins the flag to zero so this never runs.
            return 0;
        }
        let bitsel = rd32(ctx + CTX_BITSEL);
        let mut cur = start;
        if cur == end {
            return bitsel;
        }
        let table = lf_checker_rt::relocated(CLASS_TABLE);
        let mut ret = bitsel;
        while cur != end {
            let obj = rd32(cur);
            if obj != 0 {
                let idx = ((obj + KIND) as *const i16).read_unaligned() as i32;
                let class = (table as *const u32).offset(idx as isize).read_unaligned();
                if rd8(obj + PATH_SEL) == 0 {
                    let r: u32 = lf_checker_rt::callee_cdecl!(1, u32, obj, aux, ctx);
                    if (r & 0xFF) != 0 {
                        ret = r;
                    } else {
                        let keep = !(1u32.wrapping_shl(bitsel & 31)) | 0xFF00_0000;
                        wr32(obj + MASK_A, rd32(obj + MASK_A) & keep);
                        wr32(obj + MASK_B, rd32(obj + MASK_B) & keep);
                        wr32(cur, 0);
                        if rd32(obj + COUNTER) != 0 {
                            let mut mask = !rd32(obj + SEEN) & rd32(obj + BITS);
                            if rd8(obj + NARROW) & 1 != 0 {
                                mask &= !lf_checker_rt::global::<u32>(GLOBAL_MASK).read_unaligned();
                            }
                            let flag = if rd32(class + CLASS_FLAGS) & 0x100 != 0 { 7 } else { 0 };
                            let tag = rd32(cur + 4);
                            let r2: u32 = lf_checker_rt::callee_cdecl!(2, u32, mask, obj, tag, flag);
                            ret = r2;
                        } else {
                            // Leftover: the last EAX assignment was the OR.
                            ret = keep;
                        }
                    }
                } else {
                    let flags = rd32(class + CLASS_FLAGS);
                    if flags & 0x08 == 0 {
                        let parent = rd32(obj + PARENT);
                        if parent != 0 {
                            fold_into_parent(obj, parent);
                            // Leftover: the merge's last EAX value.
                            ret = !rd32(obj + SEEN);
                        } else {
                            // Leftover: the flags tested for bit 3.
                            ret = flags >> 3;
                        }
                        wr32(cur, 0);
                    } else {
                        let parent = rd32(obj + PARENT);
                        if parent != 0
                            && rd8(obj + GUARD) & 0x40 != 0
                            && rd8(obj + LEVEL) < 0xEF
                        {
                            let shifted = rd32(obj + STATE) >> 0x13;
                            if shifted & 1 == 0 {
                                fold_into_parent(obj, parent);
                                ret = !rd32(obj + SEEN);
                            } else {
                                ret = shifted;
                            }
                        } else {
                            ret = flags >> 3;
                        }
                        wr32(cur, obj);
                    }
                }
            }
            cur = cur.wrapping_add(8);
        }
        ret
    }
});
