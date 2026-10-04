// original: 0x88d330 rage::audVoiceDSoundAdpcm::vf3
// Seek the ADPCM voice to a sample: looping voices map the position
// through the resampling helper, plain voices scale it by the rate in
// float and truncate with the x87 unit, then the buffer region is
// refilled, the play cursor published and the voice gain forwarded.
// The float path overwrites the function's own incoming argument slot,
// so the stack comparison is off for this function (the value is still
// verified through the cursor it produces).

lf_k2_rt::export!(thiscall, rw_0088d330(this: *mut u8, arg: u32) -> () {
    unsafe {
        let rd = |off: usize| -> u32 { *(this.add(off) as *const u32) };
        let entry_flags = *this.add(0x8c);

        // Target sample in bytes.
        let cursor: u32;
        if entry_flags & 2 != 0 && rd(0xa4) != 0 {
            let end = rd(0xb4);
            if arg < end {
                let r: u32 = lf_k2_rt::callee_cdecl!(1, u32, arg, rd(0xc));
                cursor = rd(0xc4).wrapping_add(r.wrapping_mul(2)).wrapping_sub(rd(0xa4));
            } else {
                let r: u32 = lf_k2_rt::callee_cdecl!(1, u32, arg.wrapping_sub(end), rd(0xc));
                cursor = r.wrapping_add(r);
            }
        } else {
            // Unsigned int to float through the bias table, scaled by the
            // milli-rate, magic-rounded, then truncated by the x87 unit.
            let cvt = |v: u32| -> f32 {
                let d = (v as i32) as f64;
                let m = if v >> 31 == 0 { 0.0f64 } else { 4294967296.0f64 };
                (d + m) as f32
            };
            let mut x5 = cvt(rd(0xc)) * cvt(arg);
            x5 = x5 * f32::from_bits(0x3a83126f);
            let x5b = x5.to_bits();
            let sign = x5b & 0x80000000;
            let ax = f32::from_bits(x5b ^ sign);
            let mask: u32 = if ax < f32::from_bits(0x4b000000) { 0xffffffff } else { 0 };
            let x2 = f32::from_bits((0x4b000000 & mask) | sign);
            let mut x1 = x5 + x2;
            x1 = x1 - x2;
            let frac = x1 - x5;
            let adjust: u32 = if frac > f32::from_bits(sign) { 0xffffffff } else { 0 };
            x1 = x1 - f32::from_bits(0x3f800000 & adjust);
            // fistp qword with chop rounding; only the low half is kept.
            let q: i64 = if x1.is_nan() || x1 >= 9223372036854775808.0
                || x1 < -9223372036854775808.0
            {
                0x8000000000000000u64 as i64
            } else {
                x1 as i64
            };
            cursor = (q as u32).wrapping_add(q as u32);
        }

        // Refill the region for loop-region voices.
        if entry_flags & 0x10 != 0 {
            lf_k2_rt::callee_thiscall!(2, u32, this as u32, 1);
        }

        // Publish the cursor and forward the voice gain.
        let buf = rd(0x90);
        let vt = *(buf as *const u32);
        let set_pos: extern "stdcall" fn(u32, u32) -> u32 =
            core::mem::transmute(*((vt as *const u8).add(0x34) as *const u32));
        set_pos(buf, cursor);
        let params = rd(4);
        let gain_bits = *(params as *const u32);
        lf_k2_rt::callee_thiscall!(4, u32, this as u32, gain_bits);
        *this.add(0x8c) = entry_flags | 8;
    }
});
