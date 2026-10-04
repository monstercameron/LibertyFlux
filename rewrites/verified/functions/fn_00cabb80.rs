// original: 0x00CABB80 event_triple_construct (proposed)

/// Construct a triple-vector event record in place and return its address.
///
/// Runs the sized base constructor on `this`, installs the record's two
/// virtual tables, copies three words from the vector argument into each of
/// `+0x20`, `+0x30` and `+0x40` (nine stores), stores the two float
/// arguments (bitwise) at `+0x50` and `+0x54`, and zeroes the word at
/// `+0x58` and the byte at `+0x5A`. All float movement is bitwise.
///
/// Original: 0x00CABB80 (thiscall, three stack words).
lf_checker_rt::export!(thiscall, rw_00cabb80(this: u32, vec: u32, f0_bits: u32, f1_bits: u32) -> u32 {
    unsafe {
        const BASE_CTOR: u32 = 1;
        const VTABLE: u32 = 0x00ED8CBC;
        const VTABLE2: u32 = 0x00ED8D14;
        const BASE_SIZE_BITS: u32 = 0x40400000;
        lf_checker_rt::callee_thiscall!(BASE_CTOR, u32, this, BASE_SIZE_BITS);
        (this as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE));
        (this.wrapping_add(0x14) as *mut u32).write_unaligned(lf_checker_rt::relocated(VTABLE2));
        let mut dst = 0x20u32;
        while dst <= 0x40 {
            let mut i = 0u32;
            while i < 3 {
                let w = (vec.wrapping_add(i * 4) as *const u32).read_unaligned();
                (this.wrapping_add(dst + i * 4) as *mut u32).write_unaligned(w);
                i += 1;
            }
            dst += 0x10;
        }
        (this.wrapping_add(0x50) as *mut u32).write_unaligned(f0_bits);
        (this.wrapping_add(0x54) as *mut u32).write_unaligned(f1_bits);
        (this.wrapping_add(0x58) as *mut u16).write_unaligned(0);
        (this.wrapping_add(0x5A) as *mut u8).write(0);
        this
    }
});
