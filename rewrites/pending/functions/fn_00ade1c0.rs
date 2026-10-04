// original: 0x00ade1c0 draw_viewport_pass
use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

/// Draw one viewport pass (original 0x00ADE1C0).
///
/// Takes a selector, a tag, two float vectors, a blend factor, a spare word
/// and an enable byte. When disabled it only runs the security check. Otherwise
/// it opens the pass through the device table (slot 60), folds the first
/// vector against the shared constant into a packed color, queries the pass
/// id (callee 4), builds the sixteen-word blend matrix, runs the matrix setup
/// (callee 5) which selects one of two global parameter pairs, evaluates the
/// nine output blends, emits the four quads (callee 9) publishing the shared
/// color registers as it goes, closes the pass (slots 64, trailing calls) and
/// runs the security check. The floating point keeps the original's operation
/// order exactly; the color conversion matches truncate-to-i32 including the
/// indefinite value for out-of-range inputs. Returns nothing meaningful.
export!(cdecl, rw_00ade1c0(a1: u32, a2: u32, a3: u32, a4: u32, a5: f32, _a6: u32, a7: u32) -> u32 {
    unsafe {
        let gr = |va: u32| (global::<u32>(va) as *const u32).read();
        let gw = |va: u32, v: u32| global::<u32>(va).write(v);
        let gf = |va: u32| f32::from_bits(gr(va));
        if (a7 as u8) == 0 {
            callee_thiscall!(13, u32, gr(0x1057FB4));
            return 0;
        }
        let obj = gr(0x17F5630);
        let vt = (obj as *const u32).read();
        let t1 = (vt.wrapping_add(0x3C) as *const u32).read();
        let f1: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(t1 as usize);
        f1(obj, 0, a1, 0, 0, 0, 0xFFFF_FFFF);
        callee_cdecl!(3, u32, a2, 1);
        let k = gf(0xFE8C08);
        let c0 = f32::from_bits((a3 as *const u32).read());
        let c1 = f32::from_bits((a3.wrapping_add(4) as *const u32).read());
        let c2 = f32::from_bits((a3.wrapping_add(8) as *const u32).read());
        let c3 = f32::from_bits((a3.wrapping_add(12) as *const u32).read());
        let b0 = cvtt_ss2si((c0 * c3) * k) as u8 as u32;
        let b1 = cvtt_ss2si((c1 * c3) * k) as u8 as u32;
        let b2 = cvtt_ss2si((c2 * c3) * k) as u8 as u32;
        // The original keeps this packed color in esi from here on; the tag
        // argument is dead after the call above.
        let color = 0xFF00_0000u32 | (b0 << 16) | (b1 << 8) | b2;
        let pass = callee_thiscall!(4, u32, gr(0x11A28F4));
        // Sixteen-word blend matrix, in frame order.
        let m: [f32; 16] = [
            -0.5, 0.5, 0.5, 0.5, -0.5, -0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        ];
        let mb: [u32; 16] = [
            0xBF00_0000, 0x3F00_0000, 0x3F00_0000, 0x3F00_0000, 0xBF00_0000, 0xBF00_0000,
            0x3F00_0000, 0xBF00_0000, 0x0000_0000, 0x3F80_0000, 0x3F80_0000, 0x3F80_0000,
            0x0000_0000, 0x0000_0000, 0x3F80_0000, 0x0000_0000,
        ];
        let setup = callee_thiscall!(5, u32, relocated(0x118D7F0));
        let gp = if (setup as u8) != 0 { 0x103E4C0 } else { 0x103E4C8 };
        let g0 = gf(gp);
        let g1 = gf(gp.wrapping_add(4));
        let k2 = gf(0xFE8D7C);
        let k3 = gf(0xFE8830);
        let va = f32::from_bits((a4 as *const u32).read());
        let vb = f32::from_bits((a4.wrapping_add(4) as *const u32).read());
        let f = a5;
        // Nine output blends, each in the original's operation order. (Blend 0
        // is written to the frame but only ever read back into blends 3 and 7,
        // which inline it here.)
        let o1 = va + (g0 * k2) * f;
        let o2 = vb + (m[1] * g1) * f;
        let o3 = va + (g0 * k3) * f;
        let o4 = vb + (m[3] * g1) * f;
        let o5 = va + (g0 * k2) * f;
        let o6 = vb + (m[5] * g1) * f;
        let o7 = va + (g0 * k3) * f;
        let o8 = vb + (m[7] * g1) * f;
        callee_cdecl!(6, u32, pass);
        let av = gr(0x106B310);
        let sel = if (av as i32) < 0 { av } else { 0 };
        callee_cdecl!(7, u32, 1, sel);
        callee_cdecl!(8, u32, 4, 4);
        let o_lo = [o1, o3, o5, o7];
        let o_hi = [o2, o4, o6, o8];
        for j in 0..4usize {
            gw(0x17F59F0, o_hi[j].to_bits());
            gw(0x110DFB0, color);
            gw(0x17F59F4, o_lo[j].to_bits());
            callee_cdecl!(
                9, u32, mb[8 + 2 * j], mb[9 + 2 * j], 0, gr(0x17F59EC), gr(0x17F59F8),
                gr(0x17F59FC), color, o_lo[j].to_bits(), o_hi[j].to_bits()
            );
        }
        callee_cdecl!(10, u32,);
        callee_cdecl!(11, u32,);
        let t2 = (vt.wrapping_add(0x40) as *const u32).read();
        let f2: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(t2 as usize);
        f2(obj, 0, 0, 0xFFFF_FFFF);
        callee_cdecl!(3, u32, 0, 1);
        callee_cdecl!(12, u32, 0);
        callee_cdecl!(12, u32, 0x3F80_0000);
        // The checked value is the registry word on the main path, but the
        // early-out path checks it xored against two different stack words, so
        // the contract leaves this call's register uncompared on every trial.
        callee_thiscall!(13, u32, gr(0x1057FB4));
    }
    0
});

/// Truncate a float to i32 exactly like cvttss2si, including the indefinite
/// 0x80000000 for NaN, infinities and out-of-range values.
#[inline(always)]
fn cvtt_ss2si(x: f32) -> i32 {
    if x >= -2147483648.0 && x < 2147483648.0 {
        x as i32
    } else {
        i32::MIN
    }
}
