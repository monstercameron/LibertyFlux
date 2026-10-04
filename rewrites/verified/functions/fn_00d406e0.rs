// original: 0x00d406e0 CTaskSimpleJumpInAir::vf5

/// Decide whether a jump-in-air task accepts the requested transition.
///
/// `this` is the task, `owner` (arg0) the ped, `kind` (arg1) the
/// requested transition, `next` (arg2) a task object. Kind 2 accepts at
/// once; any kind other than 1 rejects at once. On kind 1, bit 2 of the
/// owner word at `OWNER_FLAG` (+0x29c) accepts at once, and a null `next`
/// rejects. Otherwise `next`'s type slot (virtual slot +4) is queried:
/// 0x20 queries the subtype slot (+8), whose 0x65 accepts; any other
/// first answer re-queries the type slot, whose 0x5d accepts. Accepting
/// runs the settle callee (thiscall on `this` with -32.0f as bits),
/// clears bit 13 of the owner word at `OWNER_CLR` (+0x26c) and returns 1;
/// rejecting returns 0. Only al carries the result.
///
/// Original: 0x00d406e0 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00d406e0(this: u32, owner: u32, kind: u32, next: u32) -> u32 {
    unsafe {
        const OWNER_FLAG: u32 = 0x29c;
        const OWNER_CLR: u32 = 0x26c;
        const CLR_MASK: u32 = 0xffff_dfff;
        const TYPE_SLOT: u32 = 4;
        const SUB_SLOT: u32 = 8;
        const SETTLE: u32 = 3;
        const SETTLE_ARG: u32 = 0xc100_0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn query(next: u32, slot: u32) -> u32 {
            unsafe {
                let vtable = rd32(next);
                let addr = rd32(vtable + slot);
                let f: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(addr as usize);
                f(next)
            }
        }

        let mut accept = false;
        if kind == 1 {
            if ((owner + OWNER_FLAG) as *const u8).read() & 4 != 0 {
                accept = true;
            } else if next != 0 {
                if query(next, TYPE_SLOT) == 0x20 {
                    if query(next, SUB_SLOT) == 0x65 {
                        accept = true;
                    } else if query(next, TYPE_SLOT) == 0x5d {
                        accept = true;
                    }
                } else if query(next, TYPE_SLOT) == 0x5d {
                    accept = true;
                }
            }
        } else if kind == 2 {
            accept = true;
        }
        if !accept {
            return 0;
        }
        lf_checker_rt::callee_thiscall!(SETTLE, u32, this, SETTLE_ARG);
        let flags = (owner + OWNER_CLR) as *mut u32;
        flags.write_unaligned(flags.read_unaligned() & CLR_MASK);
        1
    }
});
