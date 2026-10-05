// original: 0x00C61740 arrest_player_attach (proposed)

/// Attach the player-arrest record to a ped and announce it.
///
/// `this` is the task object: the owner ped at `+0x14`, the attached
/// record link at `+0x1c` and the wanted item at `+0x20`. `ped` is the
/// arresting ped.
///
/// Behaviour: when the owner sits in a vehicle and resists, look up the
/// owner's vehicle class and open the arrest record for it (falling back
/// to a default record when none is open); fix up the record link flags,
/// then, only for the exact wanted item and a cooperative owner, shout
/// the arrest speech, link the owner's inventory slot to this ped, take a
/// pooled helper from the global pool head and run the three-step
/// disarm-and-handcuff sequence through one scratch word. Finally mark the
/// owner's vehicle as arrest-flagged (unless its state already says
/// arrested) and consume the wanted item. Returns nothing meaningful.
///
/// Original: 0x00C61740 (thiscall, one stack word: the ped pointer; callee
/// pops it; no return channel). The scratch word handed to the last three
/// callees is an uninitialized stack slot whose value neither side ever
/// reads, so the rewrite passes its own scratch and the contract skips
/// those arguments.
lf_checker_rt::export!(thiscall, rw_00c61740(this: u32, ped: u32) -> u32 {
    unsafe {
        const TASK_OWNER: u32 = 0x14;
        const TASK_LINK: u32 = 0x1c;
        const TASK_ITEM: u32 = 0x20;
        const PED_SLOT: u32 = 0x78;
        const PED_CAB: u32 = 0x21c;
        const PED_VOICE: u32 = 0x570;
        const OWN_FLAG211: u32 = 0x211;
        const OWN_SEAT1: u32 = 0x218;
        const OWN_SEAT2: u32 = 0x219;
        const OWN_INV: u32 = 0x228;
        const OWN_RESIST: u32 = 0x26c;
        const OWN_VEH: u32 = 0xb30;
        const RESIST_BIT: u8 = 0x04;
        const WANT_ITEM: u32 = 0x124;
        const VEH_TABLE: u32 = 0x1295cd8;
        const POOL_HEAD: u32 = 0x167e2a0;
        const SAY_HASH: u32 = 0xecb174;
        const SAY_NAME: u32 = 0xecb180;
        const ATTACH_CB: u32 = 0xc5ecd0;
        const OPEN_BLEND: f32 = 4.0;
        const FULL_WEIGHT: f32 = 1.0;

        const C_FIND: u32 = 1;
        const C_OPEN: u32 = 2;
        const C_ATTACH: u32 = 3;
        const C_HASH: u32 = 4;
        const C_SAY: u32 = 5;
        const C_LINK: u32 = 6;
        const C_POOL: u32 = 7;
        const C_SPAWN: u32 = 8;
        const C_UNEQUIP: u32 = 9;
        const C_APPLY: u32 = 10;
        const C_CLEANUP: u32 = 11;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16sx(a: u32) -> i32 {
            unsafe { (a as *const i16).read_unaligned() as i32 }
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
        unsafe fn g32(file_va: u32) -> u32 {
            unsafe { rd32(lf_checker_rt::relocated(file_va)) }
        }

        let owner = rd32(this.wrapping_add(TASK_OWNER));
        if owner != 0
            && rd8(owner.wrapping_add(OWN_RESIST)) & RESIST_BIT != 0
            && rd32(owner.wrapping_add(OWN_VEH)) != 0
        {
            let b30 = rd32(owner.wrapping_add(OWN_VEH));
            let idx = rd16sx(b30.wrapping_add(0x2e));
            let entry = rd32(
                lf_checker_rt::relocated(VEH_TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
            );
            // The original stages the wanted item in its own argument slot
            // and lets the callee overwrite it; the rewrite uses a local.
            let mut slot = WANT_ITEM;
            let r = lf_checker_rt::callee_cdecl!(
                C_FIND, u32, rd32(entry.wrapping_add(0xc4)),
                &mut slot as *mut u32 as u32, ped, b30, 0, 0, 0, 1
            );
            if r != 0xffff_ffffu32 {
                let t = lf_checker_rt::callee_thiscall!(
                    C_OPEN, u32, rd32(ped.wrapping_add(PED_SLOT)), r, slot,
                    OPEN_BLEND.to_bits(), 0xffff_ffffu32
                );
                wr32(this.wrapping_add(TASK_LINK), t);
            }
        }
        if rd32(this.wrapping_add(TASK_LINK)) == 0 {
            let t = lf_checker_rt::callee_thiscall!(
                C_OPEN, u32, rd32(ped.wrapping_add(PED_SLOT)), 0x14,
                rd32(this.wrapping_add(TASK_ITEM)), OPEN_BLEND.to_bits(), 0xffff_ffffu32
            );
            wr32(this.wrapping_add(TASK_LINK), t);
        }
        let link = rd32(this.wrapping_add(TASK_LINK));
        let w = rd32(link.wrapping_add(4));
        if (w >> 15) & 1 != 0 {
            wr32(link.wrapping_add(4), w & !0x8000);
        }
        let link2 = rd32(this.wrapping_add(TASK_LINK));
        wr32(link2.wrapping_add(4), rd32(link2.wrapping_add(4)) | 0x10);
        lf_checker_rt::callee_thiscall!(
            C_ATTACH, u32, link2, 2, lf_checker_rt::relocated(ATTACH_CB), this
        );
        let o = rd32(this.wrapping_add(TASK_OWNER));
        if rd8(o.wrapping_add(OWN_FLAG211)) == 0
            && rd8(o.wrapping_add(OWN_SEAT1)) == 0
            && rd8(o.wrapping_add(OWN_SEAT2)) != 0
            && rd32(this.wrapping_add(TASK_ITEM)) == WANT_ITEM
        {
            let h = lf_checker_rt::callee_cdecl!(C_HASH, u32, lf_checker_rt::relocated(SAY_HASH), 0);
            lf_checker_rt::callee_thiscall!(
                C_SAY, u32, ped.wrapping_add(PED_VOICE), lf_checker_rt::relocated(SAY_NAME),
                0, 0, 0, 0xffff_ffffu32, rd32(this.wrapping_add(TASK_OWNER)), h,
                FULL_WEIGHT.to_bits(), 0, 0
            );
            let cab = rd32(ped.wrapping_add(PED_CAB));
            if rd32(cab.wrapping_add(0x12c)) == 2 {
                let o2 = rd32(this.wrapping_add(TASK_OWNER));
                if rd8(o2.wrapping_add(OWN_SEAT1)) == 0 && rd8(o2.wrapping_add(OWN_SEAT2)) != 0 {
                    let m = rd32(o2.wrapping_add(OWN_INV));
                    let m70 = if m == 0 { 0 } else { m.wrapping_add(0x70) };
                    if rd32(m70.wrapping_add(0x3a4)) == 0 {
                        let m2 = rd32(o2.wrapping_add(OWN_INV));
                        let slot = (if m2 == 0 { 0 } else { m2.wrapping_add(0x70) })
                            .wrapping_add(0x3a4);
                        wr32(slot, ped);
                        lf_checker_rt::callee_thiscall!(C_LINK, u32, ped, slot);
                    }
                }
            }
            let o3 = rd32(this.wrapping_add(TASK_OWNER));
            if rd8(o3.wrapping_add(OWN_FLAG211)) == 0 {
                let pool = lf_checker_rt::callee_thiscall!(C_POOL, u32, g32(POOL_HEAD));
                let e = if pool == 0 {
                    0
                } else {
                    lf_checker_rt::callee_thiscall!(C_SPAWN, u32, pool, ped)
                };
                // One scratch word stands in for the original's frame slot;
                // no callee result is read back from it.
                let mut tmp = [0u32; 1];
                let tp = tmp.as_mut_ptr() as u32;
                lf_checker_rt::callee_thiscall!(C_UNEQUIP, u32, tp, 2, e, 0);
                let o4 = rd32(this.wrapping_add(TASK_OWNER));
                let grp = rd32(o4.wrapping_add(0x224)).wrapping_add(0x84);
                lf_checker_rt::callee_thiscall!(C_APPLY, u32, grp, tp, 0, 1);
                lf_checker_rt::callee_thiscall!(C_CLEANUP, u32, tp);
            }
            let o5 = rd32(this.wrapping_add(TASK_OWNER));
            if rd8(o5.wrapping_add(OWN_RESIST)) & RESIST_BIT != 0 {
                let b = rd32(o5.wrapping_add(OWN_VEH));
                if b != 0 {
                    let bf = b.wrapping_add(0xf14);
                    wr8(bf, rd8(bf) | 0x80);
                    let st = rd32(b.wrapping_add(0x28));
                    if st & 0x7c00 != 0x0c00 {
                        let mut v = rd32(b.wrapping_add(0x28));
                        // The original tests the same masked value twice;
                        // the second test always agrees with the first.
                        if v & 0x7c00 != 0x0c00 {
                            v = (v & !0x6c00) | 0x1000;
                            wr32(b.wrapping_add(0x28), v);
                        }
                    }
                }
            }
            wr32(this.wrapping_add(TASK_ITEM), 0xffff_ffffu32);
        }
        0
    }
});
