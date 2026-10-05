// original: 0x00C60B20 player_be_arrested_tick (proposed)
/// Advance one tick of the "player is being arrested" task for a ped.
///
/// `this` is the task object: flags at `+0x0c` (bit 2 skips the
/// start-of-task setup), a state word at `+0x10` (zero means the task has
/// nothing left to do), an owner chain pointer at `+0x14`, a running timer
/// at `+0x18` and a latched stick-moved flag at `+0x1c`. `ped` is the ped
/// the task runs on.
///
/// Behaviour: first search the list hanging off the owner chain for a node
/// tagged `0x44c`; missing it (or a missing chain) counts as "not started",
/// as does any later node whose 3-bit code at `+0x08` rises above the head
/// node's code.
/// If the task is already flagged started, or it has not started, report
/// done (return 1) unless the state word says otherwise. Otherwise set the
/// task up on the ped, mark the ped arrested-tasked (`+0x2a0` bit 4),
/// advance the timer by the per-tick increment (zero) and then:
/// - if the ped is flagged finished (`+0xa70 == 1`), poll the wanted-level
///   source; a raised flag there ends the tick as done;
/// - if the ped wants to resist (`+0x26c` bit 2) or the timer has run past
///   its limit, flag the ped finished and fall to the weapon check;
/// - otherwise, while the timer is inside its window, read the analogue
///   sticks: any non-zero deflection (a quiet NaN counts as moved) latches
///   the moved flag unless a global input override is set and the button
///   code disagrees; a failed stick-device check ends the tick as done.
/// Then, unless the ped has no inventory, register the arrest search and
/// shout the matching police speech ("chased in group" early, "chased
/// solo" once the search count passes two). Finally, when the timer is past
/// its end, force the ped to drop its weapon and clear the weapon-held bit.
/// Returns 1 while the task still has work for the next tick, 0 when the
/// ped is handed off (dropped weapon or timer expiry with nothing to do).
///
/// Original: 0x00C60B20 (thiscall, one stack word: the ped pointer;
/// callee pops it). Float comparisons keep the original's unordered
/// (NaN) behaviour: every limit check is "jump if below or equal", i.e.
/// taken for NaN, written as `!(a > b)`.
lf_checker_rt::export!(thiscall, rw_00c60b20(this: u32, ped: u32) -> u8 {
    unsafe {
        const TASK_FLAGS: u32 = 0x0c;
        const TASK_STATE: u32 = 0x10;
        const TASK_OWNER: u32 = 0x14;
        const TASK_TIMER: u32 = 0x18;
        const TASK_MOVED: u32 = 0x1c;
        const FLAG_STARTED: u32 = 0x04;
        const NODE_TAG: u32 = 0x04;
        const NODE_NEXT: u32 = 0x0c;
        const WANTED_TAG: u32 = 0x44c;
        const PED_SLOT: u32 = 0x20;
        const PED_INVENTORY: u32 = 0x228;
        const PED_RESIST: u32 = 0x26c;
        const PED_TASKMARK: u32 = 0x2a0;
        const PED_WEAPONSET: u32 = 0x2b0;
        const PED_VOICE: u32 = 0x570;
        const PED_FINISHED: u32 = 0xa70;
        const RESIST_BIT: u8 = 0x04;
        const TASKMARK_BIT: u32 = 0x10;
        const TICK_ADD: u32 = 0x11735bc;
        const LIMIT_RESIST: u32 = 0xfe8a94; // 3.0
        const LIMIT_WINDOW: u32 = 0xfe88e8; // 1.0
        const INPUT_OVERRIDE: u32 = 0x1160ebc;
        const ARREST_BOARD: u32 = 0x128aa90;
        const SAY_GROUP_HASH: u32 = 0xecb1e8;
        const SAY_GROUP: u32 = 0xecb1f0;
        const SAY_SOLO_HASH: u32 = 0xecb200;
        const SAY_SOLO: u32 = 0xecb208;
        const SETUP_ANIM: f32 = 8.0;
        const SEARCH_RANGE: f32 = 20.0;
        const FULL_WEIGHT: f32 = 1.0;
        const FINISH_BLEND: f32 = -8.0;
        const BUTTON_LIMIT: u8 = 0x7f;
        const WEAPON_HELD_BIT: u32 = 0x0200_0000;

        const C_SETUP: u32 = 1;
        const C_POLL: u32 = 2;
        const C_STICKOBJ: u32 = 3;
        const C_STICKX: u32 = 4;
        const C_TOFLOAT: u32 = 5;
        const C_STICKY: u32 = 6;
        const C_BUTTONS: u32 = 7;
        const C_STICKOK: u32 = 8;
        const C_REG_A: u32 = 9;
        const C_REG_B: u32 = 10;
        const C_SEARCHES: u32 = 11;
        const C_HASH: u32 = 12;
        const C_SAY: u32 = 13;
        const C_FINISH: u32 = 14;
        const C_WEAPONMGR: u32 = 15;
        const C_DROP: u32 = 16;

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
        unsafe fn wr8(a: u32, v: u8) {
            unsafe { (a as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }
        #[inline(always)]
        unsafe fn gf(file_va: u32) -> f32 {
            unsafe { f32::from_bits(g32(file_va)) }
        }

        // Search the owner chain for the wanted-tagged node. Each node
        // carries a 3-bit code at +0x08 (bits 1..=3); the head node's code
        // is kept for the whole walk and every later node bails out as
        // "not started" when its own code is above the head's (unsigned).
        // The head always equals itself, so only later nodes can bail.
        #[inline(always)]
        unsafe fn node_code(node: u32) -> u32 {
            unsafe { (rd32(node.wrapping_add(0x08)) >> 1) & 7 }
        }
        let mut started: u8 = 1;
        let owner = rd32(this.wrapping_add(TASK_OWNER));
        if owner != 0 {
            let chain = rd32(owner.wrapping_add(0x224));
            let mut node = rd32(chain.wrapping_add(0x2e0));
            if node != 0 {
                let head = node_code(node);
                loop {
                    if head < node_code(node) {
                        break;
                    }
                    if rd32(node.wrapping_add(NODE_TAG)) == WANTED_TAG {
                        started = 0;
                        break;
                    }
                    node = rd32(node.wrapping_add(NODE_NEXT));
                    if node == 0 {
                        break;
                    }
                }
            }
        }
        let not_started = started;

        if rd32(this.wrapping_add(TASK_FLAGS)) & FLAG_STARTED == 0 {
            if not_started != 0 {
                return 1;
            }
            let speech = if rd8(ped.wrapping_add(PED_RESIST)) & RESIST_BIT != 0 {
                0x126u32
            } else {
                0x125u32
            };
            lf_checker_rt::callee_thiscall!(C_SETUP, u32, this, ped, 0x16, speech, SETUP_ANIM.to_bits(), 2);
        }
        if rd32(this.wrapping_add(TASK_STATE)) == 0 {
            return 1;
        }
        wr32(
            ped.wrapping_add(PED_TASKMARK),
            rd32(ped.wrapping_add(PED_TASKMARK)) | TASKMARK_BIT,
        );
        let timer = add(rdf(this.wrapping_add(TASK_TIMER)), gf(TICK_ADD));
        wrf(this.wrapping_add(TASK_TIMER), timer);

        // Tail shared by the finished/expiry paths: 1 while the timer is
        // inside its end limit, else drop the ped's weapon and hand off.
        macro_rules! tail {
            () => {{
                let t = rdf(this.wrapping_add(TASK_TIMER));
                if !(t > gf(LIMIT_RESIST)) {
                    return 0;
                }
                let mgr = lf_checker_rt::callee_thiscall!(
                    C_WEAPONMGR, u32, ped.wrapping_add(PED_WEAPONSET)
                );
                if mgr == 0 {
                    return 0;
                }
                if rd32(mgr.wrapping_add(0x18)) == 0 {
                    return 0;
                }
                let w = lf_checker_rt::callee_thiscall!(
                    C_DROP, u32, ped.wrapping_add(PED_WEAPONSET), ped, 1
                );
                wr32(w.wrapping_add(0x210), rd32(w.wrapping_add(0x210)) & !WEAPON_HELD_BIT);
                return 0;
            }};
        }

        // Finish path shared by the poll failure and the no-owner paths.
        macro_rules! finish_done {
            () => {{
                lf_checker_rt::callee_thiscall!(C_FINISH, u32, this, FINISH_BLEND.to_bits());
                return 1;
            }};
        }

        if rd32(ped.wrapping_add(PED_FINISHED)) == 1 {
            tail!();
        }
        let poll = lf_checker_rt::callee_cdecl!(C_POLL, u32,);
        if poll != 0 {
            let second = lf_checker_rt::callee_cdecl!(C_POLL, u32,);
            if rd8(second.wrapping_add(0x5a)) & 0x04 != 0 {
                finish_done!();
            }
            let third = lf_checker_rt::callee_cdecl!(C_POLL, u32,);
            if rd8(third.wrapping_add(0x5a)) & 0x07 != 0 {
                finish_done!();
            }
        }
        if rd8(ped.wrapping_add(PED_RESIST)) & RESIST_BIT != 0 {
            wr32(ped.wrapping_add(PED_FINISHED), 1);
            tail!();
        }
        let now = rdf(this.wrapping_add(TASK_TIMER));
        if !(gf(LIMIT_RESIST) > now) {
            wr32(ped.wrapping_add(PED_FINISHED), 1);
            tail!();
        }
        if !(now > gf(LIMIT_WINDOW)) {
            tail!();
        }

        let stick = lf_checker_rt::callee_thiscall!(C_STICKOBJ, u32, ped);
        let mut moved: u8 = 0;
        if stick == 0 {
            if not_started == 0 {
                tail!();
            }
        } else {
            if rd8(stick.wrapping_add(0x328d)) != 0 {
                let raw_x = lf_checker_rt::callee_thiscall!(C_STICKX, u32, stick);
                let x: f32 = lf_checker_rt::callee_cdecl!(C_TOFLOAT, f32, raw_x);
                moved = 0;
                if x != 0.0 {
                    moved = 1;
                } else {
                    let raw_y = lf_checker_rt::callee_thiscall!(C_STICKY, u32, stick);
                    let y: f32 = lf_checker_rt::callee_cdecl!(C_TOFLOAT, f32, raw_y);
                    if y != 0.0 {
                        moved = 1;
                    }
                }
                if g32(INPUT_OVERRIDE) == 0 {
                    let pad = lf_checker_rt::callee_thiscall!(C_BUTTONS, u32, stick);
                    let code = rd8(pad.wrapping_add(6)) ^ rd8(pad.wrapping_add(4));
                    if code > BUTTON_LIMIT
                        && rd8(this.wrapping_add(TASK_MOVED)) == 0
                        && moved != 0
                    {
                        moved = 1;
                        wr8(this.wrapping_add(TASK_MOVED), 1);
                    } else {
                        wr8(this.wrapping_add(TASK_MOVED), moved);
                        moved = 0;
                    }
                } else {
                    wr8(this.wrapping_add(TASK_MOVED), moved);
                    moved = 0;
                }
            } else {
                wr8(this.wrapping_add(TASK_MOVED), 0);
            }
            let pad = lf_checker_rt::callee_thiscall!(C_BUTTONS, u32, stick);
            let ok = lf_checker_rt::callee_thiscall!(C_STICKOK, u32, pad) as u8;
            if ok == 0 && moved == 0 {
                if not_started == 0 {
                    tail!();
                }
            } else {
                let inv = rd32(ped.wrapping_add(PED_INVENTORY));
                if inv != 0 {
                    let slot_this = inv.wrapping_add(0x70);
                    if slot_this != 0 {
                        let slot = rd32(ped.wrapping_add(PED_SLOT)).wrapping_add(0x30);
                        lf_checker_rt::callee_thiscall!(C_REG_A, u32, slot_this, slot, 2, 0, 1);
                        let slot2 = rd32(ped.wrapping_add(PED_SLOT)).wrapping_add(0x30);
                        lf_checker_rt::callee_thiscall!(
                            C_REG_B, u32,
                            lf_checker_rt::relocated(ARREST_BOARD), slot2, 0x1d, 0
                        );
                    }
                }
            }
        }

        // Arrest search + speech.
        let o = rd32(this.wrapping_add(TASK_OWNER));
        if o == 0 {
            finish_done!();
        }
        if rd8(o.wrapping_add(0x211)) != 0 {
            finish_done!();
        }
        let range_at = rd32(o.wrapping_add(0x20)).wrapping_add(0x30);
        let found = lf_checker_rt::callee_cdecl!(
            C_SEARCHES, u32, o, range_at, SEARCH_RANGE.to_bits(),
            0xffff_ffffu32, 0xffff_ffffu32, 0, 0
        );
        let (hash_name, say_name) = if (found as i32) > 2 {
            (
                lf_checker_rt::relocated(SAY_GROUP_HASH),
                lf_checker_rt::relocated(SAY_GROUP),
            )
        } else {
            (
                lf_checker_rt::relocated(SAY_SOLO_HASH),
                lf_checker_rt::relocated(SAY_SOLO),
            )
        };
        let h = lf_checker_rt::callee_cdecl!(C_HASH, u32, hash_name, 0);
        lf_checker_rt::callee_thiscall!(
            C_SAY, u32, ped.wrapping_add(PED_VOICE), say_name, 1, 1, 0,
            0xffff_ffffu32, ped, h, FULL_WEIGHT.to_bits(), 0, 0
        );
        finish_done!();
    }
});
