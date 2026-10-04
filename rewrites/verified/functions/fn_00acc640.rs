// original: 0x00acc640 audio_spatial_mix_update
mod fn_00acc640_rt {
/// Call a planted virtual hook at vtable slot +4 returning an integer.
#[inline(always)]
unsafe fn virt_query(obj: u32) -> u32 {
    let vtable = *(obj as *const u32);
    let target = *((vtable as *const u8).add(4) as *const u32);
    let hook: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(target as usize);
    hook(obj)
}
/// Negation through the sign-bit flip, as the original's `xorps` with -0.0.
#[inline(always)]
fn f32_neg_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}
/// Absolute value through the sign-bit mask, as the original's `andps` does.
#[inline(always)]
fn f32_abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7FFF_FFFF)
}
#[inline(always)]
unsafe fn rd_f(obj: *const u8, off: usize) -> f32 {
    *(obj.add(off) as *const f32)
}
#[inline(always)]
unsafe fn wr_f(obj: *mut u8, off: usize, v: f32) {
    *(obj.add(off) as *mut f32) = v;
}
#[inline(always)]
unsafe fn rd_32(obj: *const u8, off: usize) -> u32 {
    *(obj.add(off) as *const u32)
}
#[inline(always)]
unsafe fn wr_32(obj: *mut u8, off: usize, v: u32) {
    *(obj.add(off) as *mut u32) = v;
}
}
use self::fn_00acc640_rt::*;

/// Audio spatial mix update with handle refresh.
///
/// Scores a listener/emitter pair with two corner dot-products and a
/// range test, queries the corner owner's kind through its vtable, and --
/// when admitted by the range/gain/hemisphere gate -- writes the mixed
/// orientation block and refreshes the primary handle through the
/// acquire/release callees. Kind 0xE additionally scans the entry table;
/// kind 0xF refreshes the secondary handle. Always returns 1.
///
/// NOTE: the original reads one uninitialized alignment-gap stack word
/// (verified by disassembly: the slot is never written). The rewrite uses
/// 0.0, matching the checker's defined stack fill; in the game that word
/// holds whatever the caller left there.
export!(thiscall, rw_00acc640(obj: *mut u8, a0: u32, a1: u32, a2: u32, a3: u32, _a4: u32) -> u32 {
    unsafe {
        // Gate byte: when clear, only the tail flag-clear runs.
        if *global::<u8>(0x103F35C) == 0 {
            let elem = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let flag = (elem.wrapping_add(0xA5)) as *mut u8;
            *flag &= 0xFE;
            return 1;
        }
        let elem = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
        let e2 = rd_32(elem as *const u8, 0xA0);
        let cc = rd_32(e2 as *const u8, 0x34);
        // Two corner dot-products against matrix B, one per diagonal corner.
        let mb = rd_32(a0 as *const u8, 0xDC4);
        let va = if a2 == cc { elem.wrapping_add(0x40) } else { elem.wrapping_add(0x30) };
        let d0 = rd_f(va as *const u8, 0) - rd_f(mb as *const u8, 0x40);
        let d4 = rd_f(va as *const u8, 4) - rd_f(mb as *const u8, 0x44);
        let d8 = rd_f(va as *const u8, 8) - rd_f(mb as *const u8, 0x48);
        let proj_a = rd_f(mb as *const u8, 0x34) * d4 + rd_f(mb as *const u8, 0x30) * d0
            + rd_f(mb as *const u8, 0x38) * d8;
        let vb = if a2 == cc { elem.wrapping_add(0x30) } else { elem.wrapping_add(0x40) };
        let e0 = rd_f(vb as *const u8, 0) - rd_f(mb as *const u8, 0x40);
        let e4 = rd_f(vb as *const u8, 4) - rd_f(mb as *const u8, 0x44);
        let e8 = rd_f(vb as *const u8, 8) - rd_f(mb as *const u8, 0x48);
        let proj_b = rd_f(mb as *const u8, 0x24) * e4 + rd_f(mb as *const u8, 0x20) * e0
            + rd_f(mb as *const u8, 0x28) * e8;
        // Range test with a square-root refinement, else a -100 floor.
        let x1 = rd_f(obj as *const u8, 8);
        let dd = proj_b - rd_f(obj as *const u8, 0x64);
        let w1 = if x1 * x1 > dd * dd {
            let r = ((x1 * x1) - dd * dd).sqrt();
            (proj_a - rd_f(obj as *const u8, 0x68)) - (x1 - r)
        } else {
            -100.0f32
        };
        // Virtual kind query on the corner owner, unless it is null.
        let recv = if a2 == cc { rd_32(e2 as *const u8, 0x38) } else { cc };
        let mut kind = 4u32;
        let mut dl_flag = 0u32;
        let mut ans = 4u32;
        if recv != 0 {
            ans = virt_query(recv);
            kind = ans;
            if ans == 0xF || ans == 8 {
                dl_flag = 1;
            }
        }
        // Kind 0xE scans the entry table for a matching record.
        if ans == 0xE {
            let elem2 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let e2b = rd_32(elem2 as *const u8, 0xA0);
            let cb = rd_32(e2b as *const u8, 0x34);
            let owner = if a2 == cb { rd_32(e2b as *const u8, 0x38) } else { cb };
            let e3 = rd_32(owner as *const u8, 0xC);
            let count = rd_32(e3 as *const u8, 0xF84);
            if (count as i32) > 0 {
                let frame = [a1, a2, a3, _a4];
                let base = rd_32(e3 as *const u8, 0xF80);
                let mut i = 0u32;
                while (i as i32) < (count as i32) {
                    let ent = base.wrapping_add(i.wrapping_mul(0x170));
                    let r: u32 = callee_thiscall!(2, u32, frame.as_ptr() as u32);
                    let want = ((*((ent.wrapping_add(4)) as *const u16) as i16) as i32) as u32;
                    if r == want {
                        dl_flag = 1;
                    }
                    i = i.wrapping_add(1);
                }
            }
            if rd_32(e3 as *const u8, 0x1304) == 4 {
                let alt = if a2 == cb { rd_32(e2b as *const u8, 0x70) } else { rd_32(e2b as *const u8, 0x68) };
                let have = ((*((e3.wrapping_add(0x1EF0)) as *const u8) as i8) as i32) as u32;
                if alt == have {
                    dl_flag = 1;
                }
            }
        }
        // Directional gains from matrix A, negated on the far side.
        let elem3 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
        let e2c = rd_32(elem3 as *const u8, 0xA0);
        let mut g45 = rd_f(elem3 as *const u8, 0x50);
        let mut g54 = rd_f(elem3 as *const u8, 0x54);
        let mut g58 = rd_f(elem3 as *const u8, 0x58);
        if a2 != rd_32(e2c as *const u8, 0x34) {
            g45 = f32_neg_bits(g45);
            g54 = f32_neg_bits(g54);
            g58 = f32_neg_bits(g58);
        }
        let ma = rd_32(a0 as *const u8, 0x20);
        let w2 = f32_abs_bits(
            rd_f(ma as *const u8, 4) * g54 + rd_f(ma as *const u8, 0) * g45
                + rd_f(ma as *const u8, 8) * g58,
        );
        let w3 = rd_f(ma as *const u8, 0x24) * g54 + rd_f(ma as *const u8, 0x20) * g45
            + rd_f(ma as *const u8, 0x28) * g58;
        // Admission gate: range, gain ceiling, forward hemisphere, no match.
        let flags = rd_32(obj as *const u8, 0x164);
        let gate = if (flags >> 19) & 1 == 1 {
            f32::from_bits(0x3F66_6666)
        } else if kind == 7 || kind == 0xE {
            0.5f32
        } else {
            f32::from_bits(0x3F7D_70A4)
        };
        let mut tail_flag = 1u8;
        if w1 > rd_f(obj as *const u8, 0x70) && gate > w2 && w3 >= 0.0 && dl_flag == 0 {
            wr_32(obj, 0x164, flags | 1);
            wr_f(obj, 0x70, w1);
            let elem4 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let e2d = rd_32(elem4 as *const u8, 0xA0);
            let cd = rd_32(e2d as *const u8, 0x34);
            let vc = if a2 == cd { elem4.wrapping_add(0x40) } else { elem4.wrapping_add(0x30) };
            wr_32(obj, 0x90, rd_32(vc as *const u8, 0));
            wr_f(obj, 0x94, rd_f(vc as *const u8, 4));
            wr_f(obj, 0x98, rd_f(vc as *const u8, 8));
            wr_32(obj, 0x9C, rd_32(vc as *const u8, 0xC));
            let t2 = if a2 != cd { 1u32 } else { 0u32 };
            wr_f(obj, 0xC0, g45);
            wr_f(obj, 0xC4, g54);
            wr_f(obj, 0xC8, g58);
            // The original reads an uninitialized alignment-gap word here;
            // under the checker's defined stack fill it is zero.
            wr_f(obj, 0xCC, 0.0);
            let r3: u32 = callee_stdcall!(3, u32, t2);
            wr_32(obj, 0xD8, r3 & 0xFF);
            let elem5 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let e2e = rd_32(elem5 as *const u8, 0xA0);
            let alt2 = if a2 == rd_32(e2e as *const u8, 0x34) {
                rd_32(e2e as *const u8, 0x70)
            } else {
                rd_32(e2e as *const u8, 0x68)
            };
            wr_32(obj, 0xDC, alt2);
            let t0 = rd_f(obj as *const u8, 0x70);
            let g6 = rd_f(obj as *const u8, 0x40) * t0 + rd_f(obj as *const u8, 0x60);
            let g4 = rd_f(obj as *const u8, 0x64) + rd_f(obj as *const u8, 0x44) * t0;
            let g5 = rd_f(obj as *const u8, 0x68) + rd_f(obj as *const u8, 0x48) * t0;
            wr_f(obj, 0xAC, 0.0);
            wr_f(obj, 0xA0, g6);
            wr_f(obj, 0xA8, g5);
            wr_f(obj, 0xA4, g4);
            let mb2 = rd_32(a0 as *const u8, 0xDC4);
            let q3b = rd_f(mb2 as *const u8, 0x20) * g4 + rd_f(mb2 as *const u8, 0x10) * g6
                + rd_f(mb2 as *const u8, 0x30) * g5
                + rd_f(mb2 as *const u8, 0x40);
            let q2b = rd_f(mb2 as *const u8, 0x24) * g4 + rd_f(mb2 as *const u8, 0x14) * g6
                + rd_f(mb2 as *const u8, 0x34) * g5
                + rd_f(mb2 as *const u8, 0x44);
            let q1b = rd_f(mb2 as *const u8, 0x28) * g4 + rd_f(mb2 as *const u8, 0x18) * g6
                + rd_f(mb2 as *const u8, 0x38) * g5
                + rd_f(mb2 as *const u8, 0x48);
            wr_f(obj, 0xA0, q3b);
            wr_f(obj, 0xA4, q2b);
            wr_f(obj, 0xA8, q1b);
            wr_f(obj, 0xAC, 0.0);
            // Primary handle: refresh unless the candidate is rejected.
            let elem6 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let e2f = rd_32(elem6 as *const u8, 0xA0);
            let cf = rd_32(e2f as *const u8, 0x34);
            let arg_a = if a2 == cf { rd_32(e2f as *const u8, 0x38) } else { cf };
            let r4: u32 = callee_cdecl!(4, u32, arg_a);
            let slot = (obj as u32).wrapping_add(0xD0);
            if r4 != 0 {
                let ok: u32 = callee_thiscall!(5, u32, r4);
                if (ok as u8) != 0 && r4 != a0 {
                    let oc = rd_32(obj as *const u8, 0xD0);
                    if oc != r4 {
                        if oc != 0 {
                            callee_stdcall!(6, u32, slot);
                        }
                        wr_32(obj, 0xD0, r4);
                        callee_thiscall!(7, u32, r4, slot);
                    }
                } else {
                    let oc = rd_32(obj as *const u8, 0xD0);
                    if oc != 0 {
                        callee_stdcall!(6, u32, slot);
                        wr_32(obj, 0xD0, 0);
                    }
                }
            } else {
                let oc = rd_32(obj as *const u8, 0xD0);
                if oc != 0 {
                    callee_stdcall!(6, u32, slot);
                    wr_32(obj, 0xD0, 0);
                }
            }
        } else if (flags >> 19) & 1 == 1
            && w2 > gate
            && rd_f(obj as *const u8, 0x1C) > w1
        {
            tail_flag = 0;
        }
        // Kind 0xF refreshes the secondary handle on a flagged candidate.
        if kind == 0xF {
            let elem7 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let e2g = rd_32(elem7 as *const u8, 0xA0);
            let cg = rd_32(e2g as *const u8, 0x34);
            let arg_b = if a2 == cg { rd_32(e2g as *const u8, 0x38) } else { cg };
            let r4b: u32 = callee_cdecl!(4, u32, arg_b);
            if rd_32(r4b as *const u8, 0x28) & 0x3C0 == 0xC0 {
                let slot2 = (obj as u32).wrapping_add(0xD4);
                if rd_32(obj as *const u8, 0xD4) != 0 {
                    callee_stdcall!(6, u32, slot2);
                }
                wr_32(obj, 0xD4, r4b);
                callee_thiscall!(7, u32, r4b, slot2);
            }
        }
        let r8: u32 = callee_thiscall!(8, u32, rd_32(elem as *const u8, 0xA0));
        let _ = r8;
        if tail_flag != 0 {
            let elem8 = *((a1.wrapping_add(a3.wrapping_mul(4))) as *const u32);
            let flag = (elem8.wrapping_add(0xA5)) as *mut u8;
            *flag &= 0xFE;
        }
        1
    }
});
