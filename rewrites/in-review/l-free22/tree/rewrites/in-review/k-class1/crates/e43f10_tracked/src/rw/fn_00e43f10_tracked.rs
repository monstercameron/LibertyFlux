// original: 0x00E43F10 emit_owner_full_pass
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// x86 `cvttss2si` semantics: truncate toward zero; NaN and out-of-range
/// values (including both infinities and exactly +2^31) yield 0x80000000.
/// (Rust `as` saturates instead, so the edges are handled explicitly.)
fn cvttss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        0x80000000u32 as i32
    } else {
        x as i32
    }
}

/// Read an `f32` field at a byte offset from an object base.
#[inline(always)]
unsafe fn rf(base: u32, off: usize) -> f32 {
    unsafe { (base as *const u8).add(off).cast::<f32>().read() }
}

/// Run the owner's full two-loop pass when enabled (original 0x00E43F10).
///
/// After the shared level/clamp prologue and five table-fill calls, a first
/// loop keeps two running maxima of a measured float per live record, a long
/// straight-line chain mixes those maxima with the filled tables and five
/// image constants, four selected globals scale the owner's fields, and a
/// second loop emits every live record through the guarded quad publisher
/// and a five-argument call. Several float slots the chains read are never
/// written, so they contribute constant zero. Return incidental.
export!(thiscall, rw_00e43f10(this: u32) -> u32 {
    const ENABLE_OFF: usize = 0x44c;
    const COUNT_OFF: usize = 0x313;
    const SECONDARY_OFF: usize = 0x395;
    const TABLE_OFF: usize = 0x50;
    const LOOKUP_ID: u32 = 0x83;
    const LOOKUP_FINE: u32 = 0x7e;
    const FLAG: u32 = 0x01161668;
    const VTABLE_OBJ: u32 = 0x0116BFF0;
    const SEL_A0: u32 = 0x0105C880;
    const SEL_A1: u32 = 0x0105C87C;
    const SEL_B0: u32 = 0x0105C884;
    const SEL_B1: u32 = 0x0105C888;
    const C1: u32 = 0x00FE8830;
    const C2: u32 = 0x00FE8D1C;
    const C3: u32 = 0x00FE8CF8;
    const C4: u32 = 0x00FE88E8;
    const C5: u32 = 0x00FE8A24;
    unsafe {
        let obj = this as *const u8;
        if obj.add(ENABLE_OFF).read() == 0 {
            return 0;
        }
        let mut scratch = [0u32; 1];
        let out = scratch.as_mut_ptr() as u32;
        let p1 = callee_cdecl!(1, u32, out, LOOKUP_ID);
        let mut level = cvttss2si(((p1 + 4) as *const f32).read()) as u8;
        if global::<u8>(FLAG).read() != 0 {
            level = callee_thiscall!(2, u32, relocated(FLAG)) as u8;
        }
        let p2 = callee_cdecl!(1, u32, out, LOOKUP_ID);
        let fi = level as f32;
        let f2 = ((p2 + 4) as *const f32).read();
        let v = if 0.0f32 > fi {
            0.0
        } else if fi <= f2 || f2.is_nan() {
            fi
        } else {
            f2
        };
        let nbyte = cvttss2si(v) as u8;
        let mut id5slot = 0u32;
        let pal = callee_cdecl!(5, u32, &mut id5slot as *mut u32 as u32, 0x41);
        let color = ((nbyte as u32) << 24) | ((pal as *const u32).read() & 0xffffff);
        callee_cdecl!(6, u32, color);
        callee_cdecl!(7, u32, 1);
        let sbyte = (obj.add(0x358).read() as i8) as i32 as u32;
        callee_cdecl!(8, u32, sbyte);
        callee_cdecl!(9, u32, 0);
        callee_cdecl!(10, u32, rf(this, 0x350).to_bits(), rf(this, 0x354).to_bits());
        callee_cdecl!(11, u32, 0, 0x3f800000);
        let mut w_a = 0u32;
        let mut w_b = 0u32;
        let mut w_c = 0u32;
        let mut w_d = 0u32;
        let mut w_e = 0u32;
        callee_cdecl!(1, u32, out, 0x99);
        callee_cdecl!(12, u32, 1, &mut w_a as *mut u32 as u32, 0, 0);
        callee_cdecl!(1, u32, out, 0x9a);
        callee_cdecl!(13, u32, 1, 0, &mut w_b as *mut u32 as u32, 0);
        callee_cdecl!(1, u32, out, 0x9b);
        callee_cdecl!(13, u32, 1, 0, &mut w_c as *mut u32 as u32, 0);
        callee_cdecl!(1, u32, out, 0x9c);
        callee_cdecl!(13, u32, 1, 0, &mut w_d as *mut u32 as u32, 0);
        callee_cdecl!(1, u32, out, 0x9d);
        callee_cdecl!(13, u32, 2, 0, &mut w_e as *mut u32 as u32, 0);
        let (wa, wb, wc, wd, we) = (
            f32::from_bits(w_a),
            f32::from_bits(w_b),
            f32::from_bits(w_c),
            f32::from_bits(w_d),
            f32::from_bits(w_e),
        );
        let count = obj.add(COUNT_OFF).read();
        let secondary = obj.add(SECONDARY_OFF).read();
        let table = (this as *const u32).byte_add(TABLE_OFF).read();
        let mut g6 = 0.0f32;
        let mut g7 = 0.0f32;
        let mut bl = true;
        if count != 0 {
            let mut esi = 0u32;
            loop {
                if !(esi == 1 && secondary == 0) {
                    let rp = table
                        .wrapping_add(esi.wrapping_mul(0x7c))
                        .wrapping_add(0x40);
                    if (rp as *const u8).read() != 0 {
                        let a16 = callee_thiscall!(14, u32, relocated(VTABLE_OBJ), rp);
                        let x: f32 = callee_cdecl!(15, f32, a16, 1);
                        if bl {
                            if x > g6 {
                                g6 = x;
                            }
                        } else if x > g7 {
                            g7 = x;
                        }
                    }
                    bl = !bl;
                }
                esi = esi.wrapping_add(1);
                if !((esi as i32) < (count as i32)) {
                    break;
                }
            }
        }
        let c1 = global::<f32>(C1).read();
        let c2 = global::<f32>(C2).read();
        let c3 = global::<f32>(C3).read();
        let c4 = global::<f32>(C4).read();
        let c5 = global::<f32>(C5).read();
        let eax = (count as u32).wrapping_add(if secondary != 0 { 1 } else { 0 });
        let mut x4 = eax as f32;
        x4 *= c1;
        let mut x3 = f32::from_bits(c2.to_bits() & x4.to_bits());
        let mut x0 = f32::from_bits(x4.to_bits() ^ x3.to_bits());
        x0 = f32::from_bits(if x0 < c3 { 0xFFFFFFFF } else { 0 });
        let mut x1 = x4;
        let mut x2 = f32::from_bits(c3.to_bits() & x0.to_bits());
        x2 = f32::from_bits(x2.to_bits() | x3.to_bits());
        x1 += x2;
        x1 -= x2;
        x0 = x1;
        x0 -= x4;
        x0 = f32::from_bits(if !(x0 < x3) { 0xFFFFFFFF } else { 0 });
        x3 = c4;
        x0 = f32::from_bits(x0.to_bits() & x3.to_bits());
        x1 -= x0;
        x0 = 0.0;
        x0 *= x1;
        x2 = x1;
        x1 = wc;
        x1 += we;
        x2 -= x3;
        x3 = c5;
        x1 += wb;
        x2 *= 0.0;
        x2 += x0;
        x0 = 0.0;
        x1 *= x3;
        x0 *= x3;
        x1 += g6;
        x2 += x0;
        x0 = wd;
        x0 *= x3;
        x1 += g7;
        let v2c = x2;
        x1 += x0;
        x0 = x2;
        x2 = 0.0;
        x0 *= c1;
        let v44 = x1;
        x2 -= x0;
        x0 = x1;
        x1 = wa;
        x0 *= c1;
        let v14 = x2;
        x1 -= x0;
        let v18 = x1;
        let s_a0 = global::<u32>(SEL_A0).read();
        let s_a1 = global::<u32>(SEL_A1).read();
        let s_b0 = global::<u32>(SEL_B0).read();
        let s_b1 = global::<u32>(SEL_B1).read();
        let a1 = callee_cdecl!(3, u32,) as u8 != 0;
        let a2 = callee_cdecl!(3, u32,) as u8 != 0;
        let a3 = callee_cdecl!(3, u32,) as u8 != 0;
        let a4 = callee_cdecl!(3, u32,) as u8 != 0;
        let sel_b = if a1 { s_a1 } else { s_a0 };
        let sel_e = if a2 { s_b1 } else { s_b0 };
        let sel_s = if a3 { s_a1 } else { s_a0 };
        let sel_c = if a4 { s_b1 } else { s_b0 };
        let p8 = callee_cdecl!(1, u32, out, LOOKUP_FINE);
        let mut t1 = rf(this, 0x43c);
        t1 += (p8 as *const f32).read();
        t1 *= (sel_c as i32) as f32;
        let v74 = t1;
        let p9 = callee_cdecl!(1, u32, out, LOOKUP_FINE);
        let mut u1 = rf(this, 0x440);
        u1 += rf(this, 0x320);
        u1 += (p9 as *const f32).read();
        let p10 = callee_cdecl!(1, u32, out, LOOKUP_FINE);
        u1 *= (sel_s as i32) as f32;
        let v1c = u1;
        let mut z0 = (sel_e as i32) as f32;
        z0 *= rf(this, 0x444);
        let v4c = z0;
        let mut y1 = (p10 as *const f32).read();
        y1 *= c5;
        y1 += rf(this, 0x448);
        y1 *= (sel_b as i32) as f32;
        let mut color_slot = 0x96000000u32;
        let mut ivec = [v74, y1, v4c, v1c];
        callee_cdecl!(
            4, u32,
            ivec.as_mut_ptr() as u32,
            &mut color_slot as *mut u32 as u32
        );
        let ncolor = ((nbyte as u32) << 24) | 0xffffff;
        callee_thiscall!(
            16, u32, this, v18.to_bits(), v14.to_bits(), v44.to_bits(), v2c.to_bits(),
            ncolor
        );
        let mut x6 = v18;
        x6 += wd;
        let x2b = we;
        let x3b = wc;
        let mut x0b = wb;
        x0b *= c5;
        let mut x4b = v14;
        x4b += 0.0;
        let mut x1b = x6;
        x1b += x2b;
        let mut cl = true;
        let mut edi = 0u32;
        let x5init = x4b;
        x1b += x3b;
        let x6saved = x6;
        let mut x4l = x4b;
        x1b += x0b;
        let mut x5l = x5init;
        x1b += g6;
        let v2c2 = x1b.to_bits();
        if count != 0 {
            let ebp_bits = x6saved.to_bits();
            loop {
                if !(edi == 1 && secondary == 0) {
                    let (f3c, f40, f1c);
                    if cl {
                        let eax_a = x4l;
                        x4l += 0.0;
                        let mut t = x3b;
                        t += x2b;
                        t += x6saved;
                        f3c = ebp_bits;
                        f40 = eax_a.to_bits();
                        f1c = t.to_bits();
                    } else {
                        let eax_b = x5l;
                        x5l += 0.0;
                        let mut t = x3b;
                        t += x2b;
                        t += f32::from_bits(v2c2);
                        f3c = v2c2;
                        f40 = eax_b.to_bits();
                        f1c = t.to_bits();
                    }
                    let mut f44 = f32::from_bits(f40);
                    f44 += 0.0;
                    let ebx = (edi as i32).wrapping_mul(0x7c) as u32;
                    if (table.wrapping_add(ebx).wrapping_add(4) as *const u8).read() != 0 {
                        callee_thiscall!(
                            17, u32, this, edi, f3c, f40, x2b.to_bits(), 0
                        );
                    }
                    let rp2 = table.wrapping_add(ebx).wrapping_add(0x40);
                    if (rp2 as *const u8).read() != 0 {
                        let a16 = callee_thiscall!(14, u32, relocated(VTABLE_OBJ), rp2);
                        callee_cdecl!(18, u32, f1c, f44.to_bits(), a16, 0xFFFFFFFF, 0xFFFFFFFF);
                    }
                    cl = !cl;
                }
                edi = edi.wrapping_add(1);
                if !((edi as i32) < (count as i32)) {
                    break;
                }
            }
        }
        callee_cdecl!(19, u32,);
        0
    }
});
