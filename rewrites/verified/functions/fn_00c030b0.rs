// original: 0x00c030b0 stream_init_66 (proposed)

/// Initialise a streaming record (tag `0x66`) through two callees plus flags.
///
/// `this` points to the record. Calls the registrar callee with
/// (event kind `0x66`, the shared streaming global, `a1`, 0, 0) and the
/// follow-up callee with `a2`; then folds the low 2 bits of `a4` (doubled)
/// and bit 0 of `a3` into the flag at `+0x1c` (keeping its top 5 bits),
/// stores the low word of `a5` at `+0x1e` and the marker `0x14` at `+0x02`.
/// Returns the low word of `a5` (what the original leaves in `eax`).
///
/// Original: 0x00c030b0 (thiscall, five stack words).
lf_checker_rt::export!(thiscall, rw_00c030b0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        const KIND: u32 = 0x66;
        const MARKER: u8 = 0x14;
        const GLOBAL_STREAM: u32 = 0x011735a4;
        let g = (lf_checker_rt::relocated(GLOBAL_STREAM) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(1, u32, this, KIND, g, a1, 0, 0);
        lf_checker_rt::callee_thiscall!(2, u32, this, a2);
        let old = ((this.wrapping_add(0x1c)) as *const u8).read();
        let mut cl = (((a4 & 0xff) as u8) & 3).wrapping_mul(2);
        cl |= old & 0xf8;
        cl |= ((a3 & 0xff) as u8) & 1;
        ((this.wrapping_add(0x1c)) as *mut u8).write(cl);
        (this.wrapping_add(0x1e) as *mut u16).write_unaligned((a5 & 0xffff) as u16);
        ((this.wrapping_add(2)) as *mut u8).write(MARKER);
        a5 & 0xffff
    }
});
