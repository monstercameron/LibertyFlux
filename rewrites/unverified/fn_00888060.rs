// original: 0x00888060 stream_close (proposed)

/// Close a stream and detach it, under its lock.
///
/// Locks (callee 1). When the active byte at `+0x46` is set, drains
/// (callee 2) when the position at `+0x18` is positive, releases the view
/// (callee 3), clears the table word and position and parks the tag; when
/// it is clear, either empties the position (callee 4) or points the table
/// word at the view and rebases it (callee 5). Unlocks (callee 6) and,
/// unless the detached byte is set, notifies the waiter (callee 7). The
/// answer is the notify entry's answer when it runs, else the unlock's.
///
/// Original: 0x00888060 (thiscall, no stack words).
lf_checker_rt::export!(thiscall, rw_00888060(this: u32) -> u32 {
    unsafe {
        const LOCK_WORD: u32 = 0x38;
        const ACTIVE: u32 = 0x46;
        const DETACHED: u32 = 0x47;
        const POS: u32 = 0x18;
        const VIEW: u32 = 0x04;
        const P0: u32 = 0x0c;
        const TABLE: u32 = 0x3c;
        const TAG: u32 = 0x42;
        const WAITER: u32 = 0x30;
        const LOCK: u32 = 1;
        const DRAIN: u32 = 2;
        const RELEASE: u32 = 3;
        const EMPTY: u32 = 4;
        const REBASE: u32 = 5;
        const UNLOCK: u32 = 6;
        const NOTIFY: u32 = 7;
        let w = ((this + LOCK_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, w);
        let active = ((this + ACTIVE) as *const u8).read();
        if active != 0 {
            let pos = ((this + POS) as *const u32).read_unaligned();
            if pos > 0 {
                lf_checker_rt::callee_cdecl!(DRAIN, u32, 0);
            }
            let v = ((this + VIEW) as *const u32).read_unaligned();
            let p = ((this + P0) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(RELEASE, u32, v, 0, p);
            ((this + TABLE) as *mut u32).write_unaligned(0);
            ((this + TAG) as *mut u16).write_unaligned(0xffff);
            ((this + POS) as *mut u32).write_unaligned(0);
        } else if ((this + POS) as *const u32).read_unaligned() > 0 {
            lf_checker_rt::callee_thiscall!(EMPTY, u32, this, 0);
        } else {
            let v = ((this + VIEW) as *const u32).read_unaligned();
            ((this + TABLE) as *mut u32).write_unaligned(v);
            lf_checker_rt::callee_thiscall!(REBASE, u32, this);
        }
        if active != 0 {
            ((this + DETACHED) as *mut u8).write(0);
        }
        let u = lf_checker_rt::callee_cdecl!(UNLOCK, u32, w);
        let det = ((this + DETACHED) as *const u8).read();
        if det != 0 {
            u
        } else {
            let wt = ((this + WAITER) as *const u32).read_unaligned();
            lf_checker_rt::callee_cdecl!(NOTIFY, u32, wt)
        }
    }
});
