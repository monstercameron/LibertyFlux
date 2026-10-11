extern crate lf_checker_rt;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}

#[inline(always)]
unsafe fn rdf(a: u32) -> f32 {
    unsafe { f32::from_bits(rd32(a)) }
}

#[inline(always)]
fn fadd(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

#[inline(always)]
fn fsub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

#[inline(always)]
fn fmul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

#[inline(always)]
unsafe fn rd16i(a: u32) -> i32 {
    unsafe { (a as *const i16).read_unaligned() as i32 }
}

#[inline(always)]
unsafe fn wr32(a: u32, v: u32) {
    unsafe { (a as *mut u32).write_unaligned(v) }
}

#[inline(always)]
unsafe fn wr8(a: u32, v: u8) {
    unsafe { (a as *mut u8).write(v) }
}

// original: 0x00B6EB90 CTaskComplexGoToCarDoorAndStandStill::vf20
/// Evaluates this task's selected update paths for the supplied context.
/// The modeled envelope includes the null-owner and interrupt exits, the gate
/// and count checks, and selected B8 continuation and merge branches. It
/// preserves the observed call order and state writes on those paths.
///
/// The contract proves only its pinned input scenarios: non-default dispatch
/// arms, deeper tail work, and null pointers outside the explicit early-exit
/// cases are not covered. Those paths deliberately reach a marker return and
/// must not be treated as implemented by this narrow proof.
///
/// Arguments use thiscall: `this` points to the task and `arg` to its context.
/// Returns the selected helper result or child-task value, with the same task
/// state updates and observable calls as the original on the proved envelope.

lf_checker_rt::export!(thiscall, rw_00b6eb90(this: u32, arg: u32) -> u32 {
    unsafe {
        const INNER_OFF: u32 = 0x14;
        const SUB_OFF: u32 = 0x08;
        const MARK_OFF: u32 = 0x75;
        const INTR_OFF: u32 = 0x78;
        const ROW_OFF: u32 = 0x21C;
        const ROW_ID_OFF: u32 = 0x12C;
        const GATE_OFF: u32 = 0xF50;
        const GATE_FLAG_OFF: u32 = 0x219;
        const GATE_READY_OFF: u32 = 0xA70;
        const COUNT_OFF: u32 = 0x224;
        const COUNT_VAL_OFF: u32 = 0x264;
        const COUNT_MIN: i32 = 30;
        const VT_FIRST: u32 = 0x0C;
        const HELPER_CALLEE: u32 = 1;
        const GATE_CALLEE: u32 = 2;
        const V0_CALLEE: u32 = 3;
        const B45950_CALLEE: u32 = 4;
        const B506A0_CALLEE: u32 = 5;
        const B461D0_CALLEE: u32 = 6;
        const COOKIE_CALLEE: u32 = 7;
        const V1_CALLEE: u32 = 8;
        const V2_CALLEE: u32 = 9;
        const A49730_CALLEE: u32 = 10;
        const BE5240_CALLEE: u32 = 11;
        const B70C40_CALLEE: u32 = 12;
        const TEN: f32 = f32::from_bits(0x4120_0000); // 10.0
        const STAGE_TRAP: u32 = 0xDEAD_BEEF;

        let inner = rd32(this + INNER_OFF);
        if inner == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }
        if rd8(this + INTR_OFF) != 0 {
            wr8(this + MARK_OFF, 1);
            let r: u32 =
                lf_checker_rt::callee_thiscall!(HELPER_CALLEE, u32, this, 0x516, arg);
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return r;
        }
        if rd32(rd32(arg + ROW_OFF) + ROW_ID_OFF) != 2 {
            let gate = rd32(inner + GATE_OFF);
            if gate != 0 && rd8(gate + GATE_FLAG_OFF) != 0 && rd32(gate + GATE_READY_OFF) == 1
            {
                let sub = rd32(this + SUB_OFF);
                let ok: u32 =
                    lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, sub, arg, 1, 0);
                if (ok as u8) != 0 {
                    let r: u32 =
                        lf_checker_rt::callee_thiscall!(HELPER_CALLEE, u32, this, 0xCB, arg);
                    let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
                    return r;
                } else {
                    let r = rd32(this + SUB_OFF);
                    let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
                    return r;
                }
            }
        }
        let w = rd32(rd32(arg + COUNT_OFF) + COUNT_VAL_OFF);
        if (w as i32) >= COUNT_MIN {
            let first: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(this) + VT_FIRST) as usize);
            let v0 = first(this);
            let mut cell = [0u32; 4];
            let cell_ptr = cell.as_mut_ptr() as u32;
            let _: u32 = lf_checker_rt::callee_thiscall!(B45950_CALLEE, u32, cell_ptr, v0);
            let base = rd32(arg + COUNT_OFF);
            let _: u32 =
                lf_checker_rt::callee_thiscall!(B506A0_CALLEE, u32, base.wrapping_add(0x84), cell_ptr, 0, 1);
            let r = rd32(this + SUB_OFF);
            let _: u32 = lf_checker_rt::callee_thiscall!(B461D0_CALLEE, u32, cell_ptr);
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return r;
        }
        // B5: the count-below path. Any divergence toward B8 hits the
        // stage-2 trap; the stage-2 contract pins every gate to stay out.
        let s8 = rd32(this + SUB_OFF);
        let v1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(s8) + VT_FIRST) as usize);
        if v1(s8) != 0x11d {
            return b8_cont(this, arg, inner, s8);
        }
        let s8b = rd32(s8 + 0x14);
        if s8b == 0 {
            return b8_cont(this, arg, inner, s8);
        }
        let v2: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(s8b) + VT_FIRST) as usize);
        if v2(s8b) != 0x3ae {
            return b8_cont(this, arg, inner, s8);
        }
        let a4: u32 = lf_checker_rt::callee_thiscall!(A49730_CALLEE, u32, inner);
        if a4 == 4 {
            return b8_cont(this, arg, inner, s8);
        }
        let bx: u32 = lf_checker_rt::callee_thiscall!(BE5240_CALLEE, u32, s8, arg);
        if bx == 0 {
            return b8_cont(this, arg, inner, s8);
        }
        let w5c = rdf(bx + 0x5c);
        let cxp = rd32(arg + 0x20);
        let fdx = fsub(rdf(this + 0x60), rdf(cxp + 0x30));
        let fdy = fsub(rdf(this + 0x64), rdf(cxp + 0x34));
        // Continue only for exact equality; an unordered value follows the other path.
        // An unordered or over-limit squared distance also follows the other path.
        let mut go_b7 = w5c != TEN;
        if !go_b7 {
            let len2 = fadd(fmul(fdy, fdy), fmul(fdx, fdx));
            if !(TEN >= len2) {
                go_b7 = true;
            }
        }
        if !go_b7 {
            let ok: u32 = lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, s8, arg, 1, 0);
            if (ok as u8) == 0 {
                return b8_cont(this, arg, inner, s8);
            }
            let r: u32 =
                lf_checker_rt::callee_thiscall!(HELPER_CALLEE, u32, this, 0x516, arg);
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return r;
        }
        // B7.
        let t70: u32 = lf_checker_rt::callee_thiscall!(B70C40_CALLEE, u32, bx);
        if (t70 as u8) == 0 {
            return b8_cont(this, arg, inner, s8);
        }
        let ok2: u32 = lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, s8, arg, 1, 0);
        if (ok2 as u8) == 0 {
            return b8_cont(this, arg, inner, s8);
        }
        if rd8(this + 0x76) != 0 {
            wr8(this + 0x77, 1);
            let r: u32 =
                lf_checker_rt::callee_thiscall!(HELPER_CALLEE, u32, this, 0x3ae, arg);
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return r;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(HELPER_CALLEE, u32, this, 0x387, arg);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        return r;
    }
});

// Handles the count-below continuation, selected gated merges, and the
// supported default dispatch path. Other dispatch and deep-tail paths
// remain outside this proof envelope.
#[inline(never)]
unsafe fn b8_cont(this: u32, arg: u32, inner: u32, s8: u32) -> u32 {
    unsafe {
        const SUB_OFF: u32 = 0x08;
        const VT_FIRST: u32 = 0x0C;
        const VT_THIRD: u32 = 0x30;
        const GATE_CALLEE: u32 = 2;
        const COOKIE_CALLEE: u32 = 7;
        const A49730_CALLEE: u32 = 10;
        const BE5240_CALLEE: u32 = 11;
        const A49160_CALLEE: u32 = 13;
        const A2B350_CALLEE: u32 = 14;
        const B64E30_CALLEE: u32 = 15;
        const AF65D0_CALLEE: u32 = 16;
        const C6C9E0_CALLEE: u32 = 17;
        const A2A180_CALLEE: u32 = 18;
        const V30_CALLEE: u32 = 19;
        const V0C_CALLEE: u32 = 20;
        const ONE: f32 = f32::from_bits(0x3F80_0000); // 1.0
        const STAGE_TRAP: u32 = 0xDEAD_BEEF;

        let v1: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(rd32(rd32(s8) + VT_FIRST) as usize);
        if rd8(arg + 0x219) != 0 {
            // B8-long paths that do not enter the inner work converge at the
            // same dispatch merge below.
            'b8_long: {
                if v1(s8) != 0x11d {
                    break 'b8_long;
                }
                let a4b: u32 = lf_checker_rt::callee_thiscall!(A49730_CALLEE, u32, inner);
                if a4b == 4 {
                    break 'b8_long;
                }
                let _: u32 = lf_checker_rt::callee_thiscall!(A2B350_CALLEE, u32, arg);
                let ber: u32 = lf_checker_rt::callee_thiscall!(BE5240_CALLEE, u32, s8, arg);
                let b64r: u32 = lf_checker_rt::callee_thiscall!(
                    B64E30_CALLEE,
                    u32,
                    arg.wrapping_add(0x2B0)
                );
                if ber == 0 {
                    break 'b8_long;
                }
                if b64r != 0 {
                    let afr: u32 =
                        lf_checker_rt::callee_cdecl!(AF65D0_CALLEE, u32, rd32(b64r + 0x18));
                    if ((rd32(afr + 0x20) >> 7) & 1) != 0 {
                        break 'b8_long;
                    }
                }
                let s228 = rd32(arg + 0x228);
                if rd8(s228.wrapping_add(0x484)) != 0 {
                    b8_s8f_block(this, ber);
                    break 'b8_long;
                }
                let gb = rd32(arg + 0xB94);
                let c1: u32 = lf_checker_rt::callee_cdecl!(C6C9E0_CALLEE, u32, gb, 0x28);
                let c2: u32 = lf_checker_rt::callee_cdecl!(C6C9E0_CALLEE, u32, gb, 0x32);
                if c1 == c2 {
                    b8_s8f_block(this, ber);
                } else {
                    let f: f32 = lf_checker_rt::callee_thiscall!(A2A180_CALLEE, f32, arg, 0, 1);
                    if f >= ONE {
                        wr32(this + 0x18, 4);
                        let v30: extern "thiscall" fn(u32) -> u32 =
                            core::mem::transmute(rd32(rd32(ber) + VT_THIRD) as usize);
                        let r30 = v30(ber);
                        let v0c: extern "thiscall" fn(u32, u32) -> u32 =
                            core::mem::transmute(rd32(rd32(r30) + VT_FIRST) as usize);
                        let _: u32 = v0c(r30, 4);
                    }
                }
            }
        }
        // Every exercised long-branch path converges here.
        let _: u32 = lf_checker_rt::callee_thiscall!(A49160_CALLEE, u32, inner, rd32(this + 0x30));
        let r1 = v1(s8);
        let disp: u32 = if r1 == 0x11d {
            let b = rd32(s8 + 0x14);
            if b == 0 {
                0xC8
            } else {
                let v2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(rd32(rd32(b) + VT_FIRST) as usize);
                v2(b)
            }
        } else {
            v1(s8)
        };
        // Non-default dispatch arms are outside the proof envelope; the marker
        // return below makes that limitation explicit.
        match disp {
            0x387 | 0xCA | 0xCB | 0x384 | 0x389 | 0x3AE => return STAGE_TRAP,
            _ => {}
        }
        // The supported default path leaves the phase field at its entry value.
        if s8 == 0 {
            return STAGE_TRAP;
        }
        let ok: u32 = lf_checker_rt::callee_thiscall!(GATE_CALLEE, u32, s8, arg, 1, 0);
        if (ok as u8) != 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
            return 0;
        }
        let r = rd32(this + SUB_OFF);
        let _: u32 = lf_checker_rt::callee_cdecl!(COOKIE_CALLEE, u32,);
        r
    }
}

#[inline(always)]
unsafe fn b8_s8f_block(this: u32, ber: u32) {
    unsafe {
        const S8F_CALLEE: u32 = 21;
        const VT_FIRST: u32 = 0x0C;
        const VT_THIRD: u32 = 0x30;
        let s8f: u32 = lf_checker_rt::callee_thiscall!(S8F_CALLEE, u32, 0);
        let cl = rd8(s8f + 6) ^ rd8(s8f + 4);
        if cl > 0x7F {
            wr32(this + 0x18, 3);
            let v30: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(rd32(rd32(ber) + VT_THIRD) as usize);
            let r30 = v30(ber);
            let v0c: extern "thiscall" fn(u32, u32) -> u32 =
                core::mem::transmute(rd32(rd32(r30) + VT_FIRST) as usize);
            let _: u32 = v0c(r30, 3);
        }
    }
}
