// original: 0x008b84e0 analog_band_dispatch_b
/// Analog sensor band dispatcher (variant B).
///
/// Pre-fills `out` with band 0xfd's record, samples the shared analog sensor,
/// and re-fills `out` with the record of the band the reading falls into
/// (same five-threshold ladder as the other variants, top and bottom bands
/// both selecting 0xfd). Returns `out`. The original reuses its incoming
/// argument slot as scratch for the sample, which a rewrite cannot reproduce.
export!(cdecl, rw_008b84e0(out: u32) -> u32 {
    unsafe {
        const SENSOR: u32 = 0x0118D7F0;
        const C1: u32 = 0x00FE899C;
        const C2: u32 = 0x00FE8980;
        const C3: u32 = 0x00FE895C;
        const C4: u32 = 0x00FE8928;
        const C5: u32 = 0x00FE88E8;
        const PREFILL: u32 = 0xFD;
        let _: u32 = callee_cdecl!(3, u32, out, PREFILL);
        let sample: f32 = callee_thiscall!(2, f32, relocated(SENSOR), 1);
        let c1 = (relocated(C1) as *const f32).read();
        let c2 = (relocated(C2) as *const f32).read();
        let c3 = (relocated(C3) as *const f32).read();
        let c4 = (relocated(C4) as *const f32).read();
        let c5 = (relocated(C5) as *const f32).read();
        let code = if sample > c1 {
            0xFD
        } else if sample > c2 {
            0x100
        } else if sample > c3 {
            0x101
        } else if sample > c4 {
            0xFE
        } else if sample > c5 {
            0xFF
        } else {
            0xFD
        };
        let tmp = sample.to_bits();
        let filled: u32 = callee_cdecl!(4, u32, &tmp as *const u32 as u32, code);
        (out as *mut u32).write((filled as *const u32).read());
        (out.wrapping_add(4) as *mut u32)
            .write((filled.wrapping_add(4) as *const u32).read());
        out
    }
});
