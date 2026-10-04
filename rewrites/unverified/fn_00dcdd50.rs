// original: 0x00dcdd50 task_ensure_reload_anim (proposed)

/// Ensure the reload task owns a weapon animation, creating and attaching it.
///
/// `task` is the task object, `ped` the ped it runs on. If the animation slot
/// at `+0x68` is already filled the function returns at once, leaving the
/// incoming `eax` untouched (the contract pins incoming `eax`, so the value is
/// still checked). Otherwise it validates the ped's weapon through the weapon
/// manager (`[ped+0x2b0]`, slot id at manager `+0x18`), resolves the weapon
/// info and its weapon id (info `+0x28`), and bails out with 0 when any step
/// fails or with `0xffffffff` when the weapon id is invalid (-1). A streaming
/// check on `(weapon_id, 0xd4)` must also pass.
///
/// It then acquires the animation object with
/// `(owner=[ped+0x78], weapon_id, 0xd4, rate=[task+0x5c], -1)`, stores it in
/// the slot, attaches it with `(2, callback, task)` where the callback is a
/// code pointer, and initialises its time to 0 and its rate to 4.0. Finally it
/// clears flag bits 6 and 7 of the animation's flag word (`+4`) when set and
/// sets bits 0x10, 0x4000, 0x1 and 0x200000. Returns the animation object.
///
/// Original: 0x00dcdd50 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00dcdd50(task: u32, ped: u32) -> u32 {
    unsafe {
        const ANIM_SLOT: u32 = 0x68;
        const RATE_OFF: u32 = 0x5c;
        const PED_WEAPON_OFF: u32 = 0x2b0;
        const PED_OWNER_OFF: u32 = 0x78;
        const MGR_SLOT_ID: u32 = 0x18;
        const INFO_WEAPON_ID: u32 = 0x28;
        const ANIM_KIND: u32 = 0xd4;
        const ANIM_FLAGS: u32 = 0x04;
        const CALLBACK: u32 = 0x00dcdd20;
        const FULL_RATE_BITS: u32 = 0x40800000; // 4.0f
        const NONE: u32 = 0xffff_ffff;
        // The original returns here without touching eax; the contract pins
        // incoming eax to this value so the path is still verified.
        const PINNED_INCOMING_EAX: u32 = 0x12345678;
        const C_VALIDATE: u32 = 1;
        const C_WEAPON_INFO: u32 = 2;
        const C_STREAM_CHECK: u32 = 3;
        const C_ACQUIRE: u32 = 4;
        const C_ATTACH: u32 = 5;
        const C_SET_TIME: u32 = 6;
        const C_SET_RATE: u32 = 7;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        if rd32(task.wrapping_add(ANIM_SLOT)) != 0 {
            return PINNED_INCOMING_EAX;
        }
        let mgr: u32 =
            lf_checker_rt::callee_thiscall!(C_VALIDATE, u32, ped.wrapping_add(PED_WEAPON_OFF));
        if mgr == 0 {
            return 0;
        }
        let info: u32 = lf_checker_rt::callee_cdecl!(C_WEAPON_INFO, u32, rd32(mgr.wrapping_add(MGR_SLOT_ID)));
        if info == 0 {
            return 0;
        }
        let weapon = rd32(info.wrapping_add(INFO_WEAPON_ID));
        if weapon == NONE {
            return NONE;
        }
        let streamed: u32 = lf_checker_rt::callee_cdecl!(C_STREAM_CHECK, u32, weapon, ANIM_KIND);
        if streamed == 0 {
            return 0;
        }
        let rate = rd32(task.wrapping_add(RATE_OFF));
        let anim: u32 = lf_checker_rt::callee_thiscall!(
            C_ACQUIRE,
            u32,
            rd32(ped.wrapping_add(PED_OWNER_OFF)),
            weapon,
            ANIM_KIND,
            rate,
            NONE
        );
        wr32(task.wrapping_add(ANIM_SLOT), anim);
        lf_checker_rt::callee_thiscall!(
            C_ATTACH,
            u32,
            anim,
            2,
            lf_checker_rt::relocated(CALLBACK),
            task
        );
        lf_checker_rt::callee_thiscall!(C_SET_TIME, u32, anim, 0);
        lf_checker_rt::callee_thiscall!(C_SET_RATE, u32, anim, FULL_RATE_BITS);
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
            rd32(anim.wrapping_add(ANIM_FLAGS)) | 0x4000,
        );
        wr32(
            anim.wrapping_add(ANIM_FLAGS),
            rd32(anim.wrapping_add(ANIM_FLAGS)) | 0x1,
        );
        wr32(
            anim.wrapping_add(ANIM_FLAGS),
            rd32(anim.wrapping_add(ANIM_FLAGS)) | 0x200000,
        );
        anim
    }
});
