// original: 0x00B62A00 veh_copy12_dispatch
/// Copy 12 dwords from `a0` to `a1` (skipping three slots), then dispatch.
///
/// Copies dwords at offsets 0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30,
/// 0x34, 0x38 (leaving 0x0c, 0x1c, 0x2c untouched). Resolves the context twice
/// (stubbed, thiscall/0); returns 0 when the first resolution is null.
/// Otherwise reads a dispatch word at `[ctx2 + a2*4 + 0x64]` and calls the
/// worker (stubbed, thiscall/4) with `(a0, a1, word, a3)`, returning its answer.
/// Thiscall, four stack words.
export!(thiscall, rw_00b62a00(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const OFFS: [u32; 12] = [0, 4, 8, 0x10, 0x14, 0x18, 0x20, 0x24, 0x28, 0x30, 0x34, 0x38];
        const DISP: u32 = 0x64;
        for o in OFFS {
            ((a1 + o) as *mut u32).write_unaligned(((a0 + o) as *const u32).read_unaligned());
        }
        let h1: u32 = callee_thiscall!(1, u32, this);
        if h1 == 0 {
            return 0;
        }
        let h2: u32 = callee_thiscall!(1, u32, this);
        let w = ((h2.wrapping_add(a2.wrapping_mul(4)).wrapping_add(DISP)) as *const u32)
            .read_unaligned();
        callee_thiscall!(2, u32, this, a0, a1, w, a3)
    }
});
