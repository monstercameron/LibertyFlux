// original: 0x00ce7230 nm_balance_send_full (proposed)

/// Build one NaturalMotion balancer message from this task's fields and send it.
///
/// `this` is the task object. It holds six triples of floats (at `+0x00`,
/// `+0x10`, `+0x20`, `+0x30`, `+0x40`, `+0x50`), a flag word at `+0x60`, a
/// timestamp slot at `+0x64`, an optional info pointer at `+0x68`, an extra
/// integer at `+0x6c` and one more float at `+0x70`. `arg` is the ped object;
/// only the sender handle at `+0x7b4` is used.
///
/// Behaviour: when all of the first and third triples are exactly zero the
/// function does nothing. Otherwise it opens a message on a 0xcc0-byte stack
/// buffer, and unless the flag word's bit 8 is already set sends one boolean
/// and stamps that bit and the stamp slot, sends each triple whose lanes are
/// not all zero as a vec3, sends the
/// info word and extra integer when the info pointer is set, sends two
/// booleans derived from the first and third triples, sends flag-gated
/// booleans (bit 4 wins over bit 5), sends seven floats (six from globals,
/// one from `+0x70`), sends the message through the ped's sender and resets
/// the buffer. A NaN lane counts as non-zero everywhere.
///
/// Original: 0x00ce7230 (thiscall, one stack word, no return value).
///
/// `invert_tribool1` selects the deliberately wrong version (used only as the
/// checker's `mut_export`): it flips the second triple-derived boolean.
unsafe fn nm_balance_send_full(this: u32, arg: u32, invert_tribool1: bool) -> u32 {
    unsafe {
        const BEGIN: u32 = 1;
        const SET_BOOL: u32 = 2;
        const SET_VEC3: u32 = 3;
        const SET_INT: u32 = 4;
        const SET_FLOAT: u32 = 5;
        const SEND: u32 = 6;
        const RESET: u32 = 7;
        const COOKIE: u32 = 8;

        const NAME_BOOL0: u32 = 0x01051cc8;
        const NAME_VEC0: u32 = 0x01051f54;
        const NAME_VEC1: u32 = 0x01051f64;
        const NAME_VEC2: u32 = 0x01051f58;
        const NAME_VEC3: u32 = 0x01051f68;
        const NAME_VEC4: u32 = 0x01051f5c;
        const NAME_VEC5: u32 = 0x01051f60;
        const NAME_INT0: u32 = 0x01051f84;
        const NAME_INT1: u32 = 0x01051fa8;
        const NAME_TRIBOOL0: u32 = 0x01051f7c;
        const NAME_TRIBOOL1: u32 = 0x01051f80;
        const NAME_FLAGBIT3: u32 = 0x01051f74;
        const NAME_FLAGBIT4: u32 = 0x01051f6c;
        const NAME_FLAGBIT5: u32 = 0x01051f70;
        const NAME_FLOAT0: u32 = 0x01051f4c;
        const NAME_FLOAT1: u32 = 0x01051f50;
        const NAME_FLOAT2: u32 = 0x01051f8c;
        const NAME_FLOAT3: u32 = 0x01051f90;
        const NAME_FLOAT4: u32 = 0x01051f94;
        const NAME_BOOLFIX: u32 = 0x01051f88;
        const NAME_FLOAT5: u32 = 0x01051f98;
        const NAME_FLOAT6: u32 = 0x01051f9c;
        const NAME_FLOAT7: u32 = 0x01051fa0;
        const NAME_SEND: u32 = 0x01051f44;
        const STAMP_SLOT: u32 = 0x011735b4;
        const TUNING0: u32 = 0x0171d008;
        const TUNING1: u32 = 0x0171d00c;
        const TUNING2: u32 = 0x0171d010;
        const TUNING3: u32 = 0x0171d014;
        const TUNING4: u32 = 0x0171d018;
        const TUNING5: u32 = 0x0171d01c;
        const COOKIE_SLOT: u32 = 0x01057fb4;

        const FLAG_STAMPED: u32 = 0x100;
        const INFO_INNER: u32 = 0x38;
        const INFO_WORD: u32 = 8;
        const SENDER_OFF: u32 = 0x7b4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        /// Any lane set, with NaN counting as set (matches `ucomiss` + `jp`).
        #[inline(always)]
        unsafe fn any_set(base: u32) -> bool {
            unsafe { rdf(base) != 0.0 || rdf(base + 4) != 0.0 || rdf(base + 8) != 0.0 }
        }

        // Entry gate: nothing to send when the first and third triples are
        // all zero. The cookie check at the end still runs.
        let live = any_set(this) || any_set(this + 0x20);
        if live {
            let mut buf = [0u32; 0x330];
            let msg = buf.as_mut_ptr() as u32;
            lf_checker_rt::callee_thiscall!(BEGIN, u32, msg);
            let flags = rd32(this + 0x60);
            if flags & FLAG_STAMPED == 0 {
                lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_BOOL0), 1);
                ((this + 0x60) as *mut u32).write_unaligned(flags | FLAG_STAMPED);
                ((this + 0x64) as *mut u32).write_unaligned(g32(STAMP_SLOT));
            }
            let triples: [(u32, u32); 6] = [
                (NAME_VEC0, 0x00),
                (NAME_VEC1, 0x10),
                (NAME_VEC2, 0x20),
                (NAME_VEC3, 0x30),
                (NAME_VEC4, 0x40),
                (NAME_VEC5, 0x50),
            ];
            for (name, off) in triples {
                if any_set(this + off) {
                    lf_checker_rt::callee_thiscall!(
                        SET_VEC3,
                        u32,
                        msg,
                        g32(name),
                        rd32(this + off),
                        rd32(this + off + 4),
                        rd32(this + off + 8)
                    );
                }
            }
            let info = rd32(this + 0x68);
            if info != 0 {
                let word = ((rd32(info + INFO_INNER) + INFO_WORD) as *const u16).read_unaligned();
                lf_checker_rt::callee_thiscall!(SET_INT, u32, msg, g32(NAME_INT0), word as u32);
                lf_checker_rt::callee_thiscall!(SET_INT, u32, msg, g32(NAME_INT1), rd32(this + 0x6c));
            }
            lf_checker_rt::callee_thiscall!(
                SET_BOOL,
                u32,
                msg,
                g32(NAME_TRIBOOL0),
                u32::from(any_set(this))
            );
            let tri1 = any_set(this + 0x20) ^ invert_tribool1;
            lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_TRIBOOL1), u32::from(tri1));
            if flags & 0x8 != 0 {
                lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_FLAGBIT3), 1);
            }
            if flags & 0x10 != 0 {
                lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_FLAGBIT4), 1);
            } else if flags & 0x20 != 0 {
                lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_FLAGBIT5), 1);
            }
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT0), g32(TUNING0));
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT1), rd32(this + 0x70));
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT2), g32(TUNING1));
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT3), g32(TUNING2));
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT4), g32(TUNING3));
            lf_checker_rt::callee_thiscall!(SET_BOOL, u32, msg, g32(NAME_BOOLFIX), 1);
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT5), g32(TUNING4));
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT6), g32(TUNING4));
            lf_checker_rt::callee_thiscall!(SET_FLOAT, u32, msg, g32(NAME_FLOAT7), g32(TUNING5));
            lf_checker_rt::callee_thiscall!(SEND, u32, rd32(arg + SENDER_OFF), g32(NAME_SEND), msg);
            lf_checker_rt::callee_thiscall!(RESET, u32, msg);
        }
        lf_checker_rt::callee_thiscall!(COOKIE, u32, g32(COOKIE_SLOT));
        0
    }
}

lf_checker_rt::export!(thiscall, rw_00ce7230(this: u32, arg: u32) -> u32 {
    unsafe { nm_balance_send_full(this, arg, false) }
});
