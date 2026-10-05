// original: 0x00D01AB0 CTaskComplexArrestedAIPedAndDriveAway::vf20

/// Advance one tick of the arrested-ped-and-drive-away task.
///
/// `this` is the task, `ped` the ped being driven (one stack word). Every
/// tick adds the frame-time global to the timer at `+0x20`, then dispatches:
///
/// * No subject (`+0x14` is null): unless flag bit 0 of `+0x0C` is set, ask
///   the task's own virtual slot 5 (`[[this]+0x14]`, called with
///   `(ped, 1, 0)`) whether to proceed; a false answer returns the stashed
///   value at `+0x08`, a true one sets flag bit 1 and returns 0.
/// * With a subject: sub-state `+0x1C` 0 plays one speech line once the
///   timer passes 2 seconds and moves to sub-state 1; sub-state 1 moves to
///   2 past 5 seconds and plays one of two lines picked by a random draw
///   scaled by 1/32768 against one half.
/// * State `+0x18` 1 with an alert subject (byte `+0x26C` bit 2 of the
///   subject, word `+0xB30` non-zero) tries to start a get-in-vehicle task:
///   a predicate callee, a release callee, a pool allocation off the
///   manager global, and the constructor callee whose result is returned
///   after its word `+0x2C` is set to 2. A failed allocation takes the same
///   path with a null pointer, which faults on the `+0x2C` store.
/// * Otherwise, when the timer is past 2 seconds and no matching sub-task
///   is found, a cooldown at `+0x24` counts down by the frame time; while
///   it stays positive the stashed `+0x08` value is returned, else the
///   predicate is consulted once more (false keeps `+0x08`, true returns 0).
///   An expired timer returns `+0x08` after resetting the cooldown to 2.0.
/// * State 3 runs the release callee and returns `+0x08`; any other state
///   returns `+0x08` directly.
///
/// All threshold comparisons are strict (`comiss` + `jbe`/`jb`), so NaN
/// timers take the not-past branch. Float operation order is the original's.
///
/// Original: 0x00D01AB0 (thiscall, one stack word, u32 return).
lf_checker_rt::export!(thiscall, rw_00d01ab0(this: u32, ped: u32) -> u32 {
    unsafe {
        const STASH: u32 = 0x08;
        const FLAGS: u32 = 0x0c;
        const SUBJECT: u32 = 0x14;
        const STATE: u32 = 0x18;
        const SUBSTATE: u32 = 0x1c;
        const TIMER: u32 = 0x20;
        const COOLDOWN: u32 = 0x24;
        const VSLOT: u32 = 0x14;
        const SUB_FIND_BASE: u32 = 0x224;
        const SUB_FIND_ADJ: u32 = 0x2e0;
        const SUB_FIND_ID: u32 = 0x2e7;
        const SUB_ALERT: u32 = 0x26c;
        const SUB_ALERT_BIT: u8 = 4;
        const SUB_SLOT: u32 = 0xb30;
        const SAY_ANCHOR: u32 = 0x570;
        const SAY_ONE: u32 = 0x3f80_0000;
        const LINE_ASK: u32 = 0xedf65c;
        const LINE_FOUND_WEAPON: u32 = 0xedf668;
        const LINE_FOUND_CARDS: u32 = 0xedf67c;
        const RAND_SCALE: f32 = f32::from_bits(0x3800_0100);
        const HALF: f32 = 0.5;
        const TWO_SECS: f32 = 2.0;
        const FIVE_SECS: f32 = 5.0;
        const NEW_TASK_TAG: u32 = 0x0120_0000;
        const NEW_TASK_KIND: u32 = 0x1b;
        const NEW_TASK_PRIO: u32 = 0xffff_fffb; // -5
        const NEW_TASK_MODE: u32 = 2;
        const G_FRAME_TIME: u32 = 0x0117_35bc;
        const G_TASK_MGR: u32 = 0x0167_e2a0;
        const C_RAND: u32 = 1;
        const C_SAY: u32 = 2;
        const C_TRY_START: u32 = 3;
        const C_RELEASE: u32 = 4;
        const C_POOL_ALLOC: u32 = 5;
        const C_NEW_TASK: u32 = 6;
        const C_FIND_SUBTASK: u32 = 7;

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
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn say_line(ped: u32, line_file_va: u32) {
            unsafe {
                // The original pushes the line pointer as an immediate that
                // carries a relocation entry, so it arrives relocated.
                let line = lf_checker_rt::relocated(line_file_va);
                let _: u32 = lf_checker_rt::callee_thiscall!(
                    C_SAY, u32, ped.wrapping_add(SAY_ANCHOR), line, 0, 0, 0,
                    0xffff_ffff, 0, 0, SAY_ONE, 0, 0
                );
            }
        }

        let g = f32::from_bits(
            (lf_checker_rt::global::<u32>(G_FRAME_TIME) as *const u32).read_unaligned(),
        );
        let timer = add(g, rdf(this.wrapping_add(TIMER)));
        wrf(this.wrapping_add(TIMER), timer);

        if rd32(this.wrapping_add(SUBJECT)) == 0 {
            if rd8(this.wrapping_add(FLAGS)) & 1 == 0 {
                let slot: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    core::mem::transmute(
                        rd32(rd32(this).wrapping_add(VSLOT)) as usize
                    );
                if (slot(this, ped, 1, 0) & 0xff) == 0 {
                    return rd32(this.wrapping_add(STASH));
                }
                wr32(
                    this.wrapping_add(FLAGS),
                    rd32(this.wrapping_add(FLAGS)) | 2,
                );
            }
            return 0;
        }

        match rd32(this.wrapping_add(SUBSTATE)) {
            0 => {
                if timer > TWO_SECS {
                    say_line(ped, LINE_ASK);
                    wr32(this.wrapping_add(SUBSTATE), 1);
                }
            }
            1 => {
                if timer > FIVE_SECS {
                    wr32(this.wrapping_add(SUBSTATE), 2);
                    let draw: u32 = lf_checker_rt::callee_cdecl!(C_RAND, u32,);
                    let scaled = mul((draw as i32) as f32, RAND_SCALE);
                    say_line(
                        ped,
                        if HALF > scaled {
                            LINE_FOUND_WEAPON
                        } else {
                            LINE_FOUND_CARDS
                        },
                    );
                }
            }
            _ => {}
        }

        let state = rd32(this.wrapping_add(STATE));
        if state == 1 {
            let subject = rd32(this.wrapping_add(SUBJECT));
            let alert = rd8(subject.wrapping_add(SUB_ALERT)) & SUB_ALERT_BIT != 0;
            let slot = rd32(subject.wrapping_add(SUB_SLOT));
            if alert && slot != 0 {
                let ok: u32 =
                    lf_checker_rt::callee_thiscall!(C_TRY_START, u32, this, ped, 1, 0);
                if (ok & 0xff) != 0 {
                    let _: u32 = lf_checker_rt::callee_thiscall!(C_RELEASE, u32, ped, 0);
                    let mgr = (lf_checker_rt::global::<u32>(G_TASK_MGR) as *const u32)
                        .read_unaligned();
                    let pool: u32 = lf_checker_rt::callee_thiscall!(C_POOL_ALLOC, u32, mgr);
                    if pool == 0 {
                        wr32(this.wrapping_add(STATE), 3);
                        // The original stores through the null pointer here
                        // and faults; both sides must fault identically.
                        wr32(pool.wrapping_add(0x2c), NEW_TASK_MODE);
                        return 0;
                    }
                    let task: u32 = lf_checker_rt::callee_thiscall!(
                        C_NEW_TASK, u32, pool, slot, NEW_TASK_PRIO, NEW_TASK_KIND,
                        NEW_TASK_TAG, 0
                    );
                    wr32(this.wrapping_add(STATE), 3);
                    wr32(task.wrapping_add(0x2c), NEW_TASK_MODE);
                    return task;
                }
            }
            // Cooldown path.
            if !(rdf(this.wrapping_add(TIMER)) > TWO_SECS) {
                let stash = rd32(this.wrapping_add(STASH));
                wrf(this.wrapping_add(COOLDOWN), TWO_SECS);
                return stash;
            }
            let find_base = rd32(subject.wrapping_add(SUB_FIND_BASE));
            let found: u32 = lf_checker_rt::callee_thiscall!(
                C_FIND_SUBTASK, u32, find_base.wrapping_add(SUB_FIND_ADJ),
                SUB_FIND_ID, 0
            );
            if (found & 0xff) != 0 {
                let stash = rd32(this.wrapping_add(STASH));
                wrf(this.wrapping_add(COOLDOWN), TWO_SECS);
                return stash;
            }
            let left = sub(rdf(this.wrapping_add(COOLDOWN)), g);
            wrf(this.wrapping_add(COOLDOWN), left);
            if !(0.0 < left) {
                let ok2: u32 =
                    lf_checker_rt::callee_thiscall!(C_TRY_START, u32, this, ped, 1, 0);
                if (ok2 & 0xff) != 0 {
                    return 0;
                }
            }
            return rd32(this.wrapping_add(STASH));
        }
        if state == 3 {
            let _: u32 = lf_checker_rt::callee_thiscall!(C_RELEASE, u32, ped, 0);
        }
        rd32(this.wrapping_add(STASH))
    }
});
