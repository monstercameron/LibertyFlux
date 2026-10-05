// original: 0x00B64E70 veh_adjust_limit
/// Adjust a live limit, or route through the indexed fallback.
///
/// Asks for the live object (stubbed, thiscall/0). When non-null and its word
/// at +0x18 equals `a0`, computes `d = [obj+0x60] - [obj+0x5c] + sx(a1)`: for
/// non-negative `d` calls the applier (stubbed, thiscall/1) with `sx(a1)`,
/// else resolves the cap object (stubbed, cdecl/1), clamps
/// `min([obj+0x5c]+d, sx(capword))` at zero (signed) and calls the setter
/// (stubbed, thiscall/1) before the applier. When the live object is missing
/// or stale, resolves `a0` (stubbed, cdecl/1), and when slot `(3*index+9)`
/// holds `a0` calls the fallback setter (stubbed, thiscall/1) with `a1`.
/// Thiscall, two stack words. No meaningful return value.
export!(thiscall, rw_00b64e70(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const CUR: u32 = 0x18;
        const HI: u32 = 0x60;
        const LO: u32 = 0x5c;
        const CAPW: u32 = 0x86;
        const IDX: u32 = 4;
        let obj: u32 = callee_thiscall!(1, u32, this);
        if obj != 0 && ((obj + CUR) as *const u32).read_unaligned() == a0 {
            let sx = ((a1 & 0xFFFF) as u16 as i16) as i32;
            let d = ((obj + HI) as *const u32).read_unaligned()
                .wrapping_sub(((obj + LO) as *const u32).read_unaligned())
                .wrapping_add(sx as u32);
            if (d as i32) < 0 {
                let h: u32 = callee_cdecl!(2, u32, ((obj + CUR) as *const u32).read_unaligned());
                let cap = ((h + CAPW) as *const i16).read_unaligned() as i32;
                let e = ((obj + LO) as *const u32).read_unaligned().wrapping_add(d);
                let v: i32 = if (e as i32) < 0 {
                    0
                } else if (e as i32) > cap {
                    cap
                } else {
                    e as i32
                };
                let _: u32 = callee_thiscall!(3, u32, obj, v as u32);
            }
            let _: u32 = callee_thiscall!(4, u32, obj, sx as u32);
            return 0;
        }
        let h: u32 = callee_cdecl!(2, u32, a0);
        let i = ((h + IDX) as *const u32).read_unaligned();
        let slot = this.wrapping_add((i.wrapping_mul(3).wrapping_add(9)).wrapping_mul(4));
        if (slot as *const u32).read_unaligned() != a0 {
            return 0;
        }
        let _: u32 = callee_thiscall!(5, u32, slot, a1);
        0
    }
});
