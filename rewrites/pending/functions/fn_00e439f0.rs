// original: 0x00E439F0 emit_owner_records
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

/// Run the owner's record pass when enabled (original 0x00E439F0).
///
/// Reads the level/clamp like its siblings, derives four scaled vectors from
/// the selected global pair and the owner's fields, publishes them through
/// two calls, then walks the owner's sub-records: each live record is visited
/// through one call, measured through a second, and emitted through the
/// guarded quad publisher. A reload byte can fast-forward the walk, and the
/// last call's second float argument always reads a zeroed scratch slot.
/// Return incidental.
export!(thiscall, rw_00e439f0(this: u32) -> u32 {
    const ENABLE_OFF: usize = 0x44c;
    const COUNT_OFF: usize = 0x313;
    const RELOAD_OFF: usize = 0x3cc;
    const TABLE_OFF: usize = 0x50;
    const LOOKUP_ID: u32 = 0x83;
    const FLAG: u32 = 0x01161668;
    const SEL_A0: u32 = 0x0105C880;
    const SEL_A1: u32 = 0x0105C87C;
    const SEL_B0: u32 = 0x0105C884;
    const SEL_B1: u32 = 0x0105C888;
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
        let f43c = rf(this, 0x43c);
        let f440 = rf(this, 0x440);
        let b313 = obj.add(COUNT_OFF).read();
        let f314 = rf(this, 0x314);
        let f318 = rf(this, 0x318);
        let f31c = rf(this, 0x31c);
        let f320 = rf(this, 0x320);
        let mut x0 = f314;
        x0 += f43c;
        x0 += f318;
        let mut x1 = ((b313 as i32) - 2) as f32;
        x1 *= f31c;
        x1 += x0;
        let t1 = x1;
        let t0 = f440 + f320;
        let color_a = ((nbyte as u32) << 24) | 0x10000;
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
        let g3 = (sel_c as i32) as f32 * f43c;
        let g2 = (sel_s as i32) as f32 * f440;
        let g1 = (sel_e as i32) as f32 * t1;
        let g0 = (sel_b as i32) as f32 * t0;
        let mut color_slot = color_a;
        let mut vec = [g3, g0, g1, g2];
        callee_cdecl!(4, u32, vec.as_mut_ptr() as u32, &mut color_slot as *mut u32 as u32);
        let pal = callee_cdecl!(5, u32, &mut color_slot as *mut u32 as u32, 0x41);
        let color_b = ((nbyte as u32) << 24) | ((pal as *const u32).read() & 0xffffff);
        callee_cdecl!(6, u32, color_b);
        callee_cdecl!(7, u32, 1);
        let sbyte = (obj.add(0x358).read() as i8) as i32 as u32;
        callee_cdecl!(8, u32, sbyte);
        callee_cdecl!(9, u32, 0);
        let f350 = rf(this, 0x350);
        let f354 = rf(this, 0x354);
        callee_cdecl!(10, u32, f350.to_bits(), f354.to_bits());
        let count = obj.add(COUNT_OFF).read() as u32;
        let table = (this as *const u32).byte_add(TABLE_OFF).read();
        let f328 = rf(this, 0x328);
        let d348 = (this as *const u32).byte_add(0x348).read();
        let d34c = (this as *const u32).byte_add(0x34c).read();
        let mut g3slot = g3;
        let mut fill_slot = 0u32;
        let mut esi = 0u32;
        let mut edi = 0u32;
        if (count as i32) > 0 {
            loop {
                callee_thiscall!(
                    11, u32, this,
                    &mut g3slot as *mut f32 as u32,
                    &mut fill_slot as *mut u32 as u32,
                    edi, esi
                );
                let rec = (table.wrapping_add(esi.wrapping_mul(0x7c)).wrapping_add(4)
                    as *const u8)
                    .read();
                if rec != 0 {
                    let mut bx = f328;
                    bx += f314;
                    if esi != 0 {
                        bx += f318;
                    }
                    let bval: f32;
                    if esi == 0 {
                        bval = bx;
                    } else if esi == 1 {
                        bval = bx;
                    } else {
                        let mut q = ((edi as i32).wrapping_sub(1)) as f32;
                        q *= f31c;
                        q += bx;
                        bval = q;
                    }
                    callee_cdecl!(12, u32, fill_slot, bval.to_bits());
                    callee_thiscall!(13, u32, this, esi, fill_slot, 0, d348, d34c);
                }
                if esi == 1 {
                    let rb = obj.add(RELOAD_OFF).read() as i8;
                    if rb > 0 {
                        esi = rb as u32;
                    }
                }
                esi = esi.wrapping_add(1);
                edi = edi.wrapping_add(1);
                if !((esi as i32) < (count as i32)) {
                    break;
                }
            }
        }
        callee_cdecl!(14, u32,);
        0
    }
});
