// original: 0x00c9f6b0 CSimpleIkManager::vf1

/// Step the simple IK manager: gate on the window state, then solve a limb.
///
/// Entry (thiscall, no stack words): calls the `IsIconic` import on the
/// window handle kept at file `0x17accd8`; a zero answer falls back to the
/// game bytes at file `0x105b48f`/`0x17ed8d1` to pick the active flag, then
/// two more game bytes (file `0x1173590`/`0x1173591`) are ORed in and, unless
/// the result is zero, the byte at file `0x1160c39` must be nonzero to
/// continue. The object at `this+0x10` must have bit `0x10` at `+0xf4`, and
/// the filter call (callee 2) must answer zero; anything else returns with
/// the last value in eax.
///
/// Tail (entered by the original's jump, inlined here): reads the mode field
/// at `obj+0x28` (bits 6-9 must be 3 or 6) and the watch flag (`obj+0x78`
/// non-null with bit 1 at its target); otherwise returns the watch value.
/// Loads a row from the game table at file `0x1295cd8` by the signed index
/// at `obj+0x2e`, calls the row handler (slot `+0x38`, probe 0), resolves
/// the data row through two holder calls (slot `+0xa0`, null falls back to
/// `obj+0x100`) plus the data call (slot `+0xe0`), and gates on
/// `answer*0xe0 + [row+4]` being nonzero. The solver call (callee 6) yields
/// the accumulator triple; unless the constant at file `0xfe8d78` exceeds
/// its z, a clamped blend (cap file `0xfe88e8`, game floats at file
/// `0x1050bcc`/`0x1050c60`/`0x1050c64`/`0x1050c68`, row field `+0x28`)
/// accumulates into it and the triple is stored back. Returns the solver
/// answer, or the row word on the gate exit.
///
/// Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00c9f6b0(this: u32) -> u32 {
    unsafe {
        const WND_GLOBAL: u32 = 0x17accd8;
        const FALLBACK_A: u32 = 0x105b48f;
        const FALLBACK_B: u32 = 0x17ed8d1;
        const OR_A: u32 = 0x1173590;
        const OR_B: u32 = 0x1173591;
        const GATE_BYTE: u32 = 0x1160c39;
        const OBJ_OFF: u32 = 0x10;
        const FLAG_OFF: u32 = 0xf4;
        const FLAG_BIT: u8 = 0x10;
        const MODE_OFF: u32 = 0x28;
        const WATCH_OFF: u32 = 0x78;
        const IDX_OFF: u32 = 0x2e;
        const ROW_TABLE: u32 = 0x1295cd8;
        const VT_SLOT: u32 = 0x38;
        const VT_HOLDER: u32 = 0xa0;
        const VT_DATA: u32 = 0xe0;
        const FALLBACK_ROW: u32 = 0x100;
        const ROW_WORD: u32 = 4;
        const STRIDE: u32 = 0xe0;
        const ROW_FIELD: u32 = 0x28;
        const TRIPLE_OFF: u32 = 0x30;
        const CONST_C0: u32 = 0xfe8d78;
        const CONST_C1: u32 = 0xfe88e8;
        const GF_G0: u32 = 0x1050bcc;
        const GF_G1: u32 = 0x1050c60;
        const GF_G2: u32 = 0x1050c64;
        const GF_G3: u32 = 0x1050c68;
        type Hook1 = extern "thiscall" fn(u32, u32) -> u32;
        type Hook0 = extern "thiscall" fn(u32) -> u32;
        let hwnd = lf_checker_rt::global::<u32>(WND_GLOBAL).read_unaligned();
        let imp: u32 = lf_checker_rt::callee_stdcall!(1, u32, hwnd);
        let mut eax = imp;
        let mut al: u8;
        if imp == 0 {
            if lf_checker_rt::global::<u8>(FALLBACK_A).read() == 0 {
                al = 0;
            } else if lf_checker_rt::global::<u8>(FALLBACK_B).read() != 0 {
                al = 1;
            } else {
                al = 0;
            }
        } else {
            al = 1;
        }
        eax = (eax & 0xffffff00) | (al as u32);
        al |= lf_checker_rt::global::<u8>(OR_A).read();
        al |= lf_checker_rt::global::<u8>(OR_B).read();
        eax = (eax & 0xffffff00) | (al as u32);
        let mut gated = al == 0;
        if !gated && lf_checker_rt::global::<u8>(GATE_BYTE).read() != 0 {
            gated = true;
        }
        if !gated {
            return eax;
        }
        let obj = ((this + OBJ_OFF) as *const u32).read_unaligned();
        if ((obj + FLAG_OFF) as *const u8).read() & FLAG_BIT == 0 {
            return eax;
        }
        let c: u32 = lf_checker_rt::callee_cdecl!(2, u32, obj);
        eax = c;
        if c as u8 != 0 {
            return eax;
        }
        let m = ((((obj + MODE_OFF) as *const u32).read_unaligned() >> 6) & 0xf) as u8;
        let cl = m == 3 || m == 6;
        let t = ((obj + WATCH_OFF) as *const u32).read_unaligned();
        eax = t;
        let al2: u8;
        if t != 0 && ((t as *const u8).read() & 2) != 0 {
            al2 = 1;
        } else {
            al2 = 0;
        }
        eax = (eax & 0xffffff00) | (al2 as u32);
        if !cl || al2 == 0 {
            return eax;
        }
        let table = lf_checker_rt::relocated(ROW_TABLE);
        let idx = ((obj + IDX_OFF) as *const i16).read_unaligned() as i32;
        let entry =
            (table.wrapping_add(idx.wrapping_mul(4) as u32) as *const u32).read_unaligned();
        let vt = (entry as *const u32).read_unaligned();
        let hook: Hook1 =
            core::mem::transmute(((vt + VT_SLOT) as *const u32).read_unaligned() as usize);
        let p = hook(entry, 0u32);
        let vt2 = (obj as *const u32).read_unaligned();
        let get_a: Hook0 =
            core::mem::transmute(((vt2 + VT_HOLDER) as *const u32).read_unaligned() as usize);
        let a1 = get_a(obj);
        let row: u32;
        if a1 == 0 {
            row = ((obj + FALLBACK_ROW) as *const u32).read_unaligned();
        } else {
            let a2 = get_a(obj);
            let vt3 = (a2 as *const u32).read_unaligned();
            let get_b: Hook0 =
                core::mem::transmute(((vt3 + VT_DATA) as *const u32).read_unaligned() as usize);
            row = get_b(a2);
        }
        let row1 = ((row + ROW_WORD) as *const u32).read_unaligned();
        eax = row1;
        let x = (row1 as *const u32).read_unaligned();
        let esi_v = p.wrapping_mul(STRIDE).wrapping_add(x);
        if esi_v == 0 {
            return eax;
        }
        let r: u32 = lf_checker_rt::callee_thiscall!(6, u32, obj, p);
        eax = r;
        let bb = core::hint::black_box;
        let t30 = ((r + TRIPLE_OFF) as *const f32).read_unaligned();
        let t34 = ((r + TRIPLE_OFF + 4) as *const f32).read_unaligned();
        let t38 = ((r + TRIPLE_OFF + 8) as *const f32).read_unaligned();
        let c0 = lf_checker_rt::global::<f32>(CONST_C0).read_unaligned();
        let c1 = lf_checker_rt::global::<f32>(CONST_C1).read_unaligned();
        let mut x3 = 0.0f32;
        let mut x4 = 0.0f32;
        let mut x6 = ((esi_v + ROW_FIELD) as *const f32).read_unaligned();
        if bb(c0) > bb(t38) {
            let g0 = lf_checker_rt::global::<f32>(GF_G0).read_unaligned();
            let mut x0 = bb(c0) - bb(t38);
            x0 = bb(x0) * bb(g0);
            if x0 < 0.0 {
                x0 = 0.0;
            }
            if x0 > bb(c1) {
                x0 = c1;
            }
            let x2 = bb(c1) - bb(x0);
            let mut x5 = bb(c1) - bb(x2);
            x4 = bb(x2) * 0.0f32;
            x6 = bb(x6) * bb(x2);
            x3 = x4;
            x0 = x5;
            let g1 = lf_checker_rt::global::<f32>(GF_G1).read_unaligned();
            let g2 = lf_checker_rt::global::<f32>(GF_G2).read_unaligned();
            let g3 = lf_checker_rt::global::<f32>(GF_G3).read_unaligned();
            x0 = bb(x0) * bb(g1);
            let mut x1 = bb(x5) * bb(g2);
            x5 = bb(x5) * bb(g3);
            x3 = bb(x3) + bb(x0);
            x4 = bb(x4) + bb(x1);
            x6 = bb(x6) + bb(x5);
        }
        x3 = bb(x3) + bb(t30);
        x4 = bb(x4) + bb(t34);
        x6 = bb(x6) + bb(t38);
        ((r + TRIPLE_OFF) as *mut f32).write_unaligned(x3);
        ((r + TRIPLE_OFF + 4) as *mut f32).write_unaligned(x4);
        ((r + TRIPLE_OFF + 8) as *mut f32).write_unaligned(x6);
        eax
    }
});
