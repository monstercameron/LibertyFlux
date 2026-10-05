// original: 0x00887EC0 stream_state_probe (proposed)

/// Classify a stream tag against the object's tag slots under its lock.
///
/// Locks (callee 1) with the lock word at `this + 0x38`, compares the low
/// word of the argument against the tag words at `+0x40`, `+0x42` and
/// `+0x44` together with the state bytes at `+0x46`/`+0x47`, unlocks
/// (callee 2), and returns 0 when the tag matches the active slot while
/// both state bytes are clear, 1 when it matches the pending slot (or the
/// active slot while the pending slot is still empty), 2 when it matches
/// the spare slot, and 3 otherwise.
///
/// Original: 0x00887EC0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00887EC0(this: u32, tag: u32) -> u32 {
    unsafe {
        const LOCK_WORD: u32 = 0x38;
        const TAG_PENDING: u32 = 0x40;
        const TAG_ACTIVE: u32 = 0x42;
        const TAG_SPARE: u32 = 0x44;
        const STATE0: u32 = 0x46;
        const STATE1: u32 = 0x47;
        const EMPTY: u16 = 0xffff;
        const LOCK: u32 = 1;
        const UNLOCK: u32 = 2;
        let w = ((this + LOCK_WORD) as *const u32).read_unaligned();
        lf_checker_rt::callee_cdecl!(LOCK, u32, w);
        let ax = tag as u16;
        let active = ((this + TAG_ACTIVE) as *const u16).read_unaligned();
        let pending = ((this + TAG_PENDING) as *const u16).read_unaligned();
        let spare = ((this + TAG_SPARE) as *const u16).read_unaligned();
        let s0 = ((this + STATE0) as *const u8).read();
        let s1 = ((this + STATE1) as *const u8).read();
        let r = if s0 == 0 && s1 == 0 && ax == active {
            0
        } else if ax == active && pending == EMPTY {
            1
        } else if ax == pending {
            1
        } else if ax == spare {
            2
        } else {
            3
        };
        lf_checker_rt::callee_cdecl!(UNLOCK, u32, w);
        r
    }
});
