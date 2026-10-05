// original: 0x00B65DB0 veh_advance_full
/// Advance the small state machine to FULL (0xb), notifying along the way.
///
/// Returns at once when the count at `[this+0xc]` is already 0xb. Rebinds
/// (stubbed, thiscall/2) with `(1, 0)` at once when the head at `[this]`
/// equals the count; otherwise asks for the live object (stubbed, thiscall/0)
/// and rebinds only when it is null.
/// Visits slot `this+3*(count+3)*4` (stubbed, thiscall/0). Unless the kind at
/// `[this+0xa8]` is 0x2e with a null link at `[this+0x18]`, notifies (stubbed,
/// thiscall/5) with `(kind, word[0xac], 0, 0, 0)`. Revisits `this+0xa8`. When
/// head still equals count, reports (stubbed, thiscall/4) with
/// `(0, pick, 1, 0)` where `pick` is `[this+4]` if the cell at
/// `this+3*[this+4]*4+0x24` is positive, else 0. Stores 0xb to count and aux.
/// Thiscall, no stack words. No meaningful return value.
export!(thiscall, rw_00b65db0(this: u32) -> u32 {
    unsafe {
        const COUNT: u32 = 0xc;
        const HEAD: u32 = 0;
        const AUX: u32 = 4;
        const KIND: u32 = 0xa8;
        const WORD: u32 = 0xac;
        const LINK: u32 = 0x18;
        const PAIR: u32 = 0x24;
        const FULL: u32 = 0xb;
        const HEAVY: u32 = 0x2e;
        if ((this + COUNT) as *const u32).read_unaligned() == FULL {
            return 0;
        }
        if ((this + HEAD) as *const u32).read_unaligned()
            == ((this + COUNT) as *const u32).read_unaligned()
        {
            let _: u32 = callee_thiscall!(2, u32, this, 1, 0);
        } else {
            let cur: u32 = callee_thiscall!(1, u32, this);
            if cur == 0 {
                let _: u32 = callee_thiscall!(2, u32, this, 1, 0);
            }
        }
        let n = ((this + COUNT) as *const u32).read_unaligned();
        let slot = this.wrapping_add((n.wrapping_add(3).wrapping_mul(3)).wrapping_mul(4));
        let _: u32 = callee_thiscall!(3, u32, slot);
        let kind = ((this + KIND) as *const u32).read_unaligned();
        if kind != HEAVY || ((this + LINK) as *const u32).read_unaligned() != 0 {
            let w = ((this + WORD) as *const u16).read_unaligned() as u32;
            let _: u32 = callee_thiscall!(4, u32, this, kind, w, 0, 0, 0);
        }
        let _: u32 = callee_thiscall!(3, u32, this + KIND);
        if ((this + HEAD) as *const u32).read_unaligned()
            == ((this + COUNT) as *const u32).read_unaligned()
        {
            let a = ((this + AUX) as *const u32).read_unaligned();
            let cell = ((this.wrapping_add((a.wrapping_mul(3)).wrapping_mul(4)) + PAIR)
                as *const i32)
                .read_unaligned();
            let pick = if cell > 0 { a } else { 0 };
            let _: u32 = callee_thiscall!(5, u32, this, 0, pick, 1, 0);
        }
        ((this + COUNT) as *mut u32).write_unaligned(FULL);
        ((this + AUX) as *mut u32).write_unaligned(FULL);
        0
    }
});
