// original: 0x00BE88A0 record_init_masked (proposed)
/// Initialise a record with a value, clearing state bits.
///
/// `obj` points to the record and `value` is stored at `+0x04`. The id
/// word at `+0x00` becomes -1, the dword at `+0x28` and the byte at
/// `+0x2e` are zeroed, and the flag bytes at `+0x2c`/`+0x2d` keep only
/// bits 0xec/0x80 of their previous contents. Returns the object.
///
/// Original: 0x00BE88A0 (thiscall, one stack word). The two flag bytes
/// merge with whatever they held, so their fills matter.
lf_checker_rt::export!(thiscall, rw_00BE88A0(obj: u32, value: u32) -> u32 {
    unsafe {
        const VALUE: u32 = 0x04;
        const ID: u32 = 0x00;
        const COUNT: u32 = 0x28;
        const FLAGS_LO: u32 = 0x2c;
        const FLAGS_HI: u32 = 0x2d;
        const MODE: u32 = 0x2e;
        const FLAGS_LO_KEEP: u8 = 0xec;
        const FLAGS_HI_KEEP: u8 = 0x80;
        ((obj + VALUE) as *mut u32).write_unaligned(value);
        ((obj + ID) as *mut u32).write_unaligned(0xffff_ffff);
        ((obj + COUNT) as *mut u32).write_unaligned(0);
        let lo = (obj + FLAGS_LO) as *mut u8;
        lo.write(lo.read() & FLAGS_LO_KEEP);
        let hi = (obj + FLAGS_HI) as *mut u8;
        hi.write(hi.read() & FLAGS_HI_KEEP);
        ((obj + MODE) as *mut u8).write(0);
        obj
    }
});

