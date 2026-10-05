// original: 0x00E3E9F0 refresh_derived_state (proposed)

/// Refresh an object's derived float state from globals, a table and callees.
///
/// `this` points to an object with a source float (`+0x10`), a tag
/// (`+0x34`), a present byte (`+0x14`), the derived value (`+0xc`), a mode
/// byte (`+0x15`) and two limit floats (`+0x24`, `+0x20`). A global enable
/// byte gates everything: when clear the function just returns an
/// entry-register residue (see below).
///
/// Behaviour: when the source float is nonzero, a tag callee is asked
/// about a global dword and a neighbour routine re-runs if the tag
/// changed. A status callee's low byte is saved for the return. A float
/// provider (same scratch out-pointer each time, constant tags) feeds a
/// compare of its first answer against a second callee's integer answer
/// as a float: a strictly greater provider value zeroes the present byte
/// and the derived value and ends the function, as does a clear present
/// byte. Otherwise two more provider floats are kept. A table index from
/// a global picks a row (or none for -1) and a registry callee yields a
/// large object Selecting a working float: 127.0 by default, the row's
/// integer slot as a float when a first object flag is set and a row is
/// present (a zero there skips ahead instead), -127.0 when a first
/// byte-xor exceeds 0x7f, back to 127.0 unless a second byte-xor also
/// exceeds it (either failing check skips ahead). A clear mode byte flips
/// the working float's sign then a five-pointer call runs and its x87
/// float result is folded with the kept floats (a two-step minimum then
/// sum-minus-min) into a divisor. After that join, a second object flag
/// picks either the divisor as is or the divisor quartered when another
/// status answer's low byte is clear; the first kept float divided by it
/// is added to (mode clear) or subtracted from (mode set) the derived
/// value and stored back. Three more provider answers, another scratch
/// call and a final lookup call follow; then the first limit is compared
/// against the lookup's second out-word (a strict less, or an unordered
/// compare, moves on, as does a set mode byte), else a status call may
/// run a applier routine and flip the mode byte, the derived value is
/// zeroed and the function ends. On the moved-on path the third provider
/// float is compared against the second limit the same way (less or
/// unordered ends, as does a clear mode byte), else the status/applier
/// pair may run again; a zero source float then reloads the derived value
/// from a stale slot and the derived value is overwritten with the source
/// float. Every end re-asks the tag callee, stores the tag, and returns
/// the tag answer's high bytes with the saved status byte.
///
/// The early path's return carries the caller's entry EAX in its high
/// bytes, which no Rust rewrite can observe, so the proof contract pins
/// the enable byte set (main path only) and checks the return there; a
/// supplementary early-path run covers the early branch with the return
/// unchecked. Three scratch words the original never writes are modelled
/// as callee out-words (the provider's word feeding the fold, the
/// five-pointer call's word feeding it, the lookup's second word feeding
/// the limit compare); if the real callees do not write them those reads
/// are uninitialized stack, and the downstream logic treats every value
/// identically (moves and ordered-or-unordered compares only), so the
/// rewrite agrees either way. The float operation order is the original's.
///
/// Original: 0x00E3E9F0 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00E3E9F0(this: u32) -> u32 {
    unsafe {
        const OFF_SRC: u32 = 0x10;
        const OFF_TAG: u32 = 0x34;
        const OFF_PRESENT: u32 = 0x14;
        const OFF_VALUE: u32 = 0x0c;
        const OFF_MODE: u32 = 0x15;
        const OFF_LIM0: u32 = 0x24;
        const OFF_LIM1: u32 = 0x20;
        const ENABLE: u32 = 0x11609f6;
        const TAG_GLOBAL: u32 = 0x1160c0c;
        const INDEX_GLOBAL: u32 = 0x10330f8;
        const ROW_BASE: u32 = 0x118d470;
        const ROW_STRIDE: i32 = 0xbc;
        const ROW_SLOT: u32 = 0x10;
        const HI_TABLE: u32 = 0xfe8bc4;
        const LO_TABLE: u32 = 0xeab704;
        const SIGN_TABLE: u32 = 0xfe8fa0;
        const QTR_TABLE: u32 = 0xfe87e4;
        const OFF_FLAG0: u32 = 0x328c;
        const OFF_FLAG1: u32 = 0x328d;
        const OFF_XA0: u32 = 0x2c6e;
        const OFF_XA1: u32 = 0x2c6c;
        const OFF_XB0: u32 = 0x2c7e;
        const OFF_XB1: u32 = 0x2c7c;
        const CAL_TAG: u32 = 1;
        const CAL_RERUN: u32 = 2;
        const CAL_STATUS: u32 = 3;
        const CAL_FLOAT: u32 = 4;
        const CAL_MEASURE: u32 = 5;
        const CAL_REGISTRY: u32 = 6;
        const CAL_COMBINE: u32 = 7;
        const CAL_CHECK: u32 = 8;
        const CAL_APPLY: u32 = 9;
        const CAL_SCRATCH: u32 = 10;
        const CAL_LOOKUP: u32 = 11;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
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
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        if rd8(lf_checker_rt::relocated(ENABLE)) == 0 {
            // Early path: returns entry-EAX residue (see doc comment).
            return 0;
        }
        // Every end re-asks the tag callee (overwriting EAX) and returns
        // the tag answer's high bytes with the saved status byte.
        let finish = |saved: u8| unsafe {
            let tag: u32 = lf_checker_rt::callee_cdecl!(
                CAL_TAG,
                u32,
                rd32(lf_checker_rt::relocated(TAG_GLOBAL))
            );
            wr32(this.wrapping_add(OFF_TAG), tag);
            (tag & 0xffffff00) | saved as u32
        };

        if rdf(this.wrapping_add(OFF_SRC)) != 0.0 {
            let tag: u32 = lf_checker_rt::callee_cdecl!(
                CAL_TAG,
                u32,
                rd32(lf_checker_rt::relocated(TAG_GLOBAL))
            );
            if rd32(this.wrapping_add(OFF_TAG)) != tag {
                let _: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_RERUN, u32, this);
            }
        }
        let status: u32 = lf_checker_rt::callee_thiscall!(CAL_STATUS, u32, this);
        let saved = (status & 0xff) as u8;
        let mut scratch0: u32 = 0;
        let p1: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut scratch0 as *mut u32 as u32,
            0x31
        );
        let measure: u32 = lf_checker_rt::callee_thiscall!(CAL_MEASURE, u32, this);
        if rdf(p1) > (measure as i32) as f32 {
            wr8(this.wrapping_add(OFF_PRESENT), 0);
            wr32(this.wrapping_add(OFF_VALUE), 0);
            return finish(saved);
        }
        if rd8(this.wrapping_add(OFF_PRESENT)) == 0 {
            return finish(saved);
        }
        let p2: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut scratch0 as *mut u32 as u32,
            0x30
        );
        let float_a = rdf(p2.wrapping_add(4));
        let p3: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut scratch0 as *mut u32 as u32,
            0x2f
        );
        let float_b = rdf(p3.wrapping_add(4));
        let mut slot_fold: u32 = 0;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut slot_fold as *mut u32 as u32,
            0x2f
        );
        let fold_in = f32::from_bits(slot_fold);
        let index = rd32(lf_checker_rt::relocated(INDEX_GLOBAL)) as i32;
        let row: u32 = if index == -1 {
            0
        } else {
            lf_checker_rt::relocated(ROW_BASE)
                .wrapping_add(index.wrapping_mul(ROW_STRIDE) as u32)
        };
        let big: u32 = lf_checker_rt::callee_cdecl!(CAL_REGISTRY, u32, 0);
        // Working-float selection; None means skip ahead to the join.
        let mut selected: Option<f32> = None;
        if rd8(big.wrapping_add(OFF_FLAG0)) != 0 && row != 0 {
            let t = rd32(row.wrapping_add(ROW_SLOT)) as i32 as f32;
            if t != 0.0 {
                selected = Some(t);
            }
        } else if rd8(big.wrapping_add(OFF_FLAG1)) != 0 {
            let x1 = rd8(big.wrapping_add(OFF_XA0)) ^ rd8(big.wrapping_add(OFF_XA1));
            if x1 > 0x7f {
                selected = Some(f32::from_bits(rd32(
                    lf_checker_rt::relocated(LO_TABLE),
                )));
            } else {
                let x2 =
                    rd8(big.wrapping_add(OFF_XB0)) ^ rd8(big.wrapping_add(OFF_XB1));
                if x2 > 0x7f {
                    selected = Some(f32::from_bits(rd32(
                        lf_checker_rt::relocated(HI_TABLE),
                    )));
                }
            }
        } else {
            // Second flag clear: skip ahead to the join.
            selected = None;
        }
        // Divisor for the join: the fold result on the selected path, the
        // second kept float when skipping ahead.
        let divisor: f32;
        if let Some(mut work) = selected {
            if rd8(this.wrapping_add(OFF_MODE)) == 0 {
                let mask = rd32(lf_checker_rt::relocated(SIGN_TABLE));
                work = f32::from_bits(work.to_bits() ^ mask);
            }
            let mut s_e32 = 1.0f32.to_bits();
            let mut s_e40 = 127.0f32.to_bits();
            let mut s_e20: u32 = 0;
            let mut s_e44 = work.to_bits();
            let comb: f32 = lf_checker_rt::callee_thiscall!(
                CAL_COMBINE,
                f32,
                this,
                &mut s_e32 as *mut u32 as u32,
                &mut s_e40 as *mut u32 as u32,
                &mut slot_fold as *mut u32 as u32,
                &mut s_e20 as *mut u32 as u32,
                &mut s_e44 as *mut u32 as u32
            );
            let mut x1 = comb;
            let x2 = fold_in;
            let x0b = f32::from_bits(s_e20);
            // If the first compare takes the max, the second compare is
            // skipped entirely (an unconditional jump past it).
            if x2 > x1 {
                x1 = x2;
            } else if x1 > x0b {
                x1 = x0b;
            }
            divisor = sub(add(x0b, x2), x1);
        } else {
            divisor = float_b;
        }
        // Join: stale derived value kept for a later reload.
        let stale = rdf(this.wrapping_add(OFF_VALUE));
        let mut x0c: f32;
        if rd8(big.wrapping_add(OFF_FLAG1)) == 0 {
            x0c = divisor;
        } else {
            let check: u32 = lf_checker_rt::callee_thiscall!(CAL_CHECK, u32, this);
            x0c = divisor;
            if check & 0xff == 0 {
                x0c = mul(
                    x0c,
                    f32::from_bits(rd32(lf_checker_rt::relocated(QTR_TABLE))),
                );
            }
        }
        let ratio = div(float_a, x0c);
        if rd8(this.wrapping_add(OFF_MODE)) == 0 {
            wrf(this.wrapping_add(OFF_VALUE), add(ratio, stale));
        } else {
            wrf(this.wrapping_add(OFF_VALUE), sub(stale, ratio));
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut scratch0 as *mut u32 as u32,
            0x0
        );
        let p6: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut scratch0 as *mut u32 as u32,
            0x16
        );
        let float_c = rdf(p6.wrapping_add(4));
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_FLOAT,
            u32,
            &mut scratch0 as *mut u32 as u32,
            0x30
        );
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_SCRATCH,
            u32,
            &mut scratch0 as *mut u32 as u32
        );
        let mut slot_lim: [u32; 2] = [0, 0];
        let _: u32 = lf_checker_rt::callee_cdecl!(
            CAL_LOOKUP,
            u32,
            2,
            slot_lim.as_mut_ptr() as u32,
            0,
            0
        );
        let lim = f32::from_bits(slot_lim[1]);
        if rdf(this.wrapping_add(OFF_LIM0)) >= lim {
            if rd8(this.wrapping_add(OFF_MODE)) == 0 {
                let check: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_CHECK, u32, this);
                if check & 0xff != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_APPLY, u32, this);
                    wr8(
                        this.wrapping_add(OFF_MODE),
                        if rd8(this.wrapping_add(OFF_MODE)) == 0 {
                            1
                        } else {
                            0
                        },
                    );
                }
                wr32(this.wrapping_add(OFF_VALUE), 0);
                return finish(saved);
            }
        }
        if float_c >= rdf(this.wrapping_add(OFF_LIM1)) {
            if rd8(this.wrapping_add(OFF_MODE)) != 0 {
                let check: u32 =
                    lf_checker_rt::callee_thiscall!(CAL_CHECK, u32, this);
                if check & 0xff != 0 {
                    let _: u32 =
                        lf_checker_rt::callee_thiscall!(CAL_APPLY, u32, this);
                    wr8(
                        this.wrapping_add(OFF_MODE),
                        if rd8(this.wrapping_add(OFF_MODE)) == 0 {
                            1
                        } else {
                            0
                        },
                    );
                }
                if rdf(this.wrapping_add(OFF_SRC)) == 0.0 {
                    wrf(this.wrapping_add(OFF_SRC), stale);
                }
                let current = rd32(this.wrapping_add(OFF_SRC));
                wr32(this.wrapping_add(OFF_VALUE), current);
                return finish(saved);
            }
        }
        finish(saved)
    }
});
