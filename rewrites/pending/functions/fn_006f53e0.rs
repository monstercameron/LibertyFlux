// original: 0x006f53e0 init_record_args4
/// Initialise a small record with an extra word argument.
///
/// Like the three-argument variant but takes a fourth argument whose low
/// word is stored at `+0x2C`, and installs a different vtable. Returns
/// `this`.
rt::export!(thiscall, rw_006f53e0(this: *mut u8, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    unsafe {
        *this.add(8).cast::<u32>() = a4;
        *this.add(0x20).cast::<u32>() = a1;
        *this.add(0x24).cast::<u32>() = a2;
        *this.cast::<u32>() = rt::relocated(0x00FE4EB8);
        for off in [4usize, 0xC, 0x10, 0x14, 0x18] {
            *this.add(off).cast::<u32>() = 0;
        }
        *this.add(0x2C).cast::<u16>() = a3 as u16;
        *this.add(0x1C).cast::<u32>() = 0;
        *this.add(0x28).cast::<u32>() = this as u32;
        *this.cast::<u32>() = rt::relocated(0x00FE5424);
        this as u32
    }
});
