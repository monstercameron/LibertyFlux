// original: 0x00c04cb0 stream_init_94 (proposed)

/// Initialise a streaming record (tag `0x94`) from a source object and args.
///
/// `this` points to the record, `src` to the source. Copies the source's
/// `+0x64` word to `+0x08` and its `+0x2e` half to `+0x22`, writes the shared
/// streaming global to `+0x04`, the floats `f3`/`f4` to `+0x18`/`+0x1c`, and
/// sets bit `0x20` of `+0x20` to bit 0 of `a5`. Pushes `p2` through one
/// callee and returns that callee's answer (what the original leaves in
/// `eax`).
///
/// Original: 0x00c04cb0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00c04cb0(this: u32, src: u32, p2: u32, f3: u32, f4: u32, a5: u32) -> u32 {
    unsafe {
        const TAG: u8 = 0x94;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        (this as *mut u8).write(TAG);
        let w = (src.wrapping_add(0x64) as *const u32).read_unaligned();
        (this.wrapping_add(8) as *mut u32).write_unaligned(w);
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        (this.wrapping_add(4) as *mut u32).write_unaligned(g);
        let h = (src.wrapping_add(0x2e) as *const u16).read_unaligned();
        (this.wrapping_add(0x22) as *mut u16).write_unaligned(h);
        let old20 = ((this.wrapping_add(0x20)) as *const u8).read();
        let mut a = (((a5 & 0xff) as u8).wrapping_shl(5)) ^ old20;
        a &= 0x20;
        (this.wrapping_add(0x18) as *mut u32).write_unaligned(f3);
        (this.wrapping_add(0x1c) as *mut u32).write_unaligned(f4);
        ((this.wrapping_add(0x20)) as *mut u8).write(old20 ^ a);
        lf_checker_rt::callee_thiscall!(1, u32, this, p2)
    }
});
