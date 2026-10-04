// original: 0x00DB7500 UIFontString::vf83
// Recompute a UI font string's layout metrics and sink them into its slots.
//
// Refreshes the base frame, optionally pushes the current scale pair, then
// polls the string's visibility and (when visible) its advance sink. The
// row style, corners, colour and opacity go to the text backend like the
// neighbouring row emitter; the row text resolves either through the shared
// cache or a scratch conversion. A line-height query and the converted
// height feed three layout slots, and a final slot takes the aspect-scaled
// height derived from the display extents. Returns the final slot's answer.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, global, relocated};

export!(thiscall, rw_db7500(this_ptr: u32) -> u32 {
    /// Global UI mode bytes gating the style-word override.
    const UI_MODE: u32 = 0x0116_C250;
    const UI_MODE_ALT: u32 = 0x0116_C253;
    const UI_MODE_DRAW: u8 = 0x6A;
    /// Shared text-cache object used by the non-scratch path.
    const TEXT_CACHE: u32 = 0x0116_BFF0;
    /// Display-extent globals feeding the aspect computation.
    const EXT_A: u32 = 0x0105_C884;
    const EXT_B: u32 = 0x0105_C888;
    const EXT_C: u32 = 0x0105_C880;
    const EXT_D: u32 = 0x0105_C87C;
    /// Scale globals feeding the height computation.
    const SCALE_MUL: u32 = 0x0105_76FC;
    const SCALE_BASE: u32 = 0x0105_76F8;
    const SCALE_DIV: u32 = 0x017A_668C;

    let r32 = |off: u32| unsafe { ((this_ptr + off) as *const u32).read() };
    let r8 = |off: u32| unsafe { ((this_ptr + off) as *const u8).read() };
    let rf = |off: u32| unsafe { ((this_ptr + off) as *const f32).read() };
    let vt = r32(0);
    let vcall0 = |slot: u32| unsafe {
        let addr = ((vt + slot) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(addr as usize);
        f(this_ptr)
    };
    let vcall1 = |slot: u32, arg: u32| unsafe {
        let addr = ((vt + slot) as *const u32).read();
        let f: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(addr as usize);
        f(this_ptr, arg)
    };

    callee_thiscall!(1, u32, this_ptr);
    if r8(0x20B) != 0 {
        callee_cdecl!(2, u32, 0, r32(0x1E4));
    }
    if (vcall0(0xB4) as u8) != 0 {
        callee_thiscall!(1, u32, this_ptr);
        let addr = unsafe { ((vt + 0x8C) as *const u32).read() };
        let adv: extern "thiscall" fn(u32) -> f32 =
            unsafe { core::mem::transmute(addr as usize) };
        let _ = adv(this_ptr);
    }

    let mode = unsafe { global::<u8>(UI_MODE).read() };
    let mode_alt = unsafe { global::<u8>(UI_MODE_ALT).read() };
    let styled = r8(0x208);
    let style: u32 = if (mode == UI_MODE_DRAW || mode_alt != 0) && styled == 0 {
        2
    } else {
        r32(0x1F4)
    };
    callee_cdecl!(5, u32, style);
    callee_cdecl!(6, u32, r32(0x200), r32(0x204));
    callee_cdecl!(7, u32, r32(0x1E8));
    callee_cdecl!(8, u32, r32(0x1EC));

    let text = this_ptr.wrapping_add(0x20E);
    let height: f32 = if styled == 0 {
        let cached = callee_thiscall!(9, u32, relocated(TEXT_CACHE), text);
        callee_cdecl!(10, f32, cached, 1)
    } else {
        let mut scratch = [0u32; 8];
        callee_cdecl!(11, u32, text, scratch.as_mut_ptr() as u32);
        callee_cdecl!(10, f32, scratch.as_ptr() as u32, 1)
    };
    let line: f32 = callee_cdecl!(12, f32,);

    vcall1(0xBC, height.to_bits());
    vcall1(0xC4, line.to_bits());
    let mul = unsafe { global::<f32>(SCALE_MUL).read() };
    vcall1(0xA0, (mul * height).to_bits());

    let pick_a = callee_cdecl!(16, u32,);
    let ext_a = unsafe {
        global::<u32>(if (pick_a as u8) != 0 { EXT_B } else { EXT_A }).read()
    };
    let pick_b = callee_cdecl!(17, u32,);
    let ext_b = unsafe {
        global::<u32>(if (pick_b as u8) != 0 { EXT_D } else { EXT_C }).read()
    };
    let aspect = (ext_a as i32 as f32) / (ext_b as i32 as f32);
    let base = unsafe { global::<f32>(SCALE_BASE).read() };
    let div = unsafe { global::<f32>(SCALE_DIV).read() };
    let scaled = (base * height) / (div / aspect);
    let answer = vcall1(0x94, scaled.to_bits());
    callee_cdecl!(19, u32,);
    answer
});
