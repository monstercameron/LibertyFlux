// original: 0x00b007b0 init_param_triple
/// Initialise a 12-byte parameter block with its default triple.
export!(thiscall, rw_00b007b0(this: u32) -> u32 {
    unsafe {
        let p = this as *mut u32;
        *p = 0xefffffff;
        *p.add(1) = 0x3f800000;
        *p.add(2) = 0x3f800000;
        0
    }
});
