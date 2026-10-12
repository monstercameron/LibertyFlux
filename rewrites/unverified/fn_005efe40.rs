// original: 0x005EFE40 status_flag_check_8 (proposed)

/// True when status bit 3 is set and the ready byte is nonzero.
///
/// Reads the flag byte at `this + 0x28` and the ready byte at `this + 0xc`.
/// Returns 1 only when bit 3 of the flag byte is set and the ready byte is
/// nonzero; otherwise returns 0. Reads memory only, writes nothing.
/// Original is thiscall with no stack arguments, returning the result in AL.
lf_checker_rt::export!(thiscall, rw_005efe40(this: u32) -> u32 {
    unsafe {
        const FLAG_OFFSET: u32 = 0x28;
        const FLAG_BIT: u8 = 0x08;
        const READY_OFFSET: u32 = 0x0c;
        let flags = (this.wrapping_add(FLAG_OFFSET) as *const u8).read_unaligned();
        if flags & FLAG_BIT == 0 {
            return 0;
        }
        let ready = (this.wrapping_add(READY_OFFSET) as *const u8).read_unaligned();
        u32::from(ready != 0)
    }
});
