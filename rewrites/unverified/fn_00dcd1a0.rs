// original: 0x00dcd1a0 task_pick_anim_and_maybe_scale (proposed)

/// Pick the reload animation for the ped's weapon and attach it to the task.
///
/// `task` is the task object, `ped` the ped it runs on. The ped's weapon is
/// validated through the weapon manager (`[ped+0x2b0]`, slot id at manager
/// `+0x18`) and its info resolved; a ducking test on the ped then selects
/// which pair of candidate slots is read from the task (`+0x38/+0x3c` when
/// ducking, `+0x30/+0x34` otherwise). A pair whose first word is not -1 and
/// whose second word is not -1 is used directly as `(id, kind)`.
///
/// Otherwise the info's weapon id (info `+0x28`) and flag word (info `+0x20`)
/// decide: on the ducking path both bits 0x8000 and 0x4000 must be set and
/// the kind becomes 0xc9, else the kind comes from bit 0x4000 (0xc7 when
/// set, -1 when clear). On the plain path the ped's bytes at `+0x218/+0x219`
/// and a streaming check on `(weapon_id, 0xc8)` may select kind 0xc8, else
/// the same bit-0x4000 rule applies.
///
/// The `(id, kind)` pair is offered to a lookup helper with owner
/// `[ped+0x78]`; when it answers null a creation helper builds the animation
/// with the task's rate (`+0x40`) instead. The result is stored at `+0x48`,
/// its time set to 0 and its rate to the task rate, flag bits 6 and 7 of its
/// flag word (`+4`) cleared when set, and it is attached with
/// `(1, callback, task)` after setting bits 0x10, 0x8000 and 0x4000, where
/// the callback is a code pointer.
///
/// Finally, when a global mode word equals 2 and a global gate answers true,
/// a virtual check at slot `+0x128` of the ped runs; when it passes and the
/// object at `[ped+0x228]` holds a value above a global float threshold, the
/// animation's slot `+0x54` is set to a second global float divided by that
/// value. Returns whatever the last step left in `eax`.
///
/// Original: 0x00dcd1a0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00dcd1a0(task: u32, ped: u32) -> u32 {
    unsafe {
        const CAND_A: u32 = 0x38;
        const CAND_B: u32 = 0x3c;
        const CAND_C: u32 = 0x30;
        const CAND_D: u32 = 0x34;
        const RATE_OFF: u32 = 0x40;
        const ANIM_SLOT: u32 = 0x48;
        const ANIM_FLAGS: u32 = 0x04;
        const ANIM_SCALE: u32 = 0x54;
        const PED_WEAPON_OFF: u32 = 0x2b0;
        const PED_OWNER_OFF: u32 = 0x78;
        const MGR_SLOT_ID: u32 = 0x18;
        const INFO_FLAGS: u32 = 0x20;
        const INFO_WEAPON_ID: u32 = 0x28;
        const PED_STATE_A: u32 = 0x218;
        const PED_STATE_B: u32 = 0x219;
        const PED_EXTRA: u32 = 0x228;
        const EXTRA_VALUE: u32 = 0x5ac;
        const PED_VCHECK_SLOT: u32 = 0x128;
        const KIND_DUCK: u32 = 0xc9;
        const KIND_STREAM: u32 = 0xc8;
        const KIND_FLAG: u32 = 0xc7;
        const MODE_GLOBAL: u32 = 0x011d6fd4;
        const CMP_GLOBAL: u32 = 0x00fe8874;
        const SRC_GLOBAL: u32 = 0x00fe88e8;
        const CALLBACK: u32 = 0x00dcd170;
        const NONE: u32 = 0xffff_ffff;
        const C_VALIDATE: u32 = 1;
        const C_WEAPON_INFO: u32 = 2;
        const C_DUCKING: u32 = 3;
        const C_STREAM_CHECK: u32 = 4;
        const C_LOOKUP: u32 = 5;
        const C_CREATE: u32 = 6;
        const C_SET_TIME: u32 = 7;
        const C_SET_RATE: u32 = 8;
        const C_ATTACH: u32 = 9;
        const C_GATE: u32 = 10;
        const C_VCHECK: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        fn div(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) / core::hint::black_box(b)
        }

        let mgr: u32 =
            lf_checker_rt::callee_thiscall!(C_VALIDATE, u32, ped.wrapping_add(PED_WEAPON_OFF));
        let info: u32 =
            lf_checker_rt::callee_cdecl!(C_WEAPON_INFO, u32, rd32(mgr.wrapping_add(MGR_SLOT_ID)));
        let ducking: u32 = lf_checker_rt::callee_thiscall!(C_DUCKING, u32, ped);
        let info_id = rd32(info.wrapping_add(INFO_WEAPON_ID));
        let info_flags = rd32(info.wrapping_add(INFO_FLAGS));
        let (id, kind): (u32, u32);
        if (ducking & 0xff) != 0 {
            let a = rd32(task.wrapping_add(CAND_A));
            let mut done = false;
            let mut pair = (0u32, 0u32);
            if a != NONE {
                let b = rd32(task.wrapping_add(CAND_B));
                if b != NONE {
                    pair = (a, b);
                    done = true;
                }
            }
            if done {
                (id, kind) = pair;
            } else if info_id == NONE || info_flags & 0x8000 == 0 || info_flags & 0x4000 == 0 {
                id = info_id;
                kind = if info_flags & 0x4000 != 0 { KIND_FLAG } else { NONE };
            } else {
                id = info_id;
                kind = KIND_DUCK;
            }
        } else {
            let a = rd32(task.wrapping_add(CAND_C));
            let mut done = false;
            let mut pair = (0u32, 0u32);
            if a != NONE {
                let b = rd32(task.wrapping_add(CAND_D));
                if b != NONE {
                    pair = (a, b);
                    done = true;
                }
            }
            if done {
                (id, kind) = pair;
            } else {
                let use_stream = rd8(ped.wrapping_add(PED_STATE_A)) == 0
                    && rd8(ped.wrapping_add(PED_STATE_B)) != 0
                    && lf_checker_rt::callee_cdecl!(C_STREAM_CHECK, u32, info_id, KIND_STREAM) != 0;
                if use_stream {
                    id = info_id;
                    kind = KIND_STREAM;
                } else {
                    id = info_id;
                    kind = if info_flags & 0x4000 != 0 { KIND_FLAG } else { NONE };
                }
            }
        }
        let owner = rd32(ped.wrapping_add(PED_OWNER_OFF));
        let mut anim: u32 = lf_checker_rt::callee_thiscall!(C_LOOKUP, u32, owner, id, kind);
        wr32(task.wrapping_add(ANIM_SLOT), anim);
        if anim == 0 {
            let rate = rd32(task.wrapping_add(RATE_OFF));
            anim = lf_checker_rt::callee_thiscall!(
                C_CREATE, u32, owner, id, kind, rate, NONE
            );
            wr32(task.wrapping_add(ANIM_SLOT), anim);
        }
        lf_checker_rt::callee_thiscall!(C_SET_TIME, u32, anim, 0);
        lf_checker_rt::callee_thiscall!(
            C_SET_RATE,
            u32,
            anim,
            rd32(task.wrapping_add(RATE_OFF))
        );
        let flags = rd32(anim.wrapping_add(ANIM_FLAGS));
        if flags & 0x40 != 0 {
            wr32(anim.wrapping_add(ANIM_FLAGS), flags & !0x40);
        }
        let flags = rd32(anim.wrapping_add(ANIM_FLAGS));
        if flags & 0x80 != 0 {
            wr32(anim.wrapping_add(ANIM_FLAGS), flags & !0x80);
        }
        wr32(
            anim.wrapping_add(ANIM_FLAGS),
            rd32(anim.wrapping_add(ANIM_FLAGS)) | 0x10,
        );
        wr32(
            anim.wrapping_add(ANIM_FLAGS),
            rd32(anim.wrapping_add(ANIM_FLAGS)) | 0x8000,
        );
        wr32(
            anim.wrapping_add(ANIM_FLAGS),
            rd32(anim.wrapping_add(ANIM_FLAGS)) | 0x4000,
        );
        let mut ret: u32 =
            lf_checker_rt::callee_thiscall!(C_ATTACH, u32, anim, 1, lf_checker_rt::relocated(CALLBACK), task);
        if unsafe { lf_checker_rt::global::<u32>(MODE_GLOBAL).read_unaligned() } == 2 {
            ret = lf_checker_rt::callee_cdecl!(C_GATE, u32,);
            if (ret & 0xff) != 0 {
                let check: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                    rd32(rd32(ped).wrapping_add(PED_VCHECK_SLOT)) as usize,
                );
                ret = check(ped);
                if (ret & 0xff) != 0 {
                    let extra = rd32(ped.wrapping_add(PED_EXTRA));
                    ret = extra;
                    if extra != 0 {
                        let w = f32::from_bits(rd32(extra.wrapping_add(EXTRA_VALUE)));
                        let cmp = f32::from_bits(unsafe {
                            lf_checker_rt::global::<u32>(CMP_GLOBAL).read_unaligned()
                        });
                        if w > cmp {
                            let src = f32::from_bits(unsafe {
                                lf_checker_rt::global::<u32>(SRC_GLOBAL).read_unaligned()
                            });
                            wr32(anim.wrapping_add(ANIM_SCALE), div(src, w).to_bits());
                            ret = anim;
                        }
                    }
                }
            }
        }
        ret
    }
});
