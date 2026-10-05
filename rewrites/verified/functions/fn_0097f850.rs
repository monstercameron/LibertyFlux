// original: 0x0097F850 ped_task_switch_event (proposed)

/// Fire a switch-selected ped audio event carrying a float argument.
///
/// `this` is the task object, `sel` picks the event class and `level`
/// is a float passed by bits. After the shared audio-ready guards, a
/// selector above 0x13, a dead-end case, or any failed lookup ends the
/// call after stamping the clock word into `+0x158` (guard exits stamp
/// nothing). The class comes from a 20-entry table: two classes resolve
/// an event word through global pointer tables indexed by `+0x7C`/`+0x80`
/// (a null entry or a null word at the target's `+0xA` ends the call),
/// one runs a predicate on `[this+0x120]` and then a lazily hashed name,
/// and one exits at once.
///
/// On the main path the scratch buffer is built (word 0 = a two-stage
/// float chain: callee 4 transforms `level`, callee 5 transforms that;
/// word 5 = `[[this+0x120]+0x20]` biased by 0x30; word 8 = `[this+8]`),
/// callee 6 resolves a handle from the buffer, callee 7 derives a
/// parameter, and callee 8 (this = task, event word first) arbitrates: a
/// zero low byte releases the handle through callee 10, otherwise
/// callee 9 posts the event with a (0, -1, 0x4A) descriptor. The clock is
/// stamped into `+0x158` on the way out.
///
/// Original: 0x0097F850 (thiscall, two stack arguments, no return value).
lf_checker_rt::export!(thiscall, rw_0097F850(this: u32, sel: u32, level: u32) -> u32 {
    unsafe {
        const G_QUIT: u32 = 0x011F7060;
        const G_SESS_A: u32 = 0x012088B4;
        const G_SESS_B: u32 = 0x00F1C040;
        const G_MODE: u32 = 0x01037720;
        const G_CLOCK: u32 = 0x011735B4;
        const G_HASH_FLAG: u32 = 0x01231748;
        const G_HASH: u32 = 0x01231744;
        const SKIP_MODE: u32 = 0x12;
        const SEL_MAX: u32 = 0x13;
        const TABLE_A: u32 = 0x01231360;
        const TABLE_B: u32 = 0x01231320;
        const NAME_STR: u32 = 0x00E8CCD4;
        const FLOAT_MGR: u32 = 0x0123144C;
        const OFF_CLASS_A: u32 = 0x7C;
        const OFF_CLASS_B: u32 = 0x80;
        const OFF_PED: u32 = 0x120;
        const OFF_SUB: u32 = 0x08;
        const OFF_STAMP: u32 = 0x158;
        const PED_INNER: u32 = 0x20;
        const INNER_BIAS: u32 = 0x30;
        const WORD_OFF: u32 = 0x0A;
        // Class per selector 0..=0x13: 0 = table A, 1 = table B,
        // 2 = predicate + hash, 3 = exit.
        const CLASS: [u8; 20] = [
            0, 0, 0, 3, 0, 0, 3, 1, 1, 1, 1, 3, 2, 1, 1, 1, 3, 1, 1, 1,
        ];

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn gget(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read() }
        }
        #[inline(always)]
        unsafe fn stamp(this: u32) {
            unsafe {
                (this.wrapping_add(OFF_STAMP) as *mut u32).write_unaligned(gget(G_CLOCK));
            }
        }

        if gget(G_QUIT) == 1 {
            return 0;
        }
        if gget(G_SESS_A) != gget(G_SESS_B) {
            return 0;
        }
        if gget(G_MODE) == SKIP_MODE {
            return 0;
        }
        if sel > SEL_MAX {
            stamp(this);
            return 0;
        }
        let word = match CLASS[(sel & 0xFF) as usize] {
            0 => {
                let e = rd32(
                    lf_checker_rt::relocated(TABLE_A)
                        .wrapping_add(rd32(this.wrapping_add(OFF_CLASS_A)).wrapping_mul(4)),
                );
                if e == 0 {
                    stamp(this);
                    return 0;
                }
                rd32(e.wrapping_add(WORD_OFF))
            }
            1 => {
                let e = rd32(
                    lf_checker_rt::relocated(TABLE_B)
                        .wrapping_add(rd32(this.wrapping_add(OFF_CLASS_B)).wrapping_mul(4)),
                );
                if e == 0 {
                    stamp(this);
                    return 0;
                }
                rd32(e.wrapping_add(WORD_OFF))
            }
            2 => {
                let ok: u32 = lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    rd32(this.wrapping_add(OFF_PED))
                );
                if (ok & 0xFF) == 0 {
                    stamp(this);
                    return 0;
                }
                let flag = gget(G_HASH_FLAG);
                if flag & 1 == 0 {
                    lf_checker_rt::global::<u32>(G_HASH_FLAG).write(flag | 1);
                    let h: u32 = lf_checker_rt::callee_cdecl!(
                        2,
                        u32,
                        lf_checker_rt::relocated(NAME_STR),
                        0
                    );
                    lf_checker_rt::global::<u32>(G_HASH).write(h);
                    h
                } else {
                    gget(G_HASH)
                }
            }
            _ => {
                stamp(this);
                return 0;
            }
        };
        if word == 0 {
            stamp(this);
            return 0;
        }
        let mut buf = [0u32; 16];
        let buf_ptr = buf.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_thiscall!(3, u32, buf_ptr);
        let ped = rd32(this.wrapping_add(OFF_PED));
        let f1: f32 =
            lf_checker_rt::callee_thiscall!(4, f32, lf_checker_rt::relocated(FLOAT_MGR), level);
        let f2: f32 = lf_checker_rt::callee_cdecl!(5, f32, f1.to_bits());
        buf[0] = f2.to_bits();
        buf[5] = rd32(ped.wrapping_add(PED_INNER)).wrapping_add(INNER_BIAS);
        buf[8] = rd32(this.wrapping_add(OFF_SUB));
        let handle: u32 = lf_checker_rt::callee_thiscall!(6, u32, buf_ptr);
        let param: u32 = lf_checker_rt::callee_cdecl!(7, u32, handle);
        let arb: u32 =
            lf_checker_rt::callee_thiscall!(8, u32, this, word, buf_ptr, handle, param, 0);
        if (arb & 0xFF) == 0 {
            let _: u32 = lf_checker_rt::callee_cdecl!(10, u32, handle);
            stamp(this);
            return 0;
        }
        let mut desc = [0u32, 0xFFFF_FFFF, 0x4A];
        let desc_ptr = desc.as_mut_ptr() as u32;
        let _: u32 = lf_checker_rt::callee_cdecl!(
            9, u32, word, 0, 0, 1, buf_ptr, desc_ptr, ped, handle
        );
        stamp(this);
        0
    }
});
