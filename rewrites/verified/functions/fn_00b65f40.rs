// original: 0x00B65F40 veh_select_limit
/// Route a limit update through the live object or the indexed fallback.
///
/// Asks for the live object (stubbed, thiscall/0); when non-null and its word
/// at +0x18 still equals `a0`, asks again and calls the live setter (stubbed,
/// thiscall/1) with the low word of `a1`. Otherwise resolves `a0` (stubbed,
/// cdecl/1), computes slot `(3*index+9)` from the word at handle+4, and when
/// the slot holds `a0` calls the fallback setter (stubbed, thiscall/1) with
/// `a1`. Thiscall, two stack words. No meaningful return value.
export!(thiscall, rw_00b65f40(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const CUR: u32 = 0x18;
        const IDX: u32 = 4;
        let c1: u32 = callee_thiscall!(1, u32, this);
        if c1 != 0 {
            let c2: u32 = callee_thiscall!(1, u32, this);
            if ((c2 + CUR) as *const u32).read_unaligned() == a0 {
                let c3: u32 = callee_thiscall!(1, u32, this);
                let _: u32 = callee_thiscall!(3, u32, c3, a1 & 0xFFFF);
                return 0;
            }
        }
        let h: u32 = callee_cdecl!(2, u32, a0);
        let i = ((h + IDX) as *const u32).read_unaligned();
        let slot = this.wrapping_add((i.wrapping_mul(3).wrapping_add(9)).wrapping_mul(4));
        if (slot as *const u32).read_unaligned() != a0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(4, u32, slot, a1);
        0
    }
});
