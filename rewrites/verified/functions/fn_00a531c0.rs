// original: 0x00a531c0 vehicle_emit_box (proposed)

/// Emit the shape for the current mode into the `out` buffer.
///
/// A mode word of -1 first runs the mode callee, then the mode is read
/// again: mode 0 copies the center and replicates the half-size through the
/// buffer and clears the trailing word; mode 4 runs the emit callee with
/// `out` in ecx and two interior pointers; any other mode does nothing.
/// Thiscall, one stack word, two callees, no result.
lf_checker_rt::export!(thiscall, rw_00a531c0(this: u32, out: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x1a04;
        const C0: u32 = 0x1990;
        const C1: u32 = 0x1994;
        const C2: u32 = 0x1998;
        const HALF: u32 = 0x1a00;
        const P0: u32 = 0x19b0;
        const P1: u32 = 0x19f0;
        const UNSET: u32 = 0xffff_ffff;
        const GETMODE: u32 = 1;
        const EMIT: u32 = 2;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        if rd32(this.wrapping_add(MODE)) == UNSET {
            lf_checker_rt::callee_thiscall!(GETMODE, u32, this);
        }
        let mode = rd32(this.wrapping_add(MODE));
        if mode == 0 {
            wr32(out.wrapping_add(0x00), rd32(this.wrapping_add(C0)));
            wr32(out.wrapping_add(0x04), rd32(this.wrapping_add(C1)));
            wr32(out.wrapping_add(0x08), rd32(this.wrapping_add(C2)));
            let h = rd32(this.wrapping_add(HALF));
            wr32(out.wrapping_add(0x18), h);
            wr32(out.wrapping_add(0x14), h);
            wr32(out.wrapping_add(0x10), h);
            wr32(out.wrapping_add(0x50), 0);
        } else if mode == 4 {
            lf_checker_rt::callee_thiscall!(
                EMIT, u32, out, this.wrapping_add(P0), this.wrapping_add(P1));
        }
        0
    }
});
