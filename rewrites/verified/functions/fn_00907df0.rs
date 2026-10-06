// original: 0x00907DF0 input_ui_slot_sweep (proposed)

/// Sweep a 1500-entry global slot table, building one render element per
/// live slot and clearing each visited slot's mode word.
///
/// Arguments: none (cdecl/0, plain `ret`). All entry registers are dead.
/// Returns in EAX the last slot pointer stored to, or the pre-loop helper
/// answer when every iteration skips early.
///
/// Layout. TABLE holds one pointer per row; a live row points at a record
/// with a flag byte at `+8`, a mode word at `+0xC` (live means 4), four
/// floats at `+0x10`/`+0x14`/`+0x18`/`+0x1C`, bitfield bytes at `+0x20`
/// (bit 2 gates the row, bit 1 feeds the row call) and `+0x21` (bit 0),
/// and a parameter dword at `+0x54`. COUNT holds the current slot index N:
/// the row equal to N is skipped, and a row whose flag byte is clear reads
/// its bitfield bytes and parameter from row N instead of its own record.
/// Element objects start with a vtable pointer (`+0`), an id word (`+4`),
/// a handler pointer (`+8`) and a kind word (`+0xC`).
///
/// Algorithm. A TLS flag (slot from TLSIDX, tested word at `+0x8CC`) picks
/// the prologue: set allocates a 0x10 object, stamps the low 14 id bits
/// from the global counter (masked 0x3FFF, counter incremented), plants a
/// scratch vtable then the final one, stores the handler and a kind word
/// (6 when the mode byte is clear, else 4), and registers the object;
/// clear runs a two-word helper instead. Then rows 0..0x5DC are visited
/// (SIGNED `< 0x5DC`; every other comparison is equality or a bit test).
/// A row is skipped early when it equals N, is null, or its mode is not 4.
/// Otherwise the bit tests steer: bit2-clear with bit0-clear goes straight
/// to the float section; bit2-set runs a probe helper whose nonzero answer
/// also reaches it; anything else falls into a second check where a clear
/// bit0 or a set probe answer skips the row (still clearing its mode).
/// The float section forms S = f18 + f10 and D = f14 - f1c in that operand
/// order, builds four quads from S, D, the slot floats and zero (the fifth
/// scratch word is never written, so it reads as zero), and runs the clamp
/// helper four times collecting two floats per call. When the TLS flag is
/// set a 0x50 object is allocated and constructed with ten arguments, then
/// the id-fold pair runs: two virtual calls through slot +8 whose answers
/// feed a SIGNED `% 16` / `/ 16` fold of bits 14..24 into the id word; when
/// clear a tail helper runs and the same blocks go to a nine-argument sink.
/// Every visited row ends with its mode word cleared.
///
/// Edge cases: a null allocator answer is passed to the register helper as
/// null (no fault); a null constructor answer faults on the vtable load
/// (same fault both sides); a faulting row aborts the trial like the
/// original. The stack-cookie prologue/epilogue is anti-tamper scratch
/// below the incoming stack pointer and is not replicated.
///
/// Original: 0x00907DF0 (cdecl, no stack arguments).
/// Shared body: `signed_fold = true` is the faithful rewrite (SIGNED % 16
/// and / 16 in the id-fold); `false` is the deliberately wrong version
/// (the same operations UNSIGNED) used only as the checker's mutant.
unsafe fn body_907df0(signed_fold: bool) -> u32 {
    unsafe {
        const TABLE: u32 = 0x0118_F6F8;
        const COUNT: u32 = 0x0103_4494;
        const CTR: u32 = 0x0103_27A0;
        const MODEW: u32 = 0x0116_09F4; // mode byte is byte 2 of this word
        const TLSIDX: u32 = 0x017A_BA14;
        const TLS_FLAG: u32 = 0x8CC;
        const VT_TMP: u32 = 0x00E7_E048;
        const VT_FIN: u32 = 0x00E8_4C78;
        const DATA_PTR: u32 = 0x0059_D8B0;
        const MAT: u32 = 0x0119_0E70;
        const W_LO: u32 = 0x3D8F_5C29;
        const W_HI: u32 = 0x3F6E_147B;
        const ROWS: i32 = 0x5DC;
        const C_NEW: u32 = 1;
        const C_REG: u32 = 2;
        const C_ALT: u32 = 3;
        const C_PROBE: u32 = 4;
        const C_ROW: u32 = 5;
        const C_CLAMP: u32 = 6;
        const C_CTOR: u32 = 7;
        const C_SINK: u32 = 8;
        const C_TAIL: u32 = 9;
        const C_COOKIE: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }
        #[inline(always)]
        unsafe fn glob_mut(va: u32) -> *mut u32 {
            lf_checker_rt::relocated(va) as *mut u32
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        /// Virtual call through slot +8 (thiscall, no stack args), exactly
        /// like the original: faults identically on a null object.
        #[inline(always)]
        unsafe fn vcall(obj: u32) -> u32 {
            unsafe {
                let vt = rd32(core::hint::black_box(obj));
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(vt.wrapping_add(8)) as usize);
                f(obj)
            }
        }
        /// The id-fold pair: two slot-+8 calls, SIGNED % 16 / / 16, fold of
        /// bits 14..24 into [obj+4].
        #[inline(always)]
        unsafe fn fold_pair(obj: u32, signed: bool) {
            unsafe {
                let r1 = vcall(obj);
                let a = if signed {
                    (16i32.wrapping_sub((r1 as i32).wrapping_rem(16))).wrapping_rem(16)
                } else {
                    (16u32.wrapping_sub(r1.wrapping_rem(16)).wrapping_rem(16)) as i32
                };
                let r2 = vcall(obj);
                let q = if signed {
                    (r2 as i32).wrapping_add(a).wrapping_div(16)
                } else {
                    r2.wrapping_add(a as u32).wrapping_div(16) as i32
                };
                let m = rd32(obj.wrapping_add(4));
                let bits = ((q as u32) << 14 ^ m) & 0x01FF_C000;
                wr32(obj.wrapping_add(4), m ^ bits);
            }
        }
        #[inline(always)]
        unsafe fn row_ptr(table: u32, idx: i32) -> u32 {
            unsafe { rd32(table.wrapping_add((idx as u32).wrapping_mul(4))) }
        }

        let table = lf_checker_rt::relocated(TABLE);
        let idx = glob(TLSIDX);
        let tls_on = rd32(lf_checker_rt::tls_slot(idx as usize).wrapping_add(TLS_FLAG)) != 0;
        let mode = rd8(lf_checker_rt::relocated(MODEW).wrapping_add(2));
        let kind = if mode == 0 { 6u32 } else { 4u32 };
        let mut exit_eax: u32;
        if tls_on {
            let o = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x10, 0);
            if o != 0 {
                wr32(o, lf_checker_rt::relocated(VT_TMP));
                let m = rd32(o.wrapping_add(4));
                let ctr = glob(CTR);
                let c = (m ^ ctr) & 0x3FFF;
                wr32(o.wrapping_add(4), m ^ c);
                glob_mut(CTR).write_unaligned(ctr.wrapping_add(1));
                wr32(o, lf_checker_rt::relocated(VT_FIN));
                wr32(o.wrapping_add(8), lf_checker_rt::relocated(DATA_PTR));
                wr32(o.wrapping_add(0x0C), kind);
                exit_eax = lf_checker_rt::callee_cdecl!(C_REG, u32, o);
            } else {
                exit_eax = lf_checker_rt::callee_cdecl!(C_REG, u32, 0);
            }
        } else {
            exit_eax = lf_checker_rt::callee_cdecl!(C_ALT, u32, 2, kind);
        }

        let n = glob(COUNT) as i32;
        // Scratch frame mirror (zeroed like the checker's stack fill),
        // persistent across rows so stale reads match the original.
        let mut fr = [0u32; 0x40];
        let base = fr.as_mut_ptr() as u32;
        let at = |w: usize| base.wrapping_add((w as u32).wrapping_mul(4));
        let mut esi: i32 = 0;
        loop {
            if esi != n {
                let slot = row_ptr(table, esi);
                if slot != 0 && rd32(slot.wrapping_add(0x0C)) == 4 {
                    let flag8 = rd8(slot.wrapping_add(8));
                    // Bitfield source: own record when flagged, else row N.
                    let src20 = |tn: u32| unsafe {
                        if flag8 != 0 {
                            rd8(slot.wrapping_add(0x20))
                        } else {
                            rd8(tn.wrapping_add(0x20))
                        }
                    };
                    let src21 = |tn: u32| unsafe {
                        if flag8 != 0 {
                            rd8(slot.wrapping_add(0x21))
                        } else {
                            rd8(tn.wrapping_add(0x21))
                        }
                    };
                    let tn = row_ptr(table, n);
                    let mut floats = false;
                    if ((src20(tn) >> 2) & 1) == 0 {
                        if (src21(tn) & 1) == 0 {
                            floats = true;
                        }
                    }
                    if !floats {
                        // L2: bit2-set runs the probe (nonzero reaches the
                        // floats); bit2-clear, or a zero probe answer, falls
                        // into the second check.
                        let probe = if ((src20(tn) >> 2) & 1) == 0 {
                            0u32
                        } else {
                            lf_checker_rt::callee_cdecl!(C_PROBE, u32,) & 0xFF
                        };
                        if ((src20(tn) >> 2) & 1) != 0 && probe != 0 {
                            floats = true;
                        } else {
                            // Second check re-reads the row's own flag byte.
                            let pick = row_ptr(table, esi);
                            let alt = if rd8(pick.wrapping_add(8)) == 0 { tn } else { pick };
                            if (rd8(alt.wrapping_add(0x21)) & 1) != 0
                                && (lf_checker_rt::callee_cdecl!(C_PROBE, u32,) & 0xFF) == 0
                            {
                                floats = true;
                            }
                        }
                    }
                    if floats {
                        let f10 = f32::from_bits(rd32(slot.wrapping_add(0x10)));
                        let f14 = f32::from_bits(rd32(slot.wrapping_add(0x14)));
                        let f18 = f32::from_bits(rd32(slot.wrapping_add(0x18)));
                        let f1c = f32::from_bits(rd32(slot.wrapping_add(0x1C)));
                        wr32(base.wrapping_add(0x20), f1c.to_bits());
                        wr32(base.wrapping_add(0x2C), f10.to_bits());
                        wr32(base.wrapping_add(0x38), f18.to_bits());
                        wr32(base.wrapping_add(0x3C), f14.to_bits());
                        let bit = (src20(tn) >> 1) & 1;
                        let v54 = if flag8 != 0 {
                            rd32(slot.wrapping_add(0x54))
                        } else {
                            rd32(tn.wrapping_add(0x54))
                        };
                        lf_checker_rt::callee_cdecl!(C_ROW, u32, at(5), v54, bit as u32, 0);
                        let col = rd32(at(5));
                        wr32(at(5), (col & 0x00FF_FFFF) | 0x5000_0000);
                        // Constant block and quad geometry.
                        for (w, v) in [
                            (0x09, W_HI), (0x0A, W_HI), (0x0C, W_HI), (0x0D, W_LO),
                            (0x10, W_LO), (0x11, W_LO), (0x12, W_LO), (0x13, W_HI),
                            (0x18, W_LO), (0x19, W_HI), (0x1A, W_LO), (0x1B, W_LO),
                            (0x1C, W_HI), (0x1D, W_HI), (0x1E, W_HI), (0x1F, W_LO),
                        ] {
                            wr32(at(w), v);
                        }
                        let s = add(f18, f10);
                        let d = sub(f14, f1c);
                        let z = 0u32;
                        for (w, v) in [
                            (0x28, s.to_bits()), (0x29, d.to_bits()), (0x2A, z), (0x2B, z),
                            (0x2C, s.to_bits()), (0x2D, f14.to_bits()), (0x2E, z), (0x2F, z),
                            (0x30, f10.to_bits()), (0x31, d.to_bits()), (0x32, z), (0x33, z),
                            (0x34, f10.to_bits()), (0x35, f14.to_bits()), (0x36, z), (0x37, z),
                        ] {
                            wr32(at(w), v);
                        }
                        let mat = lf_checker_rt::relocated(MAT);
                        for k in 0..4u32 {
                            lf_checker_rt::callee_cdecl!(
                                C_CLAMP, u32, at(0x28 + (k as usize) * 4), at(0x14), mat, 1
                            );
                            wr32(at(0x20 + (k as usize) * 2), rd32(at(0x14)));
                            wr32(at(0x20 + (k as usize) * 2 + 1), rd32(at(0x15)));
                        }
                        if tls_on {
                            let o2 = lf_checker_rt::callee_cdecl!(C_NEW, u32, 0x50, 0);
                            if o2 != 0 {
                                let e = lf_checker_rt::callee_thiscall!(
                                    C_CTOR, u32, o2, at(0x20), at(0x22), at(0x24), at(0x26),
                                    at(0x18), at(0x1A), at(0x1C), at(0x1E), rd32(at(5)), 0
                                );
                                fold_pair(e, signed_fold);
                            } else {
                                fold_pair(0, signed_fold);
                            }
                        } else {
                            lf_checker_rt::callee_cdecl!(C_TAIL, u32, 0);
                            lf_checker_rt::callee_cdecl!(
                                C_SINK, u32, at(0x20), at(0x22), at(0x24), at(0x26),
                                at(0x18), at(0x1A), at(0x1C), at(0x1E), at(5)
                            );
                        }
                    }
                    let done = row_ptr(table, esi);
                    wr32(done.wrapping_add(0x0C), 0);
                    exit_eax = done;
                }
            }
            esi = esi.wrapping_add(1);
            if esi >= ROWS {
                break;
            }
        }
        // Stack-guard check the original runs on exit; the stub preserves
        // registers, so this only keeps the call logs aligned.
        lf_checker_rt::callee_cdecl!(C_COOKIE, u32,);
        exit_eax
    }
}

lf_checker_rt::export!(cdecl, rw_00907DF0() -> u32 {
    unsafe { body_907df0(true) }
});
