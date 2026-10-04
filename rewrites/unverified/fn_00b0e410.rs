// original: 0x00B0E410 net_sync_pick_handler (proposed)

/// Pick the handler pointer for a network sync object and store it.
///
/// `this` points to the object: a 16-bit id at `+0x40` (read signed), a mode
/// byte at `+0x44`, and the handler slot at `+0x14`. Every handler value is
/// the global base word plus either a fixed offset or a global offset word.
///
/// Behaviour: the probe callee is always called first with the sign-extended
/// id; its answer is kept as the fallthrough return value. The mode byte
/// minus two selects one of eight cases (2, 4, 5, 8, 9/10, 11/12, 15, 23);
/// any other mode stores nothing and returns the probe answer:
/// - mode 2: store base+0xAFC8.
/// - mode 4: store base+0x4E20, then require predicate A, else predicate B,
///   to be true (low byte of their answers); when both are false the first
///   store stands and the answer of B is returned, otherwise base+0x9C40.
/// - mode 5: store base+0x1D4C0.
/// - mode 8: when either predicate is true store base plus the global offset
///   word, otherwise base+0x7530.
/// - modes 9/10: set the mode byte to 9, store base+0x5DC.
/// - modes 11/12: set the mode byte to 11, store base+0x5DC.
/// - mode 15: re-read the id; when it equals the first global id, predicate
///   A picks between base+0xEA60 (true) and base+0x493E0 (false), otherwise
///   predicate A picks between base+0xAFC8 (true) and base+0x57E40 (false).
/// - mode 23: branch on the probe answer: zero stores nothing; 0x2F stores
///   base plus the second global offset; below 0x2F (signed) resolves
///   through the lookup callee (slot at +0x7C of its answer, or +0x78 when
///   that reads -1) added to the base; above 0x2F re-reads the id and stores
///   base plus the second global offset when it matches either middle global
///   id, base plus the third global offset when it matches the last global
///   id, else nothing.
///
/// The returned value is whatever sits in EAX on the taken path (the stored
/// pointer, the probe answer, or a predicate answer), matching the original.
///
/// Original: 0x00B0E410 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00B0E410(this: u32) -> u32 {
    unsafe {
        const OBJ_ID: u32 = 0x40;
        const OBJ_MODE: u32 = 0x44;
        const OBJ_HANDLER: u32 = 0x14;
        const G_BASE: u32 = 0x0117_35B4;
        const G_OFF_HIGH: u32 = 0x0104_00C0;
        const G_OFF_MID: u32 = 0x0104_00C4;
        const G_OFF_FLAG: u32 = 0x0104_00C8;
        const G_ID_FIRST: u32 = 0x0161_5640;
        const G_ID_MID_A: u32 = 0x012F_A3E0;
        const G_ID_MID_B: u32 = 0x012F_9E94;
        const G_ID_LAST: u32 = 0x0163_2BF0;
        const CALLEE_PROBE: u32 = 1;
        const CALLEE_PRED_A: u32 = 2;
        const CALLEE_PRED_B: u32 = 3;
        const CALLEE_LOOKUP: u32 = 4;
        const LOOKUP_PRIMARY: u32 = 0x7c;
        const LOOKUP_FALLBACK: u32 = 0x78;
        const PROBE_SPECIAL: u32 = 0x2f;

        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rd16sx(a: u32) -> u32 {
            unsafe { (a as *const i16).read_unaligned() as i32 as u32 }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn glob(va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(va)) }
        }

        let id = rd16sx(this + OBJ_ID);
        let probe: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PROBE, u32, id);
        let base = glob(G_BASE);
        let mode = rd8(this + OBJ_MODE);
        match mode {
            2 => {
                let out = base.wrapping_add(0xafc8);
                wr32(this + OBJ_HANDLER, out);
                out
            }
            4 => {
                let first = base.wrapping_add(0x4e20);
                wr32(this + OBJ_HANDLER, first);
                let a: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_A, u32,);
                if a & 0xff == 0 {
                    let b: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_B, u32,);
                    if b & 0xff == 0 {
                        return b;
                    }
                }
                let out = base.wrapping_add(0x9c40);
                wr32(this + OBJ_HANDLER, out);
                out
            }
            5 => {
                let out = base.wrapping_add(0x1d4c0);
                wr32(this + OBJ_HANDLER, out);
                out
            }
            8 => {
                let a: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_A, u32,);
                if a & 0xff != 0 {
                    let out = base.wrapping_add(glob(G_OFF_FLAG));
                    wr32(this + OBJ_HANDLER, out);
                    return out;
                }
                let b: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_B, u32,);
                let out = if b & 0xff != 0 {
                    base.wrapping_add(glob(G_OFF_FLAG))
                } else {
                    base.wrapping_add(0x7530)
                };
                wr32(this + OBJ_HANDLER, out);
                out
            }
            9 | 10 => {
                wr8(this + OBJ_MODE, 9);
                let out = base.wrapping_add(0x5dc);
                wr32(this + OBJ_HANDLER, out);
                out
            }
            11 | 12 => {
                wr8(this + OBJ_MODE, 11);
                let out = base.wrapping_add(0x5dc);
                wr32(this + OBJ_HANDLER, out);
                out
            }
            15 => {
                let id2 = rd16sx(this + OBJ_ID);
                let a: u32 = lf_checker_rt::callee_cdecl!(CALLEE_PRED_A, u32,);
                let out = if id2 == glob(G_ID_FIRST) {
                    if a & 0xff != 0 {
                        base.wrapping_add(0xea60)
                    } else {
                        base.wrapping_add(0x493e0)
                    }
                } else if a & 0xff != 0 {
                    base.wrapping_add(0xafc8)
                } else {
                    base.wrapping_add(0x57e40)
                };
                wr32(this + OBJ_HANDLER, out);
                out
            }
            23 => {
                if probe == 0 {
                    return probe;
                }
                if probe == PROBE_SPECIAL {
                    let out = base.wrapping_add(glob(G_OFF_HIGH));
                    wr32(this + OBJ_HANDLER, out);
                    return out;
                }
                if (probe as i32) < PROBE_SPECIAL as i32 {
                    let p: u32 = lf_checker_rt::callee_cdecl!(CALLEE_LOOKUP, u32, probe);
                    let mut slot = rd32(p + LOOKUP_PRIMARY);
                    if slot == 0xffff_ffff {
                        slot = rd32(p + LOOKUP_FALLBACK);
                    }
                    let out = slot.wrapping_add(base);
                    wr32(this + OBJ_HANDLER, out);
                    return out;
                }
                let id3 = rd16sx(this + OBJ_ID);
                if id3 == glob(G_ID_MID_A) || id3 == glob(G_ID_MID_B) {
                    let out = base.wrapping_add(glob(G_OFF_HIGH));
                    wr32(this + OBJ_HANDLER, out);
                    out
                } else if id3 == glob(G_ID_LAST) {
                    let out = base.wrapping_add(glob(G_OFF_MID));
                    wr32(this + OBJ_HANDLER, out);
                    out
                } else {
                    probe
                }
            }
            _ => probe,
        }
    }
});
