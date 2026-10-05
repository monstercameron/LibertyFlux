// original: 0x00B66A80 veh_indexed_limit
/// Apply the indexed limit setter when the handle slot is occupied.
///
/// Returns at once when `[this+0x14]` is null. Asks for the live object
/// (stubbed, thiscall/0); returns when null. Otherwise resolves the handle
/// (stubbed, cdecl/1) from `[obj+0x18]`, reads index `i` at handle+4 and the
/// limit word at `[obj+0x60]`, and calls the indexed setter (stubbed,
/// thiscall/1) on slot `this+(3*(i+3))*4` with the limit. Thiscall, no stack
/// words. No meaningful return value.
export!(thiscall, rw_00b66a80(this: u32) -> u32 {
    unsafe {
        const HANDLE: u32 = 0x14;
        const REF: u32 = 0x18;
        const IDX: u32 = 4;
        const LIMIT: u32 = 0x60;
        if ((this + HANDLE) as *const u32).read_unaligned() == 0 {
            return 0;
        }
        let cur: u32 = callee_thiscall!(1, u32, this);
        if cur == 0 {
            return 0;
        }
        let h: u32 = callee_cdecl!(2, u32, ((cur + REF) as *const u32).read_unaligned());
        let i = ((h + IDX) as *const u32).read_unaligned();
        let w = ((cur + LIMIT) as *const u16).read_unaligned() as u32;
        let slot = this.wrapping_add((i.wrapping_add(3).wrapping_mul(3)).wrapping_mul(4));
        let _: u32 = callee_thiscall!(3, u32, slot, w);
        0
    }
});
