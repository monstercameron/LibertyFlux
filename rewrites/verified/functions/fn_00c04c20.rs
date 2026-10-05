// original: 0x00c04c20 stream_init_95 (proposed)

/// Initialise a streaming record (tag `0x95`) from a source object and args.
///
/// `this` points to the record, `src` to the source. Copies the source's
/// `+0x64` word to `+0x08` and its `+0x2e` half to `+0x22`, writes the shared
/// streaming global to `+0x04`, the low word of `a6` to `+0x18`, the float
/// `f8` to `+0x1c` and `a5` to `+0x28`; the flag at `+0x20` combines bit 0
/// of `a9`, all of `a10` doubled, the old bit `0x20` and the low 5 bits of
/// `a7`. Then pushes `p2`/`p3`/`p4` through three callees. Returns the third
/// callee's answer (what the original leaves in `eax`).
///
/// Original: 0x00c04c20 (thiscall, eleven stack words; the eleventh is unread).
lf_checker_rt::export!(thiscall, rw_00c04c20(this: u32, src: u32, p2: u32, p3: u32, p4: u32, a5: u32, a6: u32, a7: u32, f8: u32, a9: u32, a10: u32, _a11: u32) -> u32 {
    unsafe {
        const TAG: u8 = 0x95;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        (this as *mut u8).write(TAG);
        let w = (src.wrapping_add(0x64) as *const u32).read_unaligned();
        (this.wrapping_add(8) as *mut u32).write_unaligned(w);
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        (this.wrapping_add(4) as *mut u32).write_unaligned(g);
        let h = (src.wrapping_add(0x2e) as *const u16).read_unaligned();
        (this.wrapping_add(0x22) as *mut u16).write_unaligned(h);
        (this.wrapping_add(0x18) as *mut u16).write_unaligned((a6 & 0xffff) as u16);
        let mut cl = (((a9 & 0xff) as u8) & 1) | (((a10 & 0xff) as u8).wrapping_mul(2));
        let old20 = ((this.wrapping_add(0x20)) as *const u8).read();
        cl = cl.wrapping_shl(6);
        cl |= old20 & 0x20;
        cl ^= ((a7 & 0xff) as u8) & 0x1f;
        ((this.wrapping_add(0x20)) as *mut u8).write(cl);
        (this.wrapping_add(0x1c) as *mut u32).write_unaligned(f8);
        (this.wrapping_add(0x28) as *mut u32).write_unaligned(a5);
        lf_checker_rt::callee_thiscall!(1, u32, this, p2);
        lf_checker_rt::callee_thiscall!(2, u32, this, p3);
        lf_checker_rt::callee_thiscall!(3, u32, this, p4)
    }
});
