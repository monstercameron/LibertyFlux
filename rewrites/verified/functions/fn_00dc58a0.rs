// original: 0x00dc58a0 TaskShoot_AttachWeaponAnim (proposed)

/// Attach the shoot task's weapon animation: validate the ped's weapon,
/// look up its info, check the anim slot is free, and bind a new anim clip
/// with its flags set.
///
/// `this` points to the task (flag word at `+0x6C`, rate at `+0x60`) and
/// `ped` to the ped. If `+0x6C` is already set there is nothing to do.
/// Otherwise callee 0 validates the ped's weapon block (`ped+0x2B0`); a
/// null answer ends the call. Callee 1 maps the weapon id
/// (`manager+0x18`) to its info; a null answer, or a slot (`info+0x28`)
/// of -1, ends the call. Callee 2 tests the `(slot, 0xD4)` pair; a zero
/// answer ends the call. Callee 3 (this `ped+0x78`) binds the clip and its
/// answer is kept at `+0x6C`; callee 4 attaches the update callback
/// (code address `UPDATE_CB`), callee 5 sets the clip time to zero and
/// callee 6 sets it to the global rate `RATE`.
///
/// Finally bits 6 and 7 of the clip's flag word (`clip+0x4`) are cleared
/// and bits 0, 4, 14 and 21 are set. The function returns no value (its
/// first early path leaks its entry eax), so the contract compares no
/// return channel.
///
/// Original: 0x00dc58a0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00dc58a0(this: u32, ped: u32) -> u32 {
    unsafe {
        const ANIM_KIND: u32 = 0xD4;
        const UPDATE_CB: u32 = 0x00DC57E0;
        const RATE: u32 = 0x0105783C;
        const VALIDATE_WEAPON: u32 = 0;
        const WEAPON_INFO: u32 = 1;
        const SLOT_FREE: u32 = 2;
        const BIND_CLIP: u32 = 3;
        const ATTACH_CB: u32 = 4;
        const SET_TIME_ZERO: u32 = 5;
        const SET_TIME_RATE: u32 = 6;
        const NO_SLOT: u32 = 0xffff_ffff;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        // The original returns its entry eax on this path (a leaked
        // register, not a value), so the contract compares no return channel.
        if rd32(this + 0x6c) != 0 {
            return 0;
        }
        let manager: u32 = lf_checker_rt::callee_thiscall!(VALIDATE_WEAPON, u32, ped + 0x2b0);
        if manager == 0 {
            return 0;
        }
        let info: u32 = lf_checker_rt::callee_cdecl!(WEAPON_INFO, u32, rd32(manager + 0x18));
        if info == 0 {
            return 0;
        }
        let slot = rd32(info + 0x28);
        if slot == NO_SLOT {
            return NO_SLOT;
        }
        let free: u32 = lf_checker_rt::callee_cdecl!(SLOT_FREE, u32, slot, ANIM_KIND);
        if free == 0 {
            return 0;
        }
        let clip: u32 = lf_checker_rt::callee_thiscall!(BIND_CLIP, u32, rd32(ped + 0x78), slot, ANIM_KIND, rd32(this + 0x60), NO_SLOT);
        wr32(this + 0x6c, clip);
        lf_checker_rt::callee_thiscall!(ATTACH_CB, u32, clip, 2, lf_checker_rt::relocated(UPDATE_CB), this);
        lf_checker_rt::callee_thiscall!(SET_TIME_ZERO, u32, clip, 0);
        lf_checker_rt::callee_thiscall!(SET_TIME_RATE, u32, clip, rd32(lf_checker_rt::relocated(RATE)));
        let flags = rd32(clip + 4);
        let cleared = flags & 0xffff_ff3f;
        wr32(clip + 4, cleared | 0x0020_4011);
        clip
    }
});
