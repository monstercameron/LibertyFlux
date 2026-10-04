// original: 0x00cb5c50 heading_deviation_check
/// True when the normalised leg angle deviates from the base heading.
export!(thiscall, rw_cb5c50(this_ptr: u32, a0: u32, count: u32, a2: u32) -> u8 {
    unsafe {
        if (count as i32) < 2 {
            return 0;
        }
        let p0 = ((a0.wrapping_add(0x20)) as *const u32).read();
        let f1 = ((a2.wrapping_add(0x10)) as *const u32).read();
        let f2 = ((a2.wrapping_add(0x14)) as *const u32).read();
        let f3 = ((p0.wrapping_add(0x30)) as *const u32).read();
        let f4 = ((p0.wrapping_add(0x34)) as *const u32).read();
        let r1: f32 = lf_checker_rt::callee_cdecl!(1, f32, f1, f2, f3, f4);
        let r2: f32 = lf_checker_rt::callee_cdecl!(2, f32, r1.to_bits());
        let base = ((this_ptr.wrapping_add(0x2C)) as *const f32).read();
        let r3: f32 = lf_checker_rt::callee_cdecl!(2, f32, (base - r2).to_bits());
        if r3.abs() > core::f32::consts::FRAC_PI_2 {
            1
        } else {
            0
        }
    }
});
