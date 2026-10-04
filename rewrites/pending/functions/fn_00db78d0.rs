// original: 0x00DB78D0 panel_build_textures_and_string
// Build a UI panel's textures, font string and layout bindings.
//
// Allocates and clears four working buffers, constructs the base texture
// and its four sampler bindings, then (unless the first gate byte skips it)
// a highlight texture set with its attach triplet. Unless the second gate
// byte skips them, constructs a slotted texture pair with width-derived
// bindings and a third texture set, each with its own attach triplet. Then
// constructs the panel font string, resolves two HUD colours, binds the
// caller's six-word extent struct, scales by the measured text width with a
// zero-guard branch, and finishes three string copies. The two gate argument
// slots double as scratch: later blocks overwrite them with tag words, so
// the tail observes the overwritten values, tracked here explicitly.
// Returns the final release call's answer.

use lf_checker_rt::{callee_cdecl, callee_thiscall, export, relocated};

export!(thiscall, rw_db78d0_full(
    this_ptr: u32,
    a0: u32, a1: u32, a2: u32, a3: u32, a4: u32,
    a5: u32, a6: u32, a7: u32, a8: u32, a9: u32, a10: u32,
) -> u32 {
    const SMALL_BUF: u32 = 0x80;
    const BIG_BUF: u32 = 0x100;
    const TEX_SIZE: u32 = 0x25C;
    const STR_SIZE: u32 = 0x610;
    const TAG_WORD: u32 = 0xFF17_1717;
    const SLOT_TAG: u32 = 0xFF17_9617;
    const INK_BLACK: u32 = 0xFF00_0000;
    const INK_GREY: u32 = 0xFF79_7979;

    let w32 = |base: u32, off: u32| unsafe { ((base + off) as *const u32).read() };
    let strlen = |mut p: u32| unsafe {
        let start = p;
        while (p as *const u8).read() != 0 {
            p = p.wrapping_add(1);
        }
        p.wrapping_sub(start)
    };
    let v0 = w32(this_ptr, 0);
    let vcall0 = |vt: u32, slot: u32| unsafe {
        let addr = ((vt + slot) as *const u32).read();
        let f: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(addr as usize);
        f(this_ptr)
    };
    let vcall_this = |vt: u32, slot: u32, this: u32, args: &[u32]| unsafe {
        let addr = ((vt + slot) as *const u32).read();
        match args.len() {
            1 => {
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this, args[0])
            }
            2 => {
                let f: extern "thiscall" fn(u32, u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this, args[0], args[1])
            }
            3 => {
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this, args[0], args[1], args[2])
            }
            4 => {
                let f: extern "thiscall" fn(u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this, args[0], args[1], args[2], args[3])
            }
            6 => {
                let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(this, args[0], args[1], args[2], args[3], args[4], args[5])
            }
            7 => {
                let f: extern "thiscall" fn(
                    u32, u32, u32, u32, u32, u32, u32, u32,
                ) -> u32 = core::mem::transmute(addr as usize);
                f(this, args[0], args[1], args[2], args[3], args[4], args[5], args[6])
            }
            _ => core::hint::unreachable_unchecked(),
        }
    };
    let mut scratch = [0u32; 8];
    let mut slot_a10 = a10;

    let p1e4 = callee_cdecl!(1, u32, SMALL_BUF);
    unsafe { ((this_ptr + 0x1E4) as *mut u32).write(p1e4) };
    let p1e0 = callee_cdecl!(1, u32, SMALL_BUF);
    unsafe { ((this_ptr + 0x1E0) as *mut u32).write(p1e0) };
    let p1e8 = callee_cdecl!(1, u32, SMALL_BUF);
    unsafe { ((this_ptr + 0x1E8) as *mut u32).write(p1e8) };
    let p1ec = callee_cdecl!(1, u32, BIG_BUF);
    unsafe { ((this_ptr + 0x1EC) as *mut u32).write(p1ec) };
    callee_cdecl!(2, u32, p1e4, 0, SMALL_BUF);
    callee_cdecl!(2, u32, p1e0, 0, SMALL_BUF);
    callee_cdecl!(2, u32, p1e8, 0, SMALL_BUF);
    callee_cdecl!(2, u32, p1ec, 0, BIG_BUF);
    callee_cdecl!(3, u32, p1ec, relocated(0x00EF_3414), a0, a1);

    // Base texture block (always runs).
    let u = callee_cdecl!(1, u32, TEX_SIZE);
    let va = vcall0(v0, 0x48);
    let vb = vcall0(v0, 0x48);
    let cc = callee_cdecl!(5, u32, relocated(0x00EF_3420), vb);
    let u1 = callee_thiscall!(6, u32, u, cc, va);
    unsafe { ((this_ptr + 0x1FC) as *mut u32).write(u1) };
    let v1 = w32(u1, 0);
    let tag = [TAG_WORD, a1, a2, a3];
    callee_thiscall!(8, u32, u1, 0, 0, tag.as_ptr() as u32, 0xFFFF_FFFF);
    for (kind, b0, b1) in [(4u32, 0u32, 0u32), (0x10, 0, 0), (2, 0, 0), (8, 0, 0)] {
        let d = callee_thiscall!(9, u32, scratch.as_mut_ptr() as u32, b0, b1);
        let w = [w32(d, 0), w32(d, 4), w32(d, 8), w32(d, 12), w32(d, 16), w32(d, 20)];
        vcall_this(v1, 0x114, u1, &[kind, w[0], w[1], w[2], w[3], w[4], w[5]]);
        callee_thiscall!(11, u32, scratch.as_mut_ptr() as u32);
    }

    // Highlight texture set (gated on the first gate byte).
    if (a9 as u8) != 0 {
        let s = callee_cdecl!(1, u32, TEX_SIZE);
        let va = vcall0(v0, 0x48);
        let vb = vcall0(v0, 0x48);
        let cc = callee_cdecl!(5, u32, relocated(0x00EF_3430), vb);
        let h = callee_thiscall!(6, u32, s, cc, va);
        unsafe { ((this_ptr + 0x200) as *mut u32).write(h) };
        let vh = w32(h, 0);
        let gate10 = a10;
        let col = callee_cdecl!(23, u32, &gate10 as *const u32 as u32, 0x3E, 0xFFFF_FFFF);
        callee_thiscall!(32, u32, h, 0, 0, col, 0xFFFF_FFFF);
        for (kind, b0, b1) in [(4u32, 0u32, 0u32), (0x10, 0, 0), (2, 0, 0), (8, 0, 0)] {
            let d = callee_thiscall!(9, u32, scratch.as_mut_ptr() as u32, b0, b1);
            let w = [w32(d, 0), w32(d, 4), w32(d, 8), w32(d, 12), w32(d, 16), w32(d, 20)];
            vcall_this(vh, 0x114, h, &[kind, w[0], w[1], w[2], w[3], w[4], w[5]]);
            callee_thiscall!(11, u32, scratch.as_mut_ptr() as u32);
        }
        let n = vcall0(v0, 0x4C);
        vcall_this(vh, 0x170, h, &[n]);
        vcall_this(vh, 0x28, h, &[1]);
        unsafe { ((this_ptr + 0x218) as *mut u32).write(INK_BLACK) };
    } else if (a10 as u8) != 0 {
        let s = callee_cdecl!(1, u32, TEX_SIZE);
        let va = vcall0(v0, 0x48);
        let vb = vcall0(v0, 0x48);
        let cc = callee_cdecl!(5, u32, relocated(0x00EF_3440), vb);
        let u2 = callee_thiscall!(6, u32, s, cc, va);
        unsafe { ((this_ptr + 0x204) as *mut u32).write(u2) };
        let vu = w32(u2, 0);
        let tx = callee_cdecl!(29, u32, relocated(0x00EF_344C));
        slot_a10 = SLOT_TAG;
        callee_thiscall!(33, u32, u2, tx, relocated(0x00EF_3450),
            &slot_a10 as *const u32 as u32, 0xFFFF_FFFF);
        for (kind, b0, b1) in [(4u32, 0u32, 0x40A0_0000), (2, 0x40A0_0000, 0)] {
            let d = callee_thiscall!(9, u32, scratch.as_mut_ptr() as u32, b0, b1);
            let w = [w32(d, 0), w32(d, 4), w32(d, 8), w32(d, 12), w32(d, 16), w32(d, 20)];
            let r = vcall_this(v1, 0x4C, u1, &[kind, w[0], w[1], w[2], w[3], w[4]]);
            vcall_this(vu, 0x104, u2, &[kind, r, w[5]]);
            callee_thiscall!(11, u32, scratch.as_mut_ptr() as u32);
        }
        vcall_this(vu, 0x80, u2, &[0x4170_0000, 0x4170_0000]);
        let n = vcall0(v0, 0x4C);
        vcall_this(vu, 0x17C, u2, &[n]);
        vcall_this(vu, 0x28, u2, &[1]);
        vcall_this(vu, 0x120, u2, &[0]);

        let s = callee_cdecl!(1, u32, TEX_SIZE);
        let va = vcall0(v0, 0x48);
        let vb = vcall0(v0, 0x48);
        let cc = callee_cdecl!(5, u32, relocated(0x00EF_345C), vb);
        let u3 = callee_thiscall!(6, u32, s, cc, va);
        unsafe { ((this_ptr + 0x200) as *mut u32).write(u3) };
        let v3 = w32(u3, 0);
        slot_a10 = TAG_WORD;
        callee_thiscall!(34, u32, u3, 0, 0,
            &slot_a10 as *const u32 as u32, 0xFFFF_FFFF);
        for (kind, b0, b1) in [(4u32, 0u32, 0u32), (0x10, 0, 0), (2, 0x41B8_0000, 0), (8, 0, 0)] {
            let d = callee_thiscall!(9, u32, scratch.as_mut_ptr() as u32, b0, b1);
            let w = [w32(d, 0), w32(d, 4), w32(d, 8), w32(d, 12), w32(d, 16), w32(d, 20)];
            vcall_this(v3, 0x114, u3, &[kind, w[0], w[1], w[2], w[3], w[4], w[5]]);
            callee_thiscall!(11, u32, scratch.as_mut_ptr() as u32);
        }
        let n = vcall0(v0, 0x4C);
        vcall_this(v3, 0x170, u3, &[n]);
        vcall_this(v3, 0x28, u3, &[1]);
        unsafe { ((this_ptr + 0x218) as *mut u32).write(INK_GREY) };
    } else {
        unsafe { ((this_ptr + 0x218) as *mut u32).write(INK_BLACK) };
    }

    // Font string block (always runs).
    let s = callee_cdecl!(1, u32, STR_SIZE);
    let va2 = vcall0(v0, 0x48);
    let vb2 = vcall0(v0, 0x48);
    let cc2 = callee_cdecl!(5, u32, relocated(0x00EF_346C), vb2);
    let ui = callee_thiscall!(7, u32, s, cc2, va2);
    unsafe { ((this_ptr + 0x208) as *mut u32).write(ui) };
    let v2 = w32(ui, 0);

    let col = callee_cdecl!(23, u32, &slot_a10 as *const u32 as u32, 7, 0);
    vcall_this(v2, 0x1CC, ui, &[0x4190_0000, col, 0, 2]);
    vcall_this(v2, 0x208, ui, &[this_ptr.wrapping_add(0x218)]);
    vcall_this(v2, 0x1E0, ui, &[p1ec, 0]);
    vcall_this(v2, 0x1FC, ui, &[1]);
    vcall_this(v2, 0x80, ui, &[0x41A0_0000, 0x4170_0000]);
    vcall_this(v2, 0x200, ui, &[1]);
    let r = vcall_this(v1, 0x4C, u1, &[4, a3, a4, a5, a6, a7]);
    vcall_this(v2, 0x104, ui, &[4, r, a8]);
    vcall_this(v2, 0x1D4, ui, &[0x2C]);

    let dv: f64 = callee_thiscall!(24, f64, scratch.as_mut_ptr() as u32);
    let f = dv as f32;
    let d2 = if f > 0.0 {
        let dv2: f64 = callee_thiscall!(24, f64, scratch.as_mut_ptr() as u32);
        let f2 = dv2 as f32;
        callee_thiscall!(9, u32, scratch.as_mut_ptr() as u32, f2.to_bits(), 0)
    } else {
        let x = if (a9 as u8) == 0 { 0x41C8_0000 } else { 0 };
        callee_thiscall!(9, u32, scratch.as_mut_ptr() as u32, x, 0)
    };
    let w = [w32(d2, 0), w32(d2, 4), w32(d2, 8), w32(d2, 12), w32(d2, 16), w32(d2, 20)];
    let r2 = vcall_this(v1, 0x4C, u1, &[w[0], w[1], w[2], w[3], w[4], w[5]]);
    vcall_this(v2, 0x10C, ui, &[2, r2]);
    callee_thiscall!(11, u32, scratch.as_mut_ptr() as u32);

    unsafe { ((this_ptr + 0x21C) as *mut u32).write(0) };
    callee_cdecl!(25, u32, p1e0, a0, strlen(a0));
    callee_cdecl!(25, u32, p1e4, a1, strlen(a1));
    callee_cdecl!(25, u32, p1e8, a2, strlen(a2));
    vcall_this(v0, 0x13C, this_ptr, &[1]);
    callee_thiscall!(11, u32, scratch.as_mut_ptr() as u32)
});
