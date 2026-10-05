// original: 0x00d10f80 task_scan_accept (proposed)

/// Scan the sixteen candidate slots of a task list and accept the first that
/// passes every gate, returning the accepted handle or 0.
///
/// `owner` is the accepting object, `list` the task list. The list holds an
/// array base at `+0x224` (slots at `+0x10c` to `+0x148`) and a position
/// block at `+0x20`. Each candidate carries a vtable pointer at `+0x0`, a
/// position block at `+0x20`, a flag byte at `+0xf1f` (bit `0x40` required),
/// a state word at `+0x12d0` and a helper area at `+0xdd4`.
///
/// Algorithm: for each slot in order, skip null candidates and ones without
/// the flag bit. Helper 0 runs on the helper area; when it reports success
/// the candidate's virtual slot `0xec` must yield a vector whose squared
/// length does not exceed 5. The candidate position must then lie within
/// squared distance 400 of the list position with a height gap of at most 5
/// (the gap is negated when below zero, NaN included as not-below), helper 2
/// must decline, and the state must be 7 (rewritten to 1), 1 or 5. Helper 3
/// is tried with word 1 and then word 3; the first zero answer selects that
/// word, two non-zero answers skip the slot. Helper 4 (run on the owner with
/// the list, 1 and 0) accepts the slot. The winner records 1 at list
/// `+0xa70` and 4 at owner `+0x18`, fetches the shared context through the
/// `0x167e2a0` global, and builds the handle with helper 6 (candidate,
/// selected word, `0x1b`, `0x1200000`, 0); the handle gets words 2, `0x16`
/// and `0x126` at `+0x2c`, `+0x50` and `+0x54` and is returned. A null
/// context result faults on the first handle store; no slot passing yields 0.
///
/// Float order is the original's; unordered comparisons fall through to the
/// next gate exactly as the original's conditional jumps do.
///
/// Original: 0x00d10f80 (thiscall, one stack word, full-word result).
lf_checker_rt::export!(thiscall, rw_00d10f80(owner: u32, list: u32) -> u32 {
    unsafe {
        const ARR_BASE: u32 = 0x224;
        const POS_PTR: u32 = 0x20;
        const PX: u32 = 0x30;
        const PY: u32 = 0x34;
        const PZ: u32 = 0x38;
        const FLAG_OFF: u32 = 0xf1f;
        const FLAG_BIT: u8 = 0x40;
        const STATE_OFF: u32 = 0x12d0;
        const HELPER_OFF: u32 = 0xdd4;
        const VT_SLOT: u32 = 0xec;
        const LIST_MARK: u32 = 0xa70;
        const OWNER_MARK: u32 = 0x18;
        const FIRST_SLOT: u32 = 0x10c;
        const PAST_SLOT: u32 = 0x14c;
        const GLOBAL_CTX: u32 = 0x167e2a0;
        const BUILD_TAG: u32 = 0x1b;
        const BUILD_BASE: u32 = 0x1200000;
        const VEC_LIMIT: f32 = 5.0;
        const DIST_LIMIT: f32 = 400.0;
        const SIGN: u32 = 0x8000_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
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
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }

        let entry_ecx = owner;
        let arr_base = rd32(list + ARR_BASE);
        let mut edi = FIRST_SLOT;
        'outer: loop {
            if edi >= PAST_SLOT {
                break;
            }
            let esi = rd32(arr_base.wrapping_add(edi));
            edi += 4;
            if esi == 0 {
                continue 'outer;
            }
            if rd8(esi + FLAG_OFF) & FLAG_BIT == 0 {
                continue 'outer;
            }
            let al_a: u32 = lf_checker_rt::callee_thiscall!(0, u32, esi.wrapping_add(HELPER_OFF));
            if (al_a & 0xff) != 0 {
                let vt = rd32(esi);
                let slot = rd32(vt + VT_SLOT);
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                let mut scratch = [0u32; 4];
                let vec = f(esi, scratch.as_mut_ptr() as u32);
                let vx = rdf(vec);
                let vy = rdf(vec + 4);
                let vz = rdf(vec + 8);
                let len = add(add(mul(vx, vx), mul(vy, vy)), mul(vz, vz));
                if len > VEC_LIMIT {
                    continue 'outer;
                }
            }
            let epos = rd32(esi + POS_PTR);
            let apos = rd32(list + POS_PTR);
            let dx = sub(rdf(epos + PX), rdf(apos + PX));
            let dy = sub(rdf(epos + PY), rdf(apos + PY));
            let dz = sub(rdf(epos + PZ), rdf(apos + PZ));
            let d2 = add(add(mul(dx, dx), mul(dy, dy)), mul(dz, dz));
            if d2 > DIST_LIMIT {
                continue 'outer;
            }
            let dz2 = sub(rdf(epos + PZ), rdf(apos + PZ));
            let gap = if !(dz2 < 0.0) {
                dz2
            } else {
                f32::from_bits(dz2.to_bits() ^ SIGN)
            };
            if gap > VEC_LIMIT {
                continue 'outer;
            }
            let al_c: u32 = lf_checker_rt::callee_thiscall!(2, u32, esi);
            if (al_c & 0xff) != 0 {
                continue 'outer;
            }
            let st = rd32(esi + STATE_OFF);
            if st == 7 {
                wr32(esi + STATE_OFF, 1);
            } else if st != 1 && st != 5 {
                continue 'outer;
            }
            let e1: u32 = lf_checker_rt::callee_thiscall!(3, u32, esi, 1);
            let sel;
            if e1 == 0 {
                sel = 1;
            } else {
                let e3: u32 = lf_checker_rt::callee_thiscall!(3, u32, esi, 3);
                if e3 != 0 {
                    continue 'outer;
                }
                sel = 3;
            }
            let al_e: u32 = lf_checker_rt::callee_thiscall!(4, u32, entry_ecx, list, 1, 0);
            if (al_e & 0xff) == 0 {
                continue 'outer;
            }
            wr32(list + LIST_MARK, 1);
            wr32(entry_ecx + OWNER_MARK, 4);
            let g = rd32(lf_checker_rt::relocated(GLOBAL_CTX));
            let ctx: u32 = lf_checker_rt::callee_thiscall!(5, u32, g);
            if ctx == 0 {
                wr32(0x2c, 2);
                wr32(0x50, 0x16);
                wr32(0x54, 0x126);
                return 0;
            }
            let h: u32 =
                lf_checker_rt::callee_thiscall!(6, u32, ctx, esi, sel, BUILD_TAG, BUILD_BASE, 0);
            wr32(h.wrapping_add(0x2c), 2);
            wr32(h.wrapping_add(0x50), 0x16);
            wr32(h.wrapping_add(0x54), 0x126);
            return h;
        }
        0
    }
});
