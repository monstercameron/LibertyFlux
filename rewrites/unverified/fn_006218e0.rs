// original: 0x006218e0 net_route_session_request (proposed)

/// Route a session request: check session state, poll the item list, then
/// either answer through the result sink or continue into session setup.
///
/// `this` is the session object, `a` the request, `b` an opaque token passed
/// on to the sink, `c` a second token used only by the setup call. The object
/// carries a state word at `+STATE`, two identity pairs (`+PAIR1_A/B`,
/// `+PAIR2_A/B`) and two token words (`+TOK0/TOK1`) that must match the
/// request's first two words. Each guard returns the mismatching word at once
/// (the state guard accepts only 2 and 3, compared signed).
///
/// Past the guards the request's item count (`+COUNT`) is polled through
/// callee 1, which is asked once per item and decrements the remainder on a
/// non-zero answer. A remainder above one asks callee 2 and compares both it
/// and the free-slot count against the remainder; a remainder of one asks
/// callee 4, then callee 5 for the request key, then the free-slot count, and
/// finally callee 6 to pick the answer code (2, or 4 when the answer is
/// non-zero while the tag word is zero). Shortfalls tail-call the sink
/// (callee 3) with (token, code, 0, 0) and return its answer.
///
/// Otherwise setup runs: callee 7 must agree (low byte), then callee 8 checks
/// one key and callee 11 the other; any refusal tail-calls the sink. Callee
/// 10 then aggregates the request into a scratch struct. When the object has
/// no callback (`+CB_FN` null) callees 12 and 13 fill and consume a second
/// scratch struct and a three-word native call runs; otherwise the callback
/// is invoked with (this, payload, scratch), as a thiscall through `+CB_FLAG`
/// when the flag is non-zero. A zero low byte answers through the sink with
/// code 5, a non-zero one probes the item list twice (callees 15 and 16, the
/// second with a zeroed tag) and either answers with code 2 or continues
/// into the deep setup call (callee 17) with (request, token, token2,
/// scratch, slot word), returning its answer.
///
/// Edge cases and models (see the contract): the free-slot count is
/// `+F_C + +F_D - +F_B - (zero dwords among +Z_N entries at +Z_TAB)` in
/// wrapping arithmetic, compared unsigned; untouched frame words (the slot
/// word, the native call's value slots) read as 0 under the contract's zero
/// stack fill; the callback is modeled as preserving its entry ecx (the
/// contract's preserve flag), which the probe calls then reuse; the
/// aggregator's scratch struct and the setup call's scratch pointer are
/// skipped from the call comparison with the aggregator's eight scripted
/// words snapshotted.
///
/// Original: 0x006218e0 (thiscall, three stack words), returns the sink or
/// setup answer, or the mismatching guard word.
lf_checker_rt::export!(thiscall, rw_006218e0(this: u32, a: u32, b: u32, c: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x50;
        const PAIR1_A: u32 = 0xbf0;
        const PAIR1_B: u32 = 0xc30;
        const PAIR2_A: u32 = 0xbf4;
        const PAIR2_B: u32 = 0xc34;
        const TOK0: u32 = 0x540;
        const TOK1: u32 = 0x544;
        const COUNT: u32 = 0x290;
        const ITEMS: u32 = 0x298;
        const ITEM_STEP: u32 = 0x10;
        const TAG: u32 = 0x88;
        const AUX: u32 = 0x8c;
        const BLK: u32 = 0x90;
        const KEY48: u32 = 0x48;
        const KEY50: u32 = 0x50;
        const PAYLOAD: u32 = 8;
        const F_A: u32 = 0x124;
        const F_B: u32 = 0x1e14;
        const F_C: u32 = 0x120;
        const F_D: u32 = 0x2ea8;
        const Z_N: u32 = 0x32b0;
        const Z_TAB: u32 = 0x2ec0;
        const Z_STEP: u32 = 0x20;
        const CB_FN: u32 = 0x10;
        const CB_FLAG: u32 = 0xc;
        const FC_CTX: u32 = 0x32c4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        unsafe fn free_slots(this: u32) -> u32 {
            unsafe {
                let mut f = rd32(this + F_A).wrapping_sub(rd32(this + F_B));
                f = f.wrapping_sub(rd32(this + F_A));
                f = f.wrapping_add(rd32(this + F_C));
                f = f.wrapping_add(rd32(this + F_D));
                let n = rd32(this + Z_N) as i32;
                if n > 0 {
                    let mut p = this + Z_TAB;
                    let mut z = 0u32;
                    let mut k = n;
                    while k != 0 {
                        if rd32(p) == 0 {
                            z += 1;
                        }
                        p += Z_STEP;
                        k -= 1;
                    }
                    f = f.wrapping_sub(z);
                }
                f
            }
        }

        unsafe fn single_path(this: u32, a: u32, b: u32, c: u32) -> u32 {
            unsafe {
                let tag = rd32(a + TAG);
                let q: u32 = lf_checker_rt::callee_thiscall!(4, u32, this, tag);
                if q >= 1 {
                    return main_section(this, a, b, c);
                }
                let r: u32 = lf_checker_rt::callee_thiscall!(5, u32, this, a + KEY50);
                if r != 0 {
                    return main_section(this, a, b, c);
                }
                if free_slots(this) >= 1 {
                    return main_section(this, a, b, c);
                }
                let s: u32 = lf_checker_rt::callee_thiscall!(6, u32, this, 1);
                let code = if s != 0 && tag == 0 { 4 } else { 2 };
                lf_checker_rt::callee_thiscall!(3, u32, this, b, code, 0, 0)
            }
        }

        unsafe fn bf_path(this: u32, a: u32, b: u32, c: u32, scratch: u32) -> u32 {
            unsafe {
                let r1: u32 = lf_checker_rt::callee_thiscall!(15, u32, this, a + ITEMS,
                    rd32(a + COUNT), rd32(a + TAG), scratch, 1, scratch);
                if (r1 as u8) == 0 {
                    let r2: u32 = lf_checker_rt::callee_thiscall!(16, u32, this, a + ITEMS,
                        rd32(a + COUNT), 0, scratch, 1, scratch);
                    if (r2 as u8) == 0 {
                        return lf_checker_rt::callee_thiscall!(3, u32, this, b, 2, 0, 0);
                    }
                }
                let mut f3buf = [0u32; 1];
                lf_checker_rt::callee_thiscall!(17, u32, this, a, b, c,
                    f3buf.as_mut_ptr() as u32, 0)
            }
        }

        unsafe fn main_section(this: u32, a: u32, b: u32, c: u32) -> u32 {
            unsafe {
                let t: u32 = lf_checker_rt::callee_thiscall!(7, u32, this);
                if (t as u8) == 0 {
                    return lf_checker_rt::callee_thiscall!(3, u32, this, b, 0, 0, 0);
                }
                let u: u32 = lf_checker_rt::callee_thiscall!(8, u32, this, a + KEY48);
                if u != 0 {
                    return lf_checker_rt::callee_thiscall!(3, u32, this, b, 3, 0, 0);
                }
                let v: u32 = lf_checker_rt::callee_thiscall!(11, u32, this, a + KEY50);
                if v != 0 {
                    return lf_checker_rt::callee_thiscall!(3, u32, this, b, 3, 0, 0);
                }
                let mut ea70 = [0u32; 8];
                lf_checker_rt::callee_thiscall!(10, u32, ea70.as_mut_ptr() as u32, b,
                    rd32(a + TAG), a + BLK, rd32(a + AUX), rd32(a + COUNT));
                let cb = rd32(this + CB_FN);
                if cb == 0 {
                    let mut agg = [0u32; 8];
                    lf_checker_rt::callee_thiscall!(12, u32, agg.as_mut_ptr() as u32,
                        a + PAYLOAD, b, rd32(a + TAG), a + BLK, rd32(a + AUX));
                    lf_checker_rt::callee_thiscall!(13, u32, this + FC_CTX, this,
                        agg.as_mut_ptr() as u32);
                    // Untouched frame words read as 0 under the zero stack
                    // fill: the native call's value slot, its flag byte, and
                    // the slot word it leaves behind.
                    let mut d1 = [0u32; 1];
                    let mut d2 = [0u32; 1];
                    lf_checker_rt::callee_cdecl!(14, u32,
                        d1.as_mut_ptr() as u32, d2.as_mut_ptr() as u32, 0u32);
                    let slot = 0u32;
                    let flag_is_zero = true;
                    if !flag_is_zero {
                        return bf_path(this, a, b, c, slot);
                    }
                    return lf_checker_rt::callee_thiscall!(3, u32, this, b, 5, 0, slot);
                }
                let flag = rd32(this + CB_FLAG);
                let mut ibuf = [0u32; 1];
                let ans = if flag != 0 {
                    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                        unsafe { core::mem::transmute(cb as usize) };
                    f(flag, this, a + PAYLOAD, ibuf.as_mut_ptr() as u32)
                } else {
                    let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
                        unsafe { core::mem::transmute(cb as usize) };
                    f(this, a + PAYLOAD, ibuf.as_mut_ptr() as u32)
                };
                if (ans as u8) != 0 {
                    // The stub preserves entry ecx, so the probe calls reuse
                    // the flag exactly as the original reuses it.
                    return bf_path(this, a, b, c, flag);
                }
                let slot = 0u32;
                lf_checker_rt::callee_thiscall!(3, u32, this, b, 5, 0, slot)
            }
        }

        let st = rd32(this + STATE) as i32;
        if st < 2 || st > 3 {
            return st as u32;
        }
        let x = rd32(this + PAIR1_A);
        if x != rd32(this + PAIR1_B) {
            return x;
        }
        let x = rd32(this + PAIR2_A);
        if x != rd32(this + PAIR2_B) {
            return x;
        }
        let x = rd32(this + TOK0);
        if x != rd32(a) {
            return x;
        }
        let x = rd32(this + TOK1);
        if x != rd32(a + 4) {
            return x;
        }

        let count = rd32(a + COUNT);
        if count > 1 {
            let mut cursor = a + ITEMS;
            let mut left = count;
            let mut rest = count;
            loop {
                let ok: u32 = lf_checker_rt::callee_thiscall!(1, u32, this, cursor);
                if ok != 0 {
                    rest -= 1;
                }
                cursor += ITEM_STEP;
                left -= 1;
                if left == 0 {
                    break;
                }
            }
            if rest > 1 {
                let q: u32 = lf_checker_rt::callee_thiscall!(2, u32, this, rd32(a + TAG));
                if q < rest && free_slots(this) < rest {
                    return lf_checker_rt::callee_thiscall!(3, u32, this, b, 2, 0, 0);
                }
                return main_section(this, a, b, c);
            } else if rest == 1 {
                return single_path(this, a, b, c);
            } else {
                return main_section(this, a, b, c);
            }
        } else if count == 1 {
            return single_path(this, a, b, c);
        }
        main_section(this, a, b, c)
    }
});
