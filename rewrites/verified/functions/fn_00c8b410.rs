// original: 0x00c8b410 audio_voice_build
//! Lane r-b28 second rewrite: audio voice builder (original 0x00C8B410).
//! See the export's doc comment for the behaviour contract.
// Small helpers to keep the transcription readable. All offsets and the
// operation order match the original exactly.
#[inline(always)]
unsafe fn rf32(p: *mut u8, off: usize) -> f32 {
    f32::from_bits(*(p.add(off) as *const u32))
}
#[inline(always)]
unsafe fn wf32(p: *mut u8, off: usize, v: f32) {
    *(p.add(off) as *mut f32) = v;
}
#[inline(always)]
unsafe fn ru32(p: *mut u8, off: usize) -> u32 {
    *(p.add(off) as *const u32)
}
/// Bit-exact cvttss2si: NaN, infinities and out-of-range values yield
/// i32::MIN (Rust's `as` saturates instead, which differs).
#[inline(always)]
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Shared epilogue: zeroes the +0x14 lane of the first `[this+0xA]` rows and
/// returns that count clamped at zero. (A `comiss`/`jbe`-style branch is
/// written `!(a > b)` so NaN takes the same path as the original.)
#[inline(always)]
unsafe fn tail_end(this_ptr: *mut u8) -> u32 {
    let n = *(this_ptr.add(0xA) as *const i16);
    if n > 0 {
        let arr = ru32(this_ptr, 4) as *mut u8;
        let mut ed = 0i32;
        let mut off = 0usize;
        while ed < n as i32 {
            ed += 1;
            *((arr.add(off)).add(0x14) as *mut u32) = 0;
            off += 0x20;
        }
        n as u32
    } else {
        0
    }
}

/// Shared tail block: conditional effect-chain poll followed by a possible
/// 0.6 scaling of `[this]`, then the epilogue.
#[inline(always)]
unsafe fn tail_block(this_ptr: *mut u8, src: *mut u8) -> u32 {
    let e = ru32(src, 0x25C);
    if e != 0 && ru32(e as *mut u8, 0x18) != 0x2E {
        wf32(
            this_ptr,
            0,
            rf32(this_ptr, 0) * *global::<f32>(0x00FE8858),
        );
        return tail_end(this_ptr);
    }
    let q: u32 = callee_thiscall!(7, u32, src as u32);
    let r = ru32(q as *mut u8, 0xC);
    if r == 0 {
        return tail_end(this_ptr);
    }
    let mut ocx = r;
    if *((r as *mut u8).add(4) as *const u8) == 0xC {
        let bound = *((r as *mut u8).add(0x90) as *const u16);
        let mut ans = 0u32;
        let mut i = 0u32;
        if 0u16 < bound {
            loop {
                if ans != 0 {
                    break;
                }
                ans = callee_stdcall!(8, u32, i, ans);
                i += 1;
                if !((i as i32) < (bound as i32)) {
                    break;
                }
            }
        }
        ocx = if ans != 0 { ans } else { r };
    }
    let vt = ru32(ocx as *mut u8, 0);
    let poll: extern "thiscall" fn(u32, u32) -> u32 =
        core::mem::transmute(*((vt as *mut u8).add(0x58) as *const u32) as usize);
    let s = poll(ocx, 0);
    if s == 0 {
        return tail_end(this_ptr);
    }
    if !(rf32(s as *mut u8, 0x14) > *global::<f32>(0x00FE8C58)) {
        return tail_end(this_ptr);
    }
    wf32(
        this_ptr,
        0,
        rf32(this_ptr, 0) * *global::<f32>(0x00FE8858),
    );
    tail_end(this_ptr)
}

/// Audio voice builder (original 0x00C8B410, thiscall/5).
///
/// Dispatches on the source mode (`[src+0x28] >> 6 & 0xF`) with a flag byte
/// (`a4 != 0` forwards to the sibling mixer as long as the mode is not 3):
/// mode 3 walks a sample bank (sub-mode `[src+0x7B8] == 6`, otherwise a
/// constant fill), mode 2 solves a small clamped grid through an out-param
/// callee, mode 4 builds a sized grid through two virtual samples and a
/// size-checked allocation, and any other mode skips straight to the shared
/// epilogue. Modes 2, 3 and the forward path rejoin at the epilogue; mode 4
/// additionally runs the shared effect-chain tail block. The return value is
/// the epilogue row count. Float operation order, `comiss` NaN semantics
/// (`jbe`-not-taken is a strict `>` in Rust) and `cvttss2si` saturation are
/// all reproduced bit-exactly.
export!(thiscall, rw_rb28_f2(this_ptr: *mut u8, a0: u32, a1: u32, a2: u32, src: *mut u8, a4: u32) -> u32 {
    unsafe {
        *(this_ptr.add(0xA) as *mut u16) = (a1 & 0xFFFF) as u16;
        *(this_ptr.add(8) as *mut u16) = (a2 & 0xFFFF) as u16;
        *(this_ptr.add(4) as *mut u32) = a0;
        *(this_ptr.add(0x24) as *mut u8) = 0;
        let flag = (ru32(src, 0x28) >> 6) & 0xF;
        let mut acc = 0.0f32;
        if flag == 3 {
            if ru32(src, 0x7B8) == 6 {
                let q0 = ru32(src, 0x7B4);
                if q0 == 0 {
                    return tail_end(this_ptr);
                }
                // Bank walk.
                let p0: u32 = callee_thiscall!(0, u32, q0);
                let bv = *((p0 as *mut u8).add(0x1F3) as *const u8) as u32;
                let arr = ru32(this_ptr, 4) as *mut u8;
                let h8 = *(this_ptr.add(8) as *const i16);
                let c01 = *global::<f32>(0x00FE879C);
                if h8 > 0 {
                    let mut count = 0u32;
                    let mut off = 0usize;
                    loop {
                        wf32(arr, off + 0x10, 0.0);
                        if count < bv {
                            let abase = ru32(p0 as *mut u8, 0xD4);
                            let ent = *(((abase as *mut u8).add((count * 4) as usize)) as *const u32);
                            let fv = rf32(ent as *mut u8, 4);
                            let dv: f32 = callee_thiscall!(1, f32, p0);
                            let mut v = fv / dv;
                            v *= (bv as f32) * c01;
                            v = if count.wrapping_sub(7) <= 3 {
                                v * *global::<f32>(0x00FE8914)
                            } else if count == 3 || count == 6 || count == 0x10 || count == 0x14 {
                                v * *global::<f32>(0x00FE8898)
                            } else if count > 0xC {
                                v * *global::<f32>(0x00FE88B0)
                            } else if count >= 7 {
                                v
                            } else {
                                v * *global::<f32>(0x00FE88C4)
                            };
                            wf32(arr, off + 0x10, v);
                        }
                        acc += rf32(arr, off + 0x10);
                        count += 1;
                        off += 0x20;
                        if !((count as i32) < (h8 as i32)) {
                            break;
                        }
                    }
                }
                let s = *global::<f32>(0x00FE88E8) / acc;
                *(this_ptr.add(0x10) as *mut u32) = 0;
                // Note: the bank path jumps into the middle of the tail's
                // scale sequence, storing s*10.78 directly (no 0.6 scaling).
                let g = s * *global::<f32>(0x00ED6B54);
                let g10 = s * *global::<f32>(0x00ED6B58);
                wf32(this_ptr, 0x14, g);
                wf32(this_ptr, 0x18, g);
                wf32(this_ptr, 0x1C, g);
                wf32(this_ptr, 0, g10);
                return tail_end(this_ptr);
            }
            // Constant fill (writes through a0, which aliases [this+4]).
            *(a0 as *mut u32) = 0;
            *((a0 as *mut u8).add(4) as *mut u32) = 0;
            *((a0 as *mut u8).add(8) as *mut u32) = 0xBE19999A;
            let arr = ru32(this_ptr, 4) as *mut u8;
            *((arr).add(0x10) as *mut u32) = 0x3F51EB85;
            *((arr).add(0x20) as *mut u32) = 0;
            *((arr).add(0x24) as *mut u32) = 0;
            *((arr).add(0x28) as *mut u32) = 0x3E800000;
            *((arr).add(0x30) as *mut u32) = 0x3ED1EB85;
            let d30 = rf32(arr, 0x30);
            wf32(this_ptr, 0, rf32(src, 0x120) * *global::<f32>(0x00E99B0C) / d30);
            let q = *global::<f32>(0x00ED6B54) / d30;
            wf32(this_ptr, 0x14, q);
            wf32(this_ptr, 0x18, q);
            *(this_ptr.add(0x10) as *mut u32) = 0;
            wf32(this_ptr, 0x1C, q);
            return tail_end(this_ptr);
        }
        if (a4 & 0xFF) != 0 {
            let dxs = ((a2 & 0xFFFF) as u16 as i16 as i32) as u32;
            callee_thiscall!(2, u32, this_ptr as u32, dxs, a0, src as u32);
            return tail_end(this_ptr);
        }
        if flag == 2 {
            // Clamped grid through the out-param solver.
            let mut buf_a = [0u32; 3];
            let mut buf_b = [0u32; 3];
            callee_thiscall!(
                3, u32, this_ptr as u32, src as u32,
                buf_b.as_mut_ptr() as u32, buf_a.as_mut_ptr() as u32
            );
            let fa = |i: usize| f32::from_bits(buf_a[i]);
            let fb = |i: usize| f32::from_bits(buf_b[i]);
            let cxh = *(this_ptr.add(8) as *const u16);
            let e = if cxh == 9 { 3 } else if cxh == 4 { 2 } else { 0 };
            let c05 = *global::<f32>(0x00FE8830);
            let c10 = *global::<f32>(0x00FE88E8);
            let x4 = (fa(2) - fb(2)) * c05;
            let x7 = fa(0) - fb(0);
            let x3 = fa(1) - fb(1);
            let x1 = c10 / (x4 * *global::<f32>(0x00FE8A24));
            let mut cc = cvtt(x7 * x1);
            let mut ss = cvtt(x3 * x1);
            cc = if cc < 2 { 2 } else if cc > e { e } else { cc };
            ss = if ss < 2 { 2 } else if ss > e { e } else { ss };
            let x7d = x7 / (cc as f32);
            let prod = (cc as u32).wrapping_mul(ss as u32);
            *(this_ptr.add(8) as *mut u16) = prod as u16;
            let x3d = x3 / (ss as f32);
            let mut x2 = x7d * c05 + fb(0);
            let x5f = fb(2) + x4;
            let arr = ru32(this_ptr, 4) as *mut u8;
            if cc > 0 {
                let x0b = x3d * c05 + fb(1);
                let h8b = *(this_ptr.add(8) as *const i16);
                let mut ed = 0i32;
                let mut cc2 = cc;
                while cc2 != 0 {
                    let mut x1i = x0b;
                    if ss > 0 {
                        let mut ss2 = ss;
                        while ss2 != 0 {
                            if ed < h8b as i32 {
                                let off = (ed as u32).wrapping_mul(32) as usize;
                                wf32(arr, off, x2);
                                wf32(arr, off + 4, x1i);
                                wf32(arr, off + 8, x5f);
                                wf32(arr, off + 0x10, x4);
                                acc += rf32(arr, off + 0x10);
                            }
                            ed += 1;
                            x1i += x3d;
                            ss2 -= 1;
                        }
                    }
                    x2 += x7d;
                    cc2 -= 1;
                }
            }
            let s6 = c10 / acc;
            wf32(
                this_ptr,
                0,
                rf32(src, 0x120) * *global::<f32>(0x00E99B0C) * s6,
            );
            *(this_ptr.add(0xC) as *mut u32) = 0;
            wf32(this_ptr, 0x10, s6 * *global::<f32>(0x00FE88CC));
            wf32(this_ptr, 0x14, s6 * *global::<f32>(0x00ED6B50));
            wf32(this_ptr, 0x18, s6 * *global::<f32>(0x00ED6B54));
            wf32(this_ptr, 0x1C, s6 * *global::<f32>(0x00E9B9D4));
            return tail_end(this_ptr);
        }
        if flag != 4 {
            return tail_end(this_ptr);
        }
        // Sized grid through two virtual samples.
        let dxu = (a2 & 0xFFFF) as u16;
        if dxu == 0 {
            let v = rf32(src, 0x120) * *global::<f32>(0x00E99B0C);
            *(this_ptr.add(0x1C) as *mut u32) = 0x3FFAE148;
            *(this_ptr.add(0x18) as *mut u32) = 0x3FFAE148;
            *(this_ptr.add(0x14) as *mut u32) = 0x3FFAE148;
            *(this_ptr.add(0xC) as *mut u32) = 0;
            *(this_ptr.add(0x10) as *mut u32) = 0;
            wf32(this_ptr, 0, v);
            return tail_block(this_ptr, src);
        }
        if a0 != 0 {
            return tail_block(this_ptr, src);
        }
        if dxu != 0xFFFF {
            return tail_block(this_ptr, src);
        }
        let vtable = ru32(src, 0);
        let g60: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            *(((vtable as *mut u8).add(0x60)) as *const u32) as usize,
        );
        let va = g60(src as u32);
        let g64: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
            *(((vtable as *mut u8).add(0x64)) as *const u32) as usize,
        );
        let vb = g64(src as u32);
        let c05 = *global::<f32>(0x00FE8830);
        let dx3 = rf32(vb as *mut u8, 0) - rf32(va as *mut u8, 0);
        let dz = rf32(vb as *mut u8, 8) - rf32(va as *mut u8, 8);
        let dy = rf32(vb as *mut u8, 4) - rf32(va as *mut u8, 4);
        let c60 = *global::<f32>(0x00FE8AE0);
        let c10 = *global::<f32>(0x00FE88E8);
        let c03 = *global::<f32>(0x00FE87E8);
        let mut x3 = dx3 * c05;
        let mut d = [1i32, 1i32, 2i32];
        let axes = [dx3, dy, dz];
        for k in 0..3 {
            let v = axes[k];
            if v > c60 {
                let q = cvtt(v * c05);
                d[k] = if q > 5 { 5 } else { q };
            } else if v > c10 {
                d[k] = 3;
            } else if v > c03 {
                d[k] = 2;
            }
            // else: keep the initial dimension (covers v <= 0.3 and NaN,
            // both of which take every jbe).
            let t = v * c05;
            if x3 > t {
                x3 = t / (d[k] as f32);
            }
        }
        let (mut d0, mut d1, mut d2) = (d[0], d[1], d[2]);
        let mut ax = d0.wrapping_mul(d1).wrapping_mul(d2) as u16;
        *(this_ptr.add(8) as *mut u16) = ax;
        if (ax as i16) > 0x2D {
            let mut ea = 0u32;
            if !(dx3 > dy) {
                ea = 1;
            }
            if dz > dx3 && dz > dy {
                ea = 2;
            }
            // Note: the ea==0 path falls through into the clamp sequence and
            // still clamps (d1,d2); ea==1 clamps (d0,d2); ea==2 clamps (d0,d1).
            if ea == 0 {
                if d1 > 3 {
                    d1 = 3;
                }
                if d2 > 3 {
                    d2 = 3;
                }
            } else {
                if d0 > 3 {
                    d0 = 3;
                }
                if ea == 1 {
                    if d2 > 3 {
                        d2 = 3;
                    }
                } else if d1 > 3 {
                    d1 = 3;
                }
            }
            ax = d0.wrapping_mul(d1).wrapping_mul(d2) as u16;
        }
        *(this_ptr.add(8) as *mut u16) = ax;
        *(this_ptr.add(0xA) as *mut u16) = ax;
        let size = {
            let p = ((ax as i16 as i32) as u32) as u64 * 0x20;
            if p > 0xFFFFFFFF { 0xFFFFFFFF } else { p as u32 }
        };
        let arr: u32 = callee_cdecl!(4, u32, size);
        *(this_ptr.add(4) as *mut u32) = arr;
        *(this_ptr.add(0x24) as *mut u8) = 1;
        let vc = g60(src as u32);
        let bz = rf32(vc as *mut u8, 8);
        let x1u = dx3 / (d0 as f32);
        let yu = dy / (d1 as f32);
        let zu = dz / (d2 as f32);
        let mut x4 = x1u * c05 + rf32(vc as *mut u8, 0);
        let arr8 = arr as *mut u8;
        let h8c = *(this_ptr.add(8) as *const i16);
        let mut ed = 0i32;
        // Note: the outer loop-back jumps past the x5 reload, so every outer
        // reuses the first outer's x5 (the rescale value); the mid counter
        // slot is never re-read.
        let x5 = x3;
        let x1m = yu * c05 + rf32(vc as *mut u8, 4);
        let mut cc0 = d0;
        if cc0 > 0 {
            while cc0 != 0 {
                let mut x2 = x1m;
                if d1 > 0 {
                    let mut mid = d1;
                    while mid != 0 {
                        let mut x1i = zu * c05 + bz;
                        if d2 > 0 {
                            let mut inn = d2;
                            while inn != 0 {
                                if ed < h8c as i32 {
                                    let off = (ed as u32).wrapping_mul(32) as usize;
                                    wf32(arr8, off, x4);
                                    wf32(arr8, off + 4, x2);
                                    wf32(arr8, off + 8, x1i);
                                    wf32(arr8, off + 0x10, x5);
                                    acc += rf32(arr8, off + 0x10);
                                }
                                ed += 1;
                                x1i += zu;
                                inn -= 1;
                            }
                        }
                        x2 += yu;
                        mid -= 1;
                    }
                }
                x4 += x1u;
                cc0 -= 1;
            }
        }
        let s = c10 / acc;
        let v0 = rf32(src, 0x120) * *global::<f32>(0x00E99B0C) * s;
        let s2 = s * *global::<f32>(0x00ED6B54);
        wf32(this_ptr, 0x1C, s2);
        wf32(this_ptr, 0x18, s2);
        wf32(this_ptr, 0x14, s2);
        wf32(this_ptr, 0, v0);
        tail_block(this_ptr, src)
    }
});
