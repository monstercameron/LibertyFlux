// original: 0x008b8600 analog_band_dispatch_c
/// Analog sensor band dispatcher (variant C).
///
/// Samples the shared analog sensor, classifies the reading into one of six
/// bands by comparing it against a descending ladder of five thresholds, and
/// asks the band-table filler for the band's 8-byte record, which is copied to
/// `out`. Returns `out`. Same thresholds as variant A with a different band
/// code table.
export!(cdecl, rw_008b8600(out: u32) -> u32 {
    unsafe {
        const SENSOR: u32 = 0x0118D7F0;
        const C1: u32 = 0x00FE899C;
        const C2: u32 = 0x00FE8980;
        const C3: u32 = 0x00FE895C;
        const C4: u32 = 0x00FE8928;
        const C5: u32 = 0x00FE88E8;
        let sample: f32 = callee_thiscall!(2, f32, relocated(SENSOR), 1);
        let c1 = (relocated(C1) as *const f32).read();
        let c2 = (relocated(C2) as *const f32).read();
        let c3 = (relocated(C3) as *const f32).read();
        let c4 = (relocated(C4) as *const f32).read();
        let c5 = (relocated(C5) as *const f32).read();
        let code = if sample > c1 {
            6
        } else if sample > c2 {
            0x104
        } else if sample > c3 {
            0x105
        } else if sample > c4 {
            0x102
        } else if sample > c5 {
            0x103
        } else {
            6
        };
        let tmp = sample.to_bits();
        let filled: u32 = callee_cdecl!(3, u32, &tmp as *const u32 as u32, code);
        (out as *mut u32).write((filled as *const u32).read());
        (out.wrapping_add(4) as *mut u32)
            .write((filled.wrapping_add(4) as *const u32).read());
        out
    }
});
