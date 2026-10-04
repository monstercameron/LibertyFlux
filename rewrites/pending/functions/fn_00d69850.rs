// original: 0x00d69850 probe_value_differs
// s16f09: report whether the probed value differs from the limit
// (thiscall/0).
//
// Runs the probe step over this record's linked child, converts the
// answer's low byte to float and compares against the global limit with
// `!=` (unordered included, matching the original's flag-parity trick).
// Empty slot reads as equal (returns 0).
export!(thiscall, rw_s16f09(this: *const u8) -> u32 {
    unsafe {
        let child = *((this.add(0x14)) as *const u32);
        if child == 0 {
            return 0;
        }
        let probe: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let sample = (probe(child) & 0xFF) as u8 as f32;
        let limit = *global::<f32>(0xFE8628);
        (sample != limit) as u32
    }
});
