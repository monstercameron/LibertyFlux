// original: 0x00b76b90 anim_child_swap (proposed)

/// Swap the animation child at `this+0x98c` for the second word.
///
/// Stores the second word at `this+0x98c` and the third word's low byte at
/// `this+0xbc9`. The first word's low byte selects the mode update: nonzero
/// with mode 1 at `this+0xbc8` records mode 2 and returns; zero falls into
/// the mode logic (mode 3 or 2 records mode 4; mode 0 with a non-null owner
/// at `this+0x964` runs the stop callee, thiscall on the owner with one
/// stack word 0). Finally, unless the displaced child is null or its word
/// at +0x98 is zero, runs the release callee (thiscall on the displaced
/// child). Returns the release answer when it runs, else the stop answer
/// when that ran, else the second word with its low byte replaced by the
/// mode (flag-clear path) or third-word byte (flag-set path).
///
/// Original: 0x00b76b90 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00b76b90(this: u32, s1: u32, s2: u32, s3: u32) -> u32 {
    unsafe {
        const CHILD_OFF: u32 = 0x98c;
        const FLAG_OFF: u32 = 0xbc9;
        const MODE_OFF: u32 = 0xbc8;
        const OWNER_OFF: u32 = 0x964;
        const CHILD_LIVE_OFF: u32 = 0x98;
        const STOP: u32 = 1;
        const RELEASE: u32 = 2;
        let old = ((this + CHILD_OFF) as *const u32).read_unaligned();
        ((this + CHILD_OFF) as *mut u32).write_unaligned(s2);
        ((this + FLAG_OFF) as *mut u8).write(s3 as u8);
        let mut ans = (s2 & 0xffff_ff00) | (s3 & 0xff);
        if (s1 as u8) != 0 {
            if ((this + MODE_OFF) as *const u8).read() == 1 {
                ((this + MODE_OFF) as *mut u8).write(2);
                return ans;
            }
        } else {
            let mode = ((this + MODE_OFF) as *const u8).read();
            ans = (s2 & 0xffff_ff00) | (mode as u32);
            if mode == 3 || mode == 2 {
                ((this + MODE_OFF) as *mut u8).write(4);
            } else if mode == 0 {
                let owner = ((this + OWNER_OFF) as *const u32).read_unaligned();
                if owner != 0 {
                    ans = lf_checker_rt::callee_thiscall!(STOP, u32, owner, 0);
                }
            }
        }
        if old == 0 {
            return ans;
        }
        if ((old + CHILD_LIVE_OFF) as *const u32).read_unaligned() == 0 {
            return ans;
        }
        lf_checker_rt::callee_thiscall!(RELEASE, u32, old)
    }
});
