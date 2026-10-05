// original: 0x00B65AE0 veh_gate_visit_reset
/// Forward under two readiness gates, then visit the indexed slot and reset.
///
/// Calls the guarded forwarder (stubbed, thiscall/3) with `(a0, 1, 0)` when
/// `[this+0x14]` is non-null with marker byte 3 at +0x22a. Returns unless
/// `[this+0x18]` is likewise ready. Then reads the next link at +0x25c: when
/// non-null, resolves its word at +0x18 (stubbed, cdecl/1) and, for a non-null
/// answer, visits slot `this+3*(index+3)*4` (stubbed, thiscall/0). Always
/// finishes by resetting `[this+0x18]` (stubbed, thiscall/1). Thiscall, one
/// stack word. No meaningful return value.
export!(thiscall, rw_00b65ae0(this: u32, a0: u32) -> u32 {
    unsafe {
        const LINK0: u32 = 0x14;
        const LINK1: u32 = 0x18;
        const READY: u32 = 0x22a;
        const NEXT: u32 = 0x25c;
        const IDX: u32 = 4;
        const CODE: u32 = 3;
        let o0 = ((this + LINK0) as *const u32).read_unaligned();
        if o0 != 0 && ((o0 + READY) as *const u8).read() == CODE as u8 {
            let _: u32 = callee_thiscall!(1, u32, this, a0, 1, 0);
        }
        let o1 = ((this + LINK1) as *const u32).read_unaligned();
        if o1 == 0 || ((o1 + READY) as *const u8).read() != CODE as u8 {
            return 0;
        }
        let o2 = ((o1 + NEXT) as *const u32).read_unaligned();
        if o2 != 0 {
            let h: u32 = callee_cdecl!(2, u32, ((o2 + LINK1) as *const u32).read_unaligned());
            if h != 0 {
                let i = ((h + IDX) as *const u32).read_unaligned();
                let slot = this.wrapping_add((i.wrapping_add(3).wrapping_mul(3)).wrapping_mul(4));
                let _: u32 = callee_thiscall!(3, u32, slot);
            }
        }
        let _: u32 = callee_thiscall!(4, u32, this, this + LINK1);
        0
    }
});
