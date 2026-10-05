// original: 0x00ce7750 CTaskSimpleNMBalance::vf26 (symbols)

/// Periodic update of the NaturalMotion balancer task: rebuild its message
/// from the parameter row selected by the task's index and send it twice.
///
/// `this` is the task object; the dword at `+0xe0` selects one 0xb0-byte row
/// of the global parameter table. `arg` is the ped object. The function first
/// notifies the pre-pass hook, then opens a message on a 0xcc0-byte stack
/// buffer and fills it with the row's floats and triples (gated by two flag
/// bytes in the row for the middle groups). When the target pointer at
/// `+0x28` is set, a three-float probe is fetched through it (via the inner
/// pointer at `+0x20` when that is set, else from `+0x10` directly) and, for
/// mode 3, resolved through the auxiliary hook; mode 2 resolves through the
/// mode object at `+0xf50` when that is set, and a null mode object falls
/// through to the recompute test like any other mode: the probe is recomputed
/// from the ped's data with the global multiplier, but only when the target
/// equals the ped's own at `+0xab0`. The probe triple is always sent. With no
/// target,
/// a fixed boolean and the task's own triple at `+0x40` go out instead when
/// the byte at `+0x57` is set. The tail sends the message, optionally tags an
/// integer, rebuilds a one-float message (constant 0.3 when the ped's byte at
/// `+0x14f` is set, else the row's last float) and sends that too, restamps
/// the flag word at `+0xc0`, delegates the remaining fields to the sibling
/// sender at 0x00ce7230 with `this+0x60`, and resets the buffer.
///
/// Original: 0x00ce7750 (thiscall, one stack word, no return value).
unsafe fn nm_balance_vf26(this: u32, arg: u32, swap_first_floats: bool) -> u32 {
    unsafe {
        const PRE: u32 = 1;
        const BEGIN: u32 = 2;
        const SET_BOOL: u32 = 3;
        const SET_FLOAT: u32 = 4;
        const SET_VEC3: u32 = 5;
        const AUX: u32 = 6;
        const SET_INT: u32 = 7;
        const SEND: u32 = 8;
        const BEGIN2: u32 = 9;
        const SIBLING: u32 = 10;
        const RESET: u32 = 11;
        const COOKIE: u32 = 12;

        const PRE_ARG0: u32 = 0x00edcd48;
        const IDX_TAB: u32 = 0x01051c30;
        const PARAM_TAB: u32 = 0x0171cb00;
        const ROW: u32 = 0xb0;
        const NAME_BOOL0: u32 = 0x01051cc8;
        const NAME_SEND1: u32 = 0x01051dfc;
        const NAME_SEND2: u32 = 0x01051cf8;
        const NAME_TAILFLOAT: u32 = 0x01051d10;
        const NAME_INT: u32 = 0x01051e14;
        const NAME_JOINBOOL: u32 = 0x01051e18;
        const NAME_JOINVEC: u32 = 0x01051e1c;
        const NAME_JOINFLOAT: u32 = 0x01051e24;
        const NAME_MODE3BOOL: u32 = 0x01051e3c;
        const NAME_MIDBOOL: u32 = 0x01051e5c;
        const NAME_MIDFLOAT: u32 = 0x01051e48;
        const MULT_SLOT: u32 = 0x00fe8ad8;
        const COOKIE_SLOT: u32 = 0x01057fb4;
        const AUX_ID: u32 = 0x4b5;
        const TAIL_CONST: u32 = 0x3e99999a; // 0.3f32
        const SENDER_OFF: u32 = 0x7b4;
        const SIBLING_OFF: u32 = 0x60;

        const HEAD_FLOATS: [(u32, u32); 9] = [
            (0x01051e04, 0x10),
            (0x01051e08, 0x14),
            (0x01051e0c, 0x18),
            (0x01051e10, 0x1c),
            (0x01051e28, 0x3c),
            (0x01051e2c, 0x40),
            (0x01051e30, 0x44),
            (0x01051e34, 0x48),
            (0x01051e38, 0x4c),
        ];
        const HEAD_VEC3: [(u32, u32); 2] =
            [(0x01051e4c, 0x60), (0x01051e50, 0x70)];
        const BODY_FLOATS: [(u32, u32); 10] = [
            (0x01051e60, 0x88),
            (0x01051e64, 0x8c),
            (0x01051e68, 0x90),
            (0x01051e6c, 0x98),
            (0x01051e70, 0x9c),
            (0x01051e74, 0xa0),
            (0x01051e78, 0xa4),
            (0x01051e7c, 0xa8),
            (0x01051e80, 0xac),
            (0x01051e84, 0xb0),
        ];
        const MODE3_FLOATS: [(u32, u32); 4] = [
            (0x01051e40, 0x50),
            (0x01051e44, 0x54),
            (0x01051e54, 0x80),
            (0x01051e58, 0x84),
        ];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        fn fmul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn fadd(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }

        let idx = rd32(this + 0xe0);
        let idxtab = lf_checker_rt::relocated(IDX_TAB);
        lf_checker_rt::callee_cdecl!(
            PRE,
            u32,
            lf_checker_rt::relocated(PRE_ARG0),
            idx,
            rd32(idxtab.wrapping_add(idx.wrapping_mul(4)))
        );
        let mut buf = [0u32; 0x330];
        let msg = buf.as_mut_ptr() as u32;
        lf_checker_rt::callee_thiscall!(BEGIN, u32, msg);
        lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_BOOL0), 1);
        let row = lf_checker_rt::relocated(PARAM_TAB).wrapping_add(idx.wrapping_mul(ROW));
        // Deliberately wrong version (checker `mut_export` only): swap the
        // first two head floats.
        let head0 = if swap_first_floats { HEAD_FLOATS[1] } else { HEAD_FLOATS[0] };
        let head1 = if swap_first_floats { HEAD_FLOATS[0] } else { HEAD_FLOATS[1] };
        lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(head0.0), rd32(row + head0.1));
        lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(head1.0), rd32(row + head1.1));
        for (name, off) in HEAD_FLOATS.iter().skip(2) {
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(*name), rd32(row + off));
        }
        for (name, off) in HEAD_VEC3 {
            lf_checker_rt::callee_thiscall!(
                SET_VEC3,
                u32,
                msg,
                g32(name),
                rd32(row + off),
                rd32(row + off + 4),
                rd32(row + off + 8)
            );
        }
        for (name, off) in BODY_FLOATS {
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(name), rd32(row + off));
        }

        let mut probe = [0u32; 3];
        let target = rd32(this + 0x28);
        let mut resolved = false;
        if target != 0 {
            let inner = rd32(target + 0x20);
            let base = if inner != 0 { inner + 0x30 } else { target + 0x10 };
            probe[0] = rd32(base);
            probe[1] = rd32(base + 4);
            probe[2] = rd32(base + 8);
            let mode = (rd32(target + 0x28) >> 6) & 0xf;
            if mode == 3 {
                lf_checker_rt::callee_thiscall!(AUX, u32, target, probe.as_mut_ptr() as u32, AUX_ID);
                if rd8(row + 0xb9) != 0 {
                    lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_MODE3BOOL), 1);
                    for (name, off) in MODE3_FLOATS {
                        lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(name), rd32(row + off));
                    }
                }
                resolved = true;
            } else if mode == 2 && rd32(target + 0xf50) != 0 {
                let mobj = rd32(target + 0xf50);
                lf_checker_rt::callee_thiscall!(AUX, u32, mobj, probe.as_mut_ptr() as u32, AUX_ID);
                resolved = true;
            } else if target == rd32(arg + 0xab0) {
                let anchor = rd32(arg + 0x20);
                let g = f32::from_bits(g32(MULT_SLOT));
                let t0 = fmul(rdf(anchor + 0x10), g);
                let t1 = fmul(rdf(anchor + 0x18), g);
                let t2 = fmul(rdf(anchor + 0x14), g);
                probe[0] = fadd(rdf(anchor + 0x30), t0).to_bits();
                probe[2] = fadd(rdf(anchor + 0x38), t1).to_bits();
                probe[1] = fadd(rdf(anchor + 0x34), t2).to_bits();
            }
            lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_JOINBOOL), 1);
            lf_checker_rt::callee_thiscall!(
                SET_VEC3,
                u32,
                msg,
                g32(NAME_JOINVEC),
                probe[0],
                probe[1],
                probe[2]
            );
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_JOINFLOAT), rd32(row + 0x20));
            // when unresolved (modes other than 2 and 3) the tail below is
            // reached without the middle group, like the original's join flag.
        } else if rd8(this + 0x57) != 0 {
            lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_JOINBOOL), 1);
            lf_checker_rt::callee_thiscall!(
                SET_VEC3,
                u32,
                msg,
                g32(NAME_JOINVEC),
                rd32(this + 0x40),
                rd32(this + 0x44),
                rd32(this + 0x48)
            );
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_JOINFLOAT), rd32(row + 0x20));
            resolved = true;
        }
        if resolved {
            if rd8(row + 0xba) != 0 {
                lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_MIDBOOL), 1);
                lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_MIDFLOAT), rd32(row + 0x58));
            }
        }
        if rd8(arg + 0x14f) & 1 != 0 {
            lf_checker_rt::callee_thiscall!(SET_INT, u32, msg, g32(NAME_INT), 4);
        }
        lf_checker_rt::callee_thiscall!(SEND, u32, rd32(arg + SENDER_OFF), g32(NAME_SEND1), msg);
        lf_checker_rt::callee_thiscall!(BEGIN2, u32, msg);
        if rd8(arg + 0x14f) & 1 != 0 {
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_TAILFLOAT), TAIL_CONST);
        } else {
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_TAILFLOAT), rd32(row + 0xb4));
        }
        lf_checker_rt::callee_thiscall!(SEND, u32, rd32(arg + SENDER_OFF), g32(NAME_SEND2), msg);
        let stamp = rd32(this + 0xc0);
        ((this + 0xc0) as *mut u32)
            .write_unaligned((stamp & 0xfff1ffff) | 0x00010000);
        lf_checker_rt::callee_thiscall!(SIBLING, u32, this.wrapping_add(SIBLING_OFF), arg);
        lf_checker_rt::callee_thiscall!(RESET, u32, msg);
        lf_checker_rt::callee_thiscall!(COOKIE, u32, g32(COOKIE_SLOT));
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00ce7750(this: u32, arg: u32) -> u32 {
    unsafe { nm_balance_vf26(this, arg, false) }
});
