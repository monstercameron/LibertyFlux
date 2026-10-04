// original: 0x00E43D50 emit_indexed_quad_guarded
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

/// Publish one indexed quad with a guarded index (original 0x00E43D50).
///
/// Twin of `rw_00e43870`: same level/clamp/publish shape with lookup ids
/// 0x7f/0x80 and a different flag global, plus two differences. When the
/// index is 1 the owner's secondary flag must also be set, and a color-table
/// call resolves the palette entry whose low 24 bits join the packed color.
/// The return value is incidental (callers ignore it).
export!(thiscall, rw_00e43d50(this: u32, index: i32, a1: f32, a2: f32, a3: f32, a4: f32) -> u32 {
    const ENABLE_OFF: usize = 0x44c;
    const SECONDARY_OFF: usize = 0x395;
    const SLOT_OFF: usize = 0x4c;
    const LOOKUP_LEVEL: u32 = 0x7f;
    const LOOKUP_ENTRY: u32 = 0x80;
    const FLAG: u32 = 0x01161668;
    const WHITE_RGB: u32 = 0x00ff_ffff;
    unsafe {
        if (this as *const u8).add(ENABLE_OFF).read() == 0
            || (index == 1 && (this as *const u8).add(SECONDARY_OFF).read() == 0)
        {
            return 0;
        }
        let mut scratch = [0u32; 1];
        let out = scratch.as_mut_ptr() as u32;
        let p1 = callee_cdecl!(1, u32, out, LOOKUP_LEVEL);
        let mut level = cvttss2si((p1 as *const f32).read()) as u8;
        if global::<u8>(FLAG).read() != 0 {
            level = callee_thiscall!(2, u32, relocated(FLAG)) as u8;
        }
        let p2 = callee_cdecl!(1, u32, out, LOOKUP_LEVEL);
        let fi = level as f32;
        let f2 = (p2 as *const f32).read();
        let v = if 0.0f32 > fi {
            0.0
        } else if fi <= f2 || f2.is_nan() {
            fi
        } else {
            f2
        };
        let p3 = callee_cdecl!(1, u32, out, LOOKUP_ENTRY);
        let entry = cvttss2si((p3 as *const f32).read()) as u8;
        let pal = callee_cdecl!(3, u32, out, entry as u32);
        let n = cvttss2si(v);
        let color = ((n as u8 as u32) << 24) | ((pal as *const u32).read() & WHITE_RGB);
        let _ = (a2, a4);
        let sum2 = a3 + a1;
        let slot = (this as *const u32).add(SLOT_OFF / 4).read();
        callee_thiscall!(
            4,
            u32,
            slot.wrapping_add((index as u32).wrapping_mul(4))
        );
        let mut c0 = a1;
        let mut c1 = a1;
        let mut c2 = sum2;
        let mut c3 = sum2;
        let mut c4 = color;
        callee_cdecl!(
            5, u32,
            &mut c0 as *mut f32 as u32,
            &mut c1 as *mut f32 as u32,
            &mut c2 as *mut f32 as u32,
            &mut c3 as *mut f32 as u32,
            &mut c4 as *mut u32 as u32,
        );
        callee_cdecl!(6, u32,)
    }
});
