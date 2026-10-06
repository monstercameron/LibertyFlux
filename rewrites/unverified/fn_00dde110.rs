// original: 0x00DDE110 UITextField child selector
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Return the child object for `which`: the secondary child at `+0x1EC`
/// when `which` is exactly 1, otherwise the primary child at `+0x1E8`.
/// `which` is compared as an exact integer (zero, then one after decrement).
/// Original: thiscall, one stack word, returns the pointer in eax.
lf_checker_rt::export!(thiscall, rw_00DDE110(this: u32, which: u32) -> u32 {
    unsafe {
        const CHILD_PRIMARY: u32 = 0x1E8;
        const CHILD_SECONDARY: u32 = 0x1EC;
        let slot = if which == 1 { CHILD_SECONDARY } else { CHILD_PRIMARY };
        ((this + slot) as *const u32).read_unaligned()
    }
});
