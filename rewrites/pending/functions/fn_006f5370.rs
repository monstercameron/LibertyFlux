// original: 0x006f5370 init_record_args3
/// Initialise a small record: store three arguments, install the vtable,
/// clear the spare words, and link the record to itself at `+0x28`.
///
/// Returns `this`.
rt::export!(thiscall, rw_006f5370(this: *mut u8, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        *this.add(8).cast::<u32>() = a3;
        *this.add(0x20).cast::<u32>() = a1;
        *this.cast::<u32>() = rt::relocated(0x00FE4EB8);
        for off in [4usize, 0xC, 0x10, 0x14, 0x18] {
            *this.add(off).cast::<u32>() = 0;
        }
        *this.add(0x24).cast::<u32>() = a2;
        *this.add(0x1C).cast::<u32>() = 0;
        *this.add(0x28).cast::<u32>() = this as u32;
        *this.cast::<u32>() = rt::relocated(0x00FE5430);
        this as u32
    }
});
