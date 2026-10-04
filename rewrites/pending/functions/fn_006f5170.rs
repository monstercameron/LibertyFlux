// original: 0x006f5170 init_with_critsecs
/// Initialise an object with two critical sections.
///
/// Installs the vtable, zeroes the header words, initialises the sections
/// at offsets `0x14` and `0x34`, clears the low two flag bits at `0x5C`,
/// and returns `this`.
rt::export!(thiscall, rw_006f5170(this: *mut u8) -> u32 {
    unsafe {
        *this.cast::<u32>() = rt::relocated(0x00FE5418);
        for off in [4usize, 8, 0xC, 0x10] {
            *this.add(off).cast::<u32>() = 0;
        }
        rt::callee_stdcall!(1, u32, this.add(0x14) as u32);
        rt::callee_stdcall!(1, u32, this.add(0x34) as u32);
        *this.add(0x54).cast::<u32>() = 0;
        *this.add(0x58).cast::<u32>() = 0;
        *this.add(0x5C) &= 0xFC;
        this as u32
    }
});
