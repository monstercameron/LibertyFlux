// original: 0x00DDE230 UITextField is text empty
/// UITextField helper (input-ui subsystem). `this` is the field object.
/// Return 1 when the field's text is empty, else 0: fetch the text pointer
/// from the text getter and scan for the NUL terminator. `this` is only
/// passed through to the getter. Original: thiscall, no stack words.
lf_checker_rt::export!(thiscall, rw_00DDE230(this: u32) -> u32 {
    unsafe {
        const TEXT_GETTER: u32 = 1;
        let s = lf_checker_rt::callee_thiscall!(TEXT_GETTER, u32, this);
        let mut p = s;
        while ((p) as *const u8).read() != 0 {
            p = p.wrapping_add(1);
        }
        ((p == s) as u32)
    }
});
