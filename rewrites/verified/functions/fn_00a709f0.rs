// original: 0x00A709F0 CTaskComplexPlayerSettingsTask::vf20

/// Refresh a player settings task: either mark the owner's settings applied
/// or rebuild the task's selected variant.
///
/// `this` is the task (`+0x08` owner object, `+0x14` done flag byte, `+0x18`
/// and `+0x1C` variant words, `+0x1C` also the selected id). `arg` is a
/// game object or null. A set done flag returns the owner at once.
///
/// With a null argument the owner is settled instead: the done flag is set,
/// and unless the owner's flag byte at `+0x0C` already has bit 0, the
/// owner's virtual slot at `+0x14` is invoked (thiscall: owner, 0, 1, 0) and
/// a false answer returns the owner; otherwise bit 1 of the owner flags is
/// set and null is returned.
///
/// With an argument, a checker predicate (thiscall on `arg + 0x2B0`, 1) and
/// the selected id decide between two paths. The direct path needs the
/// predicate true and id `0xDD`, then a gate predicate (thiscall on the
/// owner: arg, 1, 0); a variant probe (thiscall on the task) then selects
/// id `0xDA` (true) or `0xD7` (false). The alternate path needs the
/// predicate false at the second try and an id other than `0xDD`/`-1`,
/// passes the same gate, and resets the id to `0xDD`. Either path then
/// allocates from the pool whose pointer lives in a data global and builds
/// the replacement through a maker call (thiscall on the block: words
/// `+0x18`/`+0x1C`, `8.0f`, `0`, `1.0f`, `0`), whose result is returned; a
/// null block returns null. Any failed gate returns the owner.
///
/// Original: 0x00A709F0 (thiscall, one stack word). Returns a pointer or
/// null in `eax`; callee answers are observed through `al` only.
lf_checker_rt::export!(thiscall, rw_00A709F0(this: u32, arg: u32) -> u32 {
    unsafe {
        const OWNER: u32 = 0x08;
        const DONE: u32 = 0x14;
        const WORD18: u32 = 0x18;
        const SEL: u32 = 0x1c;
        const OWNER_FLAGS: u32 = 0x0c;
        const VT_SLOT: u32 = 0x14;
        const PROBE_OFF: u32 = 0x2b0;
        const SEL_WANT: u32 = 0xdd;
        const SEL_ALT: u32 = 0xd7;
        const SEL_PROBE: u32 = 0xda;
        const POOL_GLOBAL: u32 = 0x167e2a0;
        const C_VCALL: u32 = 1;
        const C_CHECK1: u32 = 2;
        const C_GATE1: u32 = 3;
        const C_PROBE: u32 = 4;
        const C_ALLOC: u32 = 5;
        const C_MAKE: u32 = 6;
        const C_CHECK2: u32 = 7;
        const C_GATE2: u32 = 8;

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
        fn low_set(ans: u32) -> bool {
            ans & 0xff != 0
        }

        if rd8(this + DONE) != 0 {
            return rd32(this + OWNER);
        }
        if arg == 0 {
            let owner = rd32(this + OWNER);
            wr8(this + DONE, 1);
            if rd8(owner + OWNER_FLAGS) & 1 == 0 {
                let slot = rd32(rd32(owner) + VT_SLOT);
                let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
                    unsafe { core::mem::transmute(slot as usize) };
                if !low_set(f(owner, 0, 1, 0)) {
                    return rd32(this + OWNER);
                }
                wr32(owner + OWNER_FLAGS, rd32(owner + OWNER_FLAGS) | 2);
            }
            return 0;
        }
        let direct = low_set(lf_checker_rt::callee_thiscall!(
            C_CHECK1,
            u32,
            arg.wrapping_add(PROBE_OFF),
            1
        )) && rd32(this + SEL) == SEL_WANT;
        if direct {
            let owner = rd32(this + OWNER);
            if !low_set(lf_checker_rt::callee_thiscall!(C_GATE1, u32, owner, arg, 1, 0))
            {
                return rd32(this + OWNER);
            }
            let picked = low_set(lf_checker_rt::callee_thiscall!(C_PROBE, u32, this));
            wr32(this + SEL, if picked { SEL_PROBE } else { SEL_ALT });
        } else {
            if low_set(lf_checker_rt::callee_thiscall!(
                C_CHECK2,
                u32,
                arg.wrapping_add(PROBE_OFF),
                1
            )) {
                return rd32(this + OWNER);
            }
            let sel = rd32(this + SEL);
            if sel == SEL_WANT || sel == 0xffff_ffff {
                return rd32(this + OWNER);
            }
            let owner = rd32(this + OWNER);
            if !low_set(lf_checker_rt::callee_thiscall!(C_GATE2, u32, owner, arg, 1, 0))
            {
                return rd32(this + OWNER);
            }
            wr32(this + SEL, SEL_WANT);
        }
        let pool = rd32(lf_checker_rt::relocated(POOL_GLOBAL));
        let block = lf_checker_rt::callee_thiscall!(C_ALLOC, u32, pool);
        if block == 0 {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(
            C_MAKE,
            u32,
            block,
            rd32(this + WORD18),
            rd32(this + SEL),
            0x4100_0000,
            0,
            0x3f80_0000,
            0
        )
    }
});
