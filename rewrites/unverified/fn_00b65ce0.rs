// original: 0x00B65CE0 veh_reconcile_link
/// Reconcile the link at `[this+0x14]` against the live object, then settle.
///
/// Zeroes `[this+0xb8]`; returns when `[this+0x14]` is null. Probes the link
/// (stubbed, thiscall/1) and arms when its low byte is set while `a0`'s is
/// clear. Asks for the live object (stubbed, thiscall/0); when null, settles
/// (visitor (stubbed, thiscall/1) if disarmed, release (stubbed, thiscall/0)
/// if armed). Otherwise resolves its word at +0x18 (stubbed, cdecl/1) and
/// settles unless slot `(3*index+9)` still holds that word. Resolves again,
/// asks for the index (stubbed, thiscall/0); on a match with the sign
/// extended tag at `[link+0x2e]`, or when armed, stores the word at
/// `[live+0x60]` (stubbed, thiscall/1) and settles. Otherwise visits the slot
/// (stubbed, thiscall/0) and the link (stubbed, thiscall/1). The original
/// also compares `a1` against its own return address on that path, which is
/// unmatchable in trials, so the fallthrough always runs. Thiscall, two
/// stack words. No meaningful return value.
export!(thiscall, rw_00b65ce0(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x14;
        const CLEAR: u32 = 0xb8;
        const REF: u32 = 0x18;
        const IDX: u32 = 4;
        const TAG: u32 = 0x2e;
        const WORD: u32 = 0x60;
        ((this + CLEAR) as *mut u32).write_unaligned(0);
        let link = ((this + LINK) as *const u32).read_unaligned();
        if link == 0 {
            return 0;
        }
        let r1: u32 = callee_thiscall!(1, u32, this, link);
        let armed = (r1 & 0xFF) != 0 && (a0 & 0xFF) == 0;
        let cur: u32 = callee_thiscall!(2, u32, this);
        // Settle paths: visit the link when disarmed, release when armed.
        if cur == 0 {
            if !armed {
                let _: u32 = callee_thiscall!(6, u32, this, this + LINK);
            } else {
                let _: u32 = callee_thiscall!(8, u32, this);
            }
            return 0;
        }
        let h: u32 = callee_cdecl!(3, u32, ((cur + REF) as *const u32).read_unaligned());
        let i = ((h + IDX) as *const u32).read_unaligned();
        let slot = this.wrapping_add((i.wrapping_mul(3).wrapping_add(9)).wrapping_mul(4));
        if (slot as *const u32).read_unaligned()
            != ((cur + REF) as *const u32).read_unaligned()
        {
            if !armed {
                let _: u32 = callee_thiscall!(6, u32, this, this + LINK);
            } else {
                let _: u32 = callee_thiscall!(8, u32, this);
            }
            return 0;
        }
        let h2: u32 = callee_cdecl!(3, u32, ((cur + REF) as *const u32).read_unaligned());
        let idx: u32 = callee_thiscall!(4, u32, h2);
        let want = (((link + TAG) as *const u16).read_unaligned() as i16) as i32 as u32;
        if idx == want || armed {
            let w = ((cur + WORD) as *const u16).read_unaligned() as u32;
            let _: u32 = callee_thiscall!(7, u32, slot, w);
            if !armed {
                let _: u32 = callee_thiscall!(6, u32, this, this + LINK);
            } else {
                let _: u32 = callee_thiscall!(8, u32, this);
            }
            return 0;
        }
        // a1 is compared against the return address here; unmatchable in
        // trials (ASLR), so the fallthrough below always runs.
        let _: u32 = callee_thiscall!(5, u32, slot);
        let _: u32 = callee_thiscall!(6, u32, this, this + LINK);
        0
    }
});
