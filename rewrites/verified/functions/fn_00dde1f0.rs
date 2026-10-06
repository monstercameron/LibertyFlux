// original: 0x00DDE1F0 UITextField grandchild field
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Return word `+0x1F4` of the primary child (`this + 0x1E8`).
/// Original: thiscall, no stack words, returns the word in eax.
lf_checker_rt::export!(thiscall, rw_00DDE1F0(this: u32) -> u32 {
    unsafe {
        const CHILD_PRIMARY: u32 = 0x1E8;
        const FIELD: u32 = 0x1F4;
        let child = ((this + CHILD_PRIMARY) as *const u32).read_unaligned();
        ((child + FIELD) as *const u32).read_unaligned()
    }
});
