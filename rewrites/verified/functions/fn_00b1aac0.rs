// original: 0x00b1aac0 frontend_menu_toggle_on (symbols)

/// Applies a pending frontend-menu toggle when its trigger bit is new.
///
/// Thiscall with no stack arguments. The trigger is bit 0 of the
/// newly-set bits of the live mask against the latched mask: without it
/// the function returns -1 at once. Otherwise it copies the requested
/// id at +0x18 to +0x24 when nonnegative and changed (noting the change),
/// selects the next id into +0x28 from +0x1C, or +0x20 with flag 0x80 set
/// at +0x41 when +0x1C is -1, or -1 when both are -1 (clearing the flag
/// except on the +0x20 path), then: when the id changed and the previous
/// id was -1 it announces the switch; when it changed from a real id it
/// reverts to -1 and announces the clear; when unchanged it announces
/// only if the +0x24 copy changed. Announcements are thiscalls on the
/// shared announcer with a message constant. Returns the +0x18 id.
lf_checker_rt::export!(thiscall, rw_00b1aac0(this: u32) -> u32 {
    unsafe {
        const LIVE_MASK: u32 = 0x018b7a88;
        const LATCHED_MASK: u32 = 0x018b7a84;
        const ANNOUNCER: u32 = 0x01176888;
        const MSG_SWITCH: u32 = 0x00eab224;
        const MSG_CLEAR: u32 = 0x00eab23c;
        const NONE: u32 = 0xffff_ffff;
        const ALT_FLAG: u8 = 0x80;
        let live = (lf_checker_rt::global::<u32>(LIVE_MASK) as *const u32).read_unaligned();
        let latched =
            (lf_checker_rt::global::<u32>(LATCHED_MASK) as *const u32).read_unaligned();
        if ((live ^ latched) & live & 1) == 0 {
            return NONE;
        }
        let want = ((this + 0x18) as *const u32).read_unaligned();
        let mut copied = false;
        if (want as i32) >= 0 && ((this + 0x24) as *const u32).read_unaligned() != want {
            ((this + 0x24) as *mut u32).write_unaligned(want);
            copied = true;
        }
        let prev = ((this + 0x28) as *const u32).read_unaligned();
        let next = ((this + 0x1c) as *const u32).read_unaligned();
        if next != NONE {
            ((this + 0x28) as *mut u32).write_unaligned(next);
            let flag = (this + 0x41) as *mut u8;
            flag.write(flag.read() & !ALT_FLAG);
        } else {
            let alt = ((this + 0x20) as *const u32).read_unaligned();
            if alt != NONE {
                let flag = (this + 0x41) as *mut u8;
                flag.write(flag.read() | ALT_FLAG);
                ((this + 0x28) as *mut u32).write_unaligned(alt);
            } else {
                ((this + 0x28) as *mut u32).write_unaligned(NONE);
            }
        }
        let announcer = lf_checker_rt::relocated(ANNOUNCER);
        if ((this + 0x28) as *const u32).read_unaligned() != prev {
            if prev != NONE {
                let flag = (this + 0x41) as *mut u8;
                flag.write(flag.read() & !ALT_FLAG);
                ((this + 0x28) as *mut u32).write_unaligned(NONE);
            }
            if ((this + 0x28) as *const u32).read_unaligned() != NONE {
                lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    announcer,
                    lf_checker_rt::relocated(MSG_SWITCH)
                );
            } else {
                lf_checker_rt::callee_thiscall!(
                    1,
                    u32,
                    announcer,
                    lf_checker_rt::relocated(MSG_CLEAR)
                );
            }
        } else if copied {
            lf_checker_rt::callee_thiscall!(1, u32, announcer, lf_checker_rt::relocated(MSG_CLEAR));
        }
        want
    }
});
