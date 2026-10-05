// original: 0x00B64F00 veh_bind_indexed
/// Bind index `a0`: copy head, wire pair, visit slot, notify with `(a0,a1)`.
///
/// Resolves `a0` (stubbed, cdecl/1), stores the index at `[this+0xc]`, copies
/// `[this]` to `[this+4]`. Calls the pair wirer (stubbed, thiscall/2) on
/// `this+0xa8` with `([this+3i*4+0x24], word[this+3i*4+0x28])`, visits slot
/// `this+3*(i+3)*4` (stubbed, thiscall/0), then notifies (stubbed, thiscall/5)
/// with `(a0, a1, 1, 0, 0)`. Thiscall, two stack words. No meaningful return.
export!(thiscall, rw_00b64f00(this: u32, a0: u32, a1: u32) -> u32 {
    unsafe {
        const IDX: u32 = 4;
        const SLOT: u32 = 0xc;
        const HEAD: u32 = 0;
        const PAIR_BASE: u32 = 0x24;
        const PAIR_WORD: u32 = 0x28;
        const WIRER: u32 = 0xa8;
        let h: u32 = callee_cdecl!(1, u32, a0);
        let i = ((h + IDX) as *const u32).read_unaligned();
        ((this + SLOT) as *mut u32).write_unaligned(i);
        let head = (this as *const u32).read_unaligned();
        ((this + 4) as *mut u32).write_unaligned(head);
        let p = this.wrapping_add(i.wrapping_mul(3).wrapping_mul(4));
        let w = ((p + PAIR_WORD) as *const u16).read_unaligned() as u32;
        let v = ((p + PAIR_BASE) as *const u32).read_unaligned();
        let _: u32 = callee_thiscall!(2, u32, this + WIRER, v, w);
        let slot = this.wrapping_add((i.wrapping_add(3).wrapping_mul(3)).wrapping_mul(4));
        let _: u32 = callee_thiscall!(3, u32, slot);
        let _: u32 = callee_thiscall!(4, u32, this, a0, a1, 1, 0, 0);
        0
    }
});
