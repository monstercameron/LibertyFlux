// original: 0x00c036b0 stream_reg_9a (proposed)

/// Register a streaming event (kind `0x9a`) and store one of two payloads.
///
/// `this` points to the record. Calls the registrar callee with
/// (`0x9a`, the shared streaming global, `a2`, `a1`, 0), then sets bit 1 of
/// `+0x03` to bit 0 of `a5`. When the bit is set, stores `a6` at `+0x10`;
/// a non-zero `a7` further stores its low word at `+0x14` and `0x3039` at
/// `+0x16`, while a zero `a7` pushes `a3` through the follow-up callee
/// instead. When the bit is clear, pushes `a3` through the follow-up callee
/// and stores the float `f4` at `+0x10`. Always writes the marker `0x15` at
/// `+0x02`. Returns `0x3039`, or the follow-up callee's answer on the paths
/// that call it (what the original leaves in `eax`).
///
/// Original: 0x00c036b0 (thiscall, seven stack words).
lf_checker_rt::export!(thiscall, rw_00c036b0(this: u32, a1: u32, a2: u32, a3: u32, f4: u32, a5: u32, a6: u32, a7: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x9a;
        const MARKER: u8 = 0x15;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        const PRESENT: u16 = 0x3039;
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND, g, a2, a1, 0);
        let old3 = ((this.wrapping_add(3)) as *const u8).read();
        let mut a = (((a5 & 0xff) as u8).wrapping_mul(2)) ^ old3;
        a &= 2;
        let new3 = old3 ^ a;
        ((this.wrapping_add(3)) as *mut u8).write(new3);
        if new3 & 2 != 0 {
            (this.wrapping_add(0x10) as *mut u32).write_unaligned(a6);
            if a7 != 0 {
                (this.wrapping_add(0x14) as *mut u16).write_unaligned((a7 & 0xffff) as u16);
                (this.wrapping_add(0x16) as *mut u16).write_unaligned(PRESENT);
                ((this.wrapping_add(2)) as *mut u8).write(MARKER);
                u32::from(PRESENT)
            } else {
                let ans = lf_checker_rt::callee_thiscall!(2, u32, this, a3);
                ((this.wrapping_add(2)) as *mut u8).write(MARKER);
                ans
            }
        } else {
            let ans = lf_checker_rt::callee_thiscall!(2, u32, this, a3);
            (this.wrapping_add(0x10) as *mut u32).write_unaligned(f4);
            ((this.wrapping_add(2)) as *mut u8).write(MARKER);
            ans
        }
    }
});
