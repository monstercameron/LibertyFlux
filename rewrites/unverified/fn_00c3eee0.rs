// original: 0x00c3eee0 train_parse_fields_via_scan (proposed)
/// Parse twelve fields through a scanner into this object's slots.
///
/// `this` (ECX) is the object, `arg0` an input word. Calls scanner id 1
/// (cdecl, twelve words): arg0, a format address, eight pointers derived
/// from this (`this` itself and `+0x04..+0x18` except `+0x10`, which goes
/// last), and three scratch pointers into the original's own stack frame.
/// The contract skips those three addresses and scripts the three words
/// the scanner stores through them; the rewrite passes its own locals.
/// Afterwards the first scratch word, tested against zero, becomes the
/// byte at `this+0x2c`, and the other two become the dwords at
/// `this+0x28` and `this+0x24`. Returns the third scratch word.
///
/// Original: 0x00c3eee0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00c3eee0(this: u32, arg0: u32) -> u32 {
    unsafe {
        const SCAN: u32 = 1;
        const FORMAT: u32 = 0xec8684;
        const OUT0: u32 = 0x2c;
        const OUT1: u32 = 0x28;
        const OUT2: u32 = 0x24;
        let mut s = [0u32; 3];
        let sp = s.as_mut_ptr();
        let fmt = lf_checker_rt::relocated(FORMAT);
        let _: u32 = lf_checker_rt::callee_cdecl!(
            SCAN, u32, arg0, fmt, this, this.wrapping_add(4), this.wrapping_add(8),
            this.wrapping_add(0xc), this.wrapping_add(0x14), this.wrapping_add(0x18),
            (sp.add(2)) as u32, (sp.add(1)) as u32, (sp) as u32, this.wrapping_add(0x10)
        );
        ((this + OUT0) as *mut u8).write((s[0] != 0) as u8);
        ((this + OUT1) as *mut u32).write_unaligned(s[1]);
        ((this + OUT2) as *mut u32).write_unaligned(s[2]);
        s[2]
    }
});
