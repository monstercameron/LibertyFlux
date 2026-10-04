// original: 0x00B5D180 unnamed
/// Step one weapon-slot state machine (`this`): dispatch on the word at
/// +0x1C (0-5, anything else returns unchanged). Most cases only move the
/// state word behind counter and flag gates; case 2 resolves a weapon info
/// entry through callee 1, checks the ped's ducking and posture state
/// (callees 2-4), optionally scales the resolved value by the target's float
/// through the indirect slot at the target table +0x128 (callee 6) behind a
/// global-mode gate (callee 5), then accumulates into the counter at +0x20.
/// Case 3 forwards to callee 7 when the global counter leads. No result.
///
/// Cond: `this` points to a readable/writable object, `s0` (or null) to the
/// target object.
export!(thiscall, rw_b73_f4(this: u32, s0: u32) -> u32 {
    unsafe {
        let idx = *(this.wrapping_add(0x1C) as *const u32);
        if idx > 5 {
            return 0;
        }
        match idx {
            0 => {
                if *(this.wrapping_add(0x5C) as *const u32) != 0 {
                    return 0;
                }
                *(this.wrapping_add(0x1C) as *mut u32) = 2;
            }
            1 => {
                if *(this.wrapping_add(0x60) as *const u32) == 0 {
                    *(this.wrapping_add(0x1C) as *mut u32) = 4;
                    return 0;
                }
                let v = *(this.wrapping_add(0x5C) as *const i32);
                if v == 0 {
                    *(this.wrapping_add(0x1C) as *mut u32) = 3;
                    return 0;
                }
                if v <= 0 {
                    return 0;
                }
                let g = *global::<u32>(0x11735B4);
                if g <= *(this.wrapping_add(0x20) as *const u32) {
                    return 0;
                }
                *(this.wrapping_add(0x1C) as *mut u32) = 0;
            }
            2 => {
                let arg = *(this.wrapping_add(0x18) as *const u32);
                let w0: u32 = callee_cdecl!(1, u32, arg);
                let mut ebx = *(w0.wrapping_add(0x94) as *const u32);
                if (*(s0.wrapping_add(0x28) as *const u32) & 0x3C0) == 0xC0 {
                    let d: u32 = callee_thiscall!(2, u32, s0);
                    if (d & 0xFF) != 0 {
                        let w1: u32 = callee_cdecl!(1, u32, arg);
                        ebx = *(w1.wrapping_add(0x9C) as *const u32);
                    } else {
                        let w2: u32 = callee_cdecl!(1, u32, arg);
                        ebx = *(w2.wrapping_add(0x94) as *const u32);
                        let p: u32 = callee_thiscall!(3, u32, s0);
                        if (p & 0xFF) != 0 {
                            let _: u32 = callee_cdecl!(1, u32, arg);
                            let w4: u32 = callee_cdecl!(1, u32, arg);
                            let c: u32 = callee_cdecl!(
                                4,
                                u32,
                                *(w4.wrapping_add(0x28) as *const u32),
                                0xC8
                            );
                            if c != 0 {
                                let w5: u32 = callee_cdecl!(1, u32, arg);
                                ebx = *(w5.wrapping_add(0x98) as *const u32);
                            }
                        }
                    }
                }
                if *global::<u32>(0x11D6FD4) == 2 {
                    let a: u32 = callee_cdecl!(5, u32,);
                    if (a & 0xFF) != 0 {
                        // Indirect slot through the target's table, like the
                        // original's load-and-call; both sides land on the
                        // same planted stub.
                        let vt = *(s0 as *const u32);
                        let fp = *(vt.wrapping_add(0x128) as *const u32);
                        let t: u32 = {
                            let f: extern "thiscall" fn(u32) -> u32 =
                                core::mem::transmute(fp as usize);
                            f(s0)
                        };
                        if (t & 0xFF) != 0 {
                            let e = *(s0.wrapping_add(0x228) as *const u32);
                            if e != 0 {
                                let x = *(e.wrapping_add(0x5AC) as *const f32);
                                if x > *global::<f32>(0xFE8874) {
                                    // (float)ebx is a SIGNED conversion.
                                    let prod = mulss_exact(ebx as i32 as f32, x);
                                    ebx = cvttss2si_exact(prod) as u32;
                                }
                            }
                        }
                    }
                }
                let g = *global::<u32>(0x11735B4);
                *(this.wrapping_add(0x20) as *mut u32) =
                    g.wrapping_add(ebx);
                *(this.wrapping_add(0x1C) as *mut u32) = 3;
            }
            3 => {
                let g = *global::<u32>(0x11735B4);
                if g <= *(this.wrapping_add(0x20) as *const u32) {
                    return 0;
                }
                let _: u32 = callee_thiscall!(7, u32, this, s0);
            }
            4 => {}
            _ => {
                *(this.wrapping_add(0x1C) as *mut u32) = 0;
            }
        }
        0
    }
});
// Exact-operation helpers used above (shared across this lane's rewrites).
fn mulss_exact(dest: f32, src: f32) -> f32 {
    let a = dest.to_bits();
    if a & 0x7F800000 == 0x7F800000 && a & 0x007FFFFF != 0 {
        return f32::from_bits(a | 0x00400000);
    }
    let b = src.to_bits();
    if b & 0x7F800000 == 0x7F800000 && b & 0x007FFFFF != 0 {
        return f32::from_bits(b | 0x00400000);
    }
    dest * src
}
fn cvttss2si_exact(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}
