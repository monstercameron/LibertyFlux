// original: 0x00B65BD0 veh_maybe_forward_deep
/// Conditionally forward through a three-link chain, then always notify.
///
/// Resolves `a1` (stubbed, cdecl/1); when slot `(3*index+9)` holds `a1`,
/// visits it (stubbed, thiscall/0). When the low byte of `a2` is non-zero and
/// the chain `[this+0x14]` -> `[+0x25c]` -> `[+0x18]==a1` is fully present,
/// calls the guarded forwarder (stubbed, thiscall/3) with `(a0, a2, 0)`.
/// Always finishes by notifying (stubbed, thiscall/4) with `(7,0,0,0)`.
/// Thiscall, three stack words. No meaningful return value.
export!(thiscall, rw_00b65bd0(this: u32, a0: u32, a1: u32, a2: u32) -> u32 {
    unsafe {
        const IDX: u32 = 4;
        const LINK0: u32 = 0x14;
        const LINK1: u32 = 0x25c;
        const LINK2: u32 = 0x18;
        let h: u32 = callee_cdecl!(1, u32, a1);
        let i = ((h + IDX) as *const u32).read_unaligned();
        let slot = this.wrapping_add((i.wrapping_mul(3).wrapping_add(9)).wrapping_mul(4));
        if (slot as *const u32).read_unaligned() == a1 {
            let _: u32 = callee_thiscall!(2, u32, slot);
        }
        if (a2 & 0xFF) != 0 {
            let o1 = ((this + LINK0) as *const u32).read_unaligned();
            if o1 != 0 {
                let o2 = ((o1 + LINK1) as *const u32).read_unaligned();
                if o2 != 0 && ((o2 + LINK2) as *const u32).read_unaligned() == a1 {
                    let _: u32 = callee_thiscall!(3, u32, this, a0, a2, 0);
                }
            }
        }
        let _: u32 = callee_thiscall!(4, u32, this, 7, 0, 0, 0);
        0
    }
});
