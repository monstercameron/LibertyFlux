// original: 0x00cb5870 ped_task_dispatch_large (proposed)

/// Dispatch a ped task request by code across seven creation paths.
///
/// `this` is the requesting object: floats at `+0x18`/`+0x1c`/`+0x20`/`+0x28`,
/// a dword at `+0x24`, a bitfield at `+0x54` (bits 0, 1 and 8 feed the
/// `0x3b5` creation call, bit 8 is cleared afterwards), a handle at `+0x70`,
/// and addressable state at `+0x30`/`+0x40`. `arg1` is a context read at
/// `+0xb30` by two paths; `code` selects the path.
///
/// Paths: `0x2e2` and `0x2cf` create tasks from the context dword;
/// `0xcb` creates a default task; `0x3ae` builds a ten-argument task and
/// finishes it through a follow-up call; `0x3bf` scales an integer draw and
/// creates a task from it, then a second creation whose result feeds a final
/// follow-up; `0x3b5` creates a task from the bitfield and floats, stores
/// two dwords into the result, runs the same second-creation and follow-up
/// pair, and clears bit 8; `0x516` and any other code release the
/// handle when set and return 0, as does a null task manager at most gates.
/// A null manager at the `0x3b5` gate faults on a null write, exactly like
/// the original.
///
/// Successive gate calls on one path use independent scripted answers, so
/// every null/non-null mix is exercised. The float operation order is the
/// original's.
///
/// Original: 0x00cb5870 (thiscall, context then code on the stack), returns
/// the creation call's result, or 0 on the release and manager-missing paths.
lf_checker_rt::export!(thiscall, rw_00cb5870(this: u32, arg1: u32, code: u32) -> u32 {
    unsafe {
        const CTX_DW: u32 = 0xb30;
        const F18: u32 = 0x18;
        const F1C: u32 = 0x1c;
        const F20: u32 = 0x20;
        const DW24: u32 = 0x24;
        const F28: u32 = 0x28;
        const BITS: u32 = 0x54;
        const HANDLE: u32 = 0x70;
        const RES_DW: u32 = 0xa4;
        const RES_F: u32 = 0xa8;
        const G1: u32 = 1;
        const G2: u32 = 2;
        const G3: u32 = 3;
        const CTX4_CALLEE: u32 = 4;
        const CTX2_CALLEE: u32 = 5;
        const DEF_CALLEE: u32 = 6;
        const BIG_CALLEE: u32 = 7;
        const FIN_CALLEE: u32 = 8;
        const REL_CALLEE: u32 = 9;
        const DRAW_CALLEE: u32 = 10;
        const SCALE_CALLEE: u32 = 11;
        const AUX_CALLEE: u32 = 12;
        const BITS_CALLEE: u32 = 13;
        const TASK_MGR_GLOBAL: u32 = 0x167e2a0;
        const EIGHT: f32 = 8.0;
        const ONE_HALF: f32 = 1.5;
        const HALF: f32 = 0.5;
        const THREE: f32 = 3.0;
        const SCALE: f32 = f32::from_bits(0x38000100);

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn gate(id: u32) -> u32 {
            unsafe {
                let g = lf_checker_rt::global::<u32>(TASK_MGR_GLOBAL).read();
                match id {
                    1 => lf_checker_rt::callee_thiscall!(G1, u32, g),
                    2 => lf_checker_rt::callee_thiscall!(G2, u32, g),
                    _ => lf_checker_rt::callee_thiscall!(G3, u32, g),
                }
            }
        }
        #[inline(always)]
        unsafe fn aux_pair(mgr: u32) -> u32 {
            unsafe {
                let mgr2 = gate(3);
                if mgr2 == 0 {
                    return 0;
                }
                lf_checker_rt::callee_thiscall!(
                    AUX_CALLEE, u32, mgr2, 0, (-1.0f32).to_bits(), 0, 0, 0, 1
                )
            }
        }

        let mut c = code;
        loop {
            if c == 0x3ae {
                let mgr = gate(1);
                let task = if mgr == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(
                        BIG_CALLEE, u32, mgr,
                        rdf(this + F20).to_bits(), this.wrapping_add(0x40),
                        HALF.to_bits(), THREE.to_bits(),
                        0xffffffff, 1, 0, 0, 0, 1
                    )
                };
                let mgr2 = gate(2);
                if mgr2 == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    FIN_CALLEE, u32, mgr2, task, 0, 0, 0
                );
            }
            if c > 0x3ae {
                if c == 0x3b5 {
                    let mgr = gate(1);
                    let task = if mgr == 0 {
                        0
                    } else {
                        let bits = rd32(this + BITS);
                        let c2 = (bits >> 8) & 1;
                        let c1 = (bits >> 1) & 1;
                        let c0 = bits & 1;
                        lf_checker_rt::callee_thiscall!(
                            BITS_CALLEE, u32, mgr,
                            rdf(this + F20).to_bits(), rdf(this + F18).to_bits(),
                            c0, rdf(this + F1C).to_bits(), c1, c2
                        )
                    };
                    // A null manager faults here, as does the original.
                    ((task + RES_DW) as *mut u32).write_unaligned(rd32(this + DW24));
                    ((task + RES_F) as *mut u32).write_unaligned(rd32(this + F28));
                    let mgr2 = gate(2);
                    if mgr2 == 0 {
                        ((this + BITS) as *mut u32)
                            .write_unaligned(rd32(this + BITS) & 0xfffffeff);
                        return 0;
                    }
                    let aux = aux_pair(mgr2);
                    let r = lf_checker_rt::callee_thiscall!(
                        FIN_CALLEE, u32, mgr2, task, aux, 0, 0
                    );
                    ((this + BITS) as *mut u32)
                        .write_unaligned(rd32(this + BITS) & 0xfffffeff);
                    return r;
                }
                if c == 0x3bf {
                    let draw: u32 = lf_checker_rt::callee_cdecl!(DRAW_CALLEE, u32,);
                    let x = mul((draw as i32) as f32, SCALE);
                    let mgr = gate(1);
                    let task = if mgr == 0 {
                        0
                    } else {
                        let y = add(mul(x, ONE_HALF), ONE_HALF);
                        lf_checker_rt::callee_thiscall!(
                            SCALE_CALLEE, u32, mgr,
                            y.to_bits(), this.wrapping_add(0x30),
                            this.wrapping_add(0x30), 0xffffffff
                        )
                    };
                    let mgr2 = gate(2);
                    if mgr2 == 0 {
                        return 0;
                    }
                    let aux = aux_pair(mgr2);
                    return lf_checker_rt::callee_thiscall!(
                        FIN_CALLEE, u32, mgr2, task, aux, 0, 0
                    );
                }
                if c == 0x516 {
                    break;
                }
                c = 0x516;
                continue;
            }
            if c == 0xcb {
                let mgr = gate(1);
                if mgr == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    DEF_CALLEE, u32, mgr, 0x7d0, 0, 0, EIGHT.to_bits()
                );
            }
            if c == 0x2cf {
                let mgr = gate(1);
                if mgr == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    CTX2_CALLEE, u32, mgr, rd32(arg1 + CTX_DW), 0x7d0
                );
            }
            if c == 0x2e2 {
                let mgr = gate(1);
                if mgr == 0 {
                    return 0;
                }
                return lf_checker_rt::callee_thiscall!(
                    CTX4_CALLEE, u32, mgr, rd32(arg1 + CTX_DW), 0, 0, 0
                );
            }
            c = 0x516;
        }
        let h = rd32(this + HANDLE);
        if h != 0 {
            lf_checker_rt::callee_cdecl!(REL_CALLEE, u32, h);
        }
        0
    }
});
