// original: 0x00a128e0 latched_pair_snapshot (proposed)
/// Copy two latched globals to outputs once, then park them at a sentinel.
///
/// Compares each of the globals at 0x0103b760 and 0x0103b764 against the
/// constant at 0x00e9afb4 with unordered-aware float inequality (NaN counts
/// as different, like the original's `ucomiss`/`lahf` idiom). When different,
/// the global's bits are written to the matching output pointer and the
/// global is set to -999.0 (0xc479c000). No return value is set; the exit
/// register holds an argument or entry value, so the contract does not
/// compare it. Stdcall, two stack arguments.
export!(stdcall, rw_00a128e0(out0: u32, out1: u32) -> u32 {
    unsafe {
        const G0: u32 = 0x0103b760;
        const G1: u32 = 0x0103b764;
        const IDLE: u32 = 0x00e9afb4;
        const SENTINEL: u32 = 0xc479c000;
        let idle = f32::from_bits(*global::<u32>(IDLE));
        let g0 = f32::from_bits(*global::<u32>(G0));
        if g0 != idle {
            (out0 as *mut u32).write_unaligned(g0.to_bits());
            *global::<u32>(G0) = SENTINEL;
        }
        let g1 = f32::from_bits(*global::<u32>(G1));
        if g1 != idle {
            (out1 as *mut u32).write_unaligned(g1.to_bits());
            *global::<u32>(G1) = SENTINEL;
        }
        0
    }
});
